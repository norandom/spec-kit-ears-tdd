//! A SKOS bridge for the vocabulary.
//!
//! The vocabulary is authored in TOML and stays that way. This module exists so it does not have to
//! be a closed world: a project can seed itself from a vocabulary someone has already agreed, and
//! can publish its own for tools that speak RDF.
//!
//! **Why not author in SKOS.** Turtle reviews badly in a pull request, and the gate needs no triple
//! store. More importantly SKOS has no place for the one field the constraint model depends on: a
//! term's domain. SKOS describes concepts, not the values a variable ranges over, and expressing
//! `enum` or `int` bounds means SHACL or a private extension, at which point the format is no longer
//! standard anyway. So the mapping is deliberately lossy in one direction and additive in the other.
//!
//! **What that means in practice.** An imported vocabulary that carries no domain information gets
//! `entity`, which is the honest reading: a published thesaurus says what concepts exist, not which
//! of them are booleans. Adding domains is the work of turning someone else's vocabulary into one
//! this project can build guards from.

use std::collections::{BTreeMap, BTreeSet};

use oxrdf::{Literal, NamedNode, NamedOrBlankNode, Term as RdfTerm};
use oxttl::TurtleParser;

use crate::vocabulary::{Domain, Term};

pub const SKOS: &str = "http://www.w3.org/2004/02/skos/core#";
pub const OWL: &str = "http://www.w3.org/2002/07/owl#";
pub const DCTERMS: &str = "http://purl.org/dc/terms/";
pub const XSD: &str = "http://www.w3.org/2001/XMLSchema#";

/// The private extension carrying what SKOS has no term for.
///
/// Namespaced rather than invented bare, so a consumer that does not understand it ignores it
/// instead of misreading it, and a consumer that does can round-trip without loss.
pub const EARS: &str = "https://norandom.github.io/spec-kit-ears-tdd/ns#";

/// Render the vocabulary as a SKOS concept scheme in Turtle.
pub fn export(terms: &BTreeMap<String, (Term, String)>, base: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "@prefix skos: <{SKOS}> .\n\
         @prefix owl:  <{OWL}> .\n\
         @prefix dct:  <{DCTERMS}> .\n\
         @prefix xsd:  <{XSD}> .\n\
         @prefix ears: <{EARS}> .\n\
         @base <{base}> .\n\n"
    ));
    out.push_str("<> a skos:ConceptScheme .\n");

    for (identifier, (term, _)) in terms {
        out.push_str(&format!("\n<#{identifier}> a skos:Concept ;\n"));
        out.push_str("    skos:inScheme <> ;\n");
        // A term nothing stands above is a top concept. Stating it is what lets a browser show the
        // vocabulary as a tree rather than a flat list.
        if term.broader.is_empty() {
            out.push_str("    skos:topConceptOf <> ;\n");
        }
        out.push_str(&format!("    skos:prefLabel {} ;\n", literal(&term.label)));
        if !term.definition.trim().is_empty() {
            out.push_str(&format!(
                "    skos:definition {} ;\n",
                literal(&term.definition)
            ));
        }
        for alternative in &term.alt_labels {
            out.push_str(&format!("    skos:altLabel {} ;\n", literal(alternative)));
        }
        for broader in &term.broader {
            out.push_str(&format!("    skos:broader <#{broader}> ;\n"));
        }
        if term.deprecated {
            out.push_str("    owl:deprecated true ;\n");
        }
        if let Some(replacement) = &term.replaced_by {
            out.push_str(&format!("    dct:isReplacedBy <#{replacement}> ;\n"));
        }
        out.push_str(&describe_domain(&term.domain));
        // Replace the trailing predicate separator with a statement terminator.
        while out.ends_with(" ;\n") {
            out.truncate(out.len() - 3);
            out.push_str(" .\n");
        }
    }
    out
}

fn describe_domain(domain: &Domain) -> String {
    match domain {
        Domain::Entity => "    ears:domain \"entity\" ;\n".to_string(),
        Domain::Bool => "    ears:domain \"bool\" ;\n".to_string(),
        Domain::Enum { values } => {
            let mut out = String::from("    ears:domain \"enum\" ;\n");
            for value in values {
                out.push_str(&format!("    ears:value {} ;\n", literal(value)));
            }
            out
        }
        Domain::Int { min, max } => format!(
            "    ears:domain \"int\" ;\n    \
             ears:minimum \"{min}\"^^xsd:integer ;\n    \
             ears:maximum \"{max}\"^^xsd:integer ;\n"
        ),
    }
}

/// A Turtle literal, escaped by the RDF library rather than by hand.
fn literal(value: &str) -> String {
    Literal::new_simple_literal(value).to_string()
}

/// Read a SKOS concept scheme and render it as a vocabulary file.
///
/// Returns the TOML text, or the parse errors that stopped it.
pub fn import(turtle: &[u8]) -> Result<String, Vec<String>> {
    let mut collected: BTreeMap<String, Collected> = BTreeMap::new();
    let mut concepts: BTreeSet<String> = BTreeSet::new();
    let mut errors = Vec::new();

    for triple in TurtleParser::new().for_reader(turtle) {
        let triple = match triple {
            Ok(triple) => triple,
            Err(error) => {
                errors.push(error.to_string());
                continue;
            }
        };
        let NamedOrBlankNode::NamedNode(subject) = &triple.subject else {
            // Blank nodes carry no identifier a vocabulary can use as a join key, which is the one
            // thing a term must have.
            continue;
        };
        let Some(identifier) = local_name(subject) else {
            continue;
        };
        let predicate = triple.predicate.as_str().to_string();

        if predicate == format!("{}type", RDF_NS) {
            if let RdfTerm::NamedNode(object) = &triple.object {
                if object.as_str() == format!("{SKOS}Concept") {
                    concepts.insert(identifier.clone());
                }
            }
            continue;
        }

        let entry = collected.entry(identifier).or_default();
        let text = literal_text(&triple.object);
        let target = named_local(&triple.object);

        match predicate.as_str() {
            p if p == format!("{SKOS}prefLabel") => entry.label = text,
            p if p == format!("{SKOS}definition") => entry.definition = text,
            p if p == format!("{SKOS}altLabel") => {
                if let Some(value) = text {
                    entry.alt_labels.insert(value);
                }
            }
            p if p == format!("{SKOS}broader") => {
                if let Some(value) = target {
                    entry.broader.insert(value);
                }
            }
            p if p == format!("{SKOS}narrower") => {
                // Recorded from the other side. SKOS treats broader and narrower as inverses, and a
                // published vocabulary may assert either or both.
                if let Some(value) = target {
                    entry.narrower_of.insert(value);
                }
            }
            p if p == format!("{OWL}deprecated") => {
                entry.deprecated = matches!(text.as_deref(), Some("true"));
            }
            p if p == format!("{DCTERMS}isReplacedBy") => entry.replaced_by = target,
            p if p == format!("{EARS}domain") => entry.domain_kind = text,
            p if p == format!("{EARS}value") => {
                if let Some(value) = text {
                    entry.values.push(value);
                }
            }
            p if p == format!("{EARS}minimum") => entry.minimum = text.and_then(|t| t.parse().ok()),
            p if p == format!("{EARS}maximum") => entry.maximum = text.and_then(|t| t.parse().ok()),
            _ => {}
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    // Narrower assertions become broader ones on the other concept, so the output states the
    // relation the one way this project reads it.
    let inverses: Vec<(String, String)> = collected
        .iter()
        .flat_map(|(identifier, entry)| {
            entry
                .narrower_of
                .iter()
                .map(move |child| (child.clone(), identifier.clone()))
        })
        .collect();
    for (child, parent) in inverses {
        collected.entry(child).or_default().broader.insert(parent);
    }

    Ok(render(&collected, &concepts))
}

const RDF_NS: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";

#[derive(Default)]
struct Collected {
    label: Option<String>,
    definition: Option<String>,
    alt_labels: BTreeSet<String>,
    broader: BTreeSet<String>,
    narrower_of: BTreeSet<String>,
    deprecated: bool,
    replaced_by: Option<String>,
    domain_kind: Option<String>,
    values: Vec<String>,
    minimum: Option<i64>,
    maximum: Option<i64>,
}

fn render(collected: &BTreeMap<String, Collected>, concepts: &BTreeSet<String>) -> String {
    let mut out = String::from(
        "# Imported from a SKOS concept scheme.\n\
         #\n\
         # Definitions carried across where the source had them. Where it did not, the definition is\n\
         # empty and the gate fails until someone writes one, which is the same rule that applies to\n\
         # a scaffolded vocabulary: an imported term nobody has read grounds nothing.\n\
         #\n\
         # Domains default to `entity` unless the source carried this project's extension. A\n\
         # published vocabulary says which concepts exist, not which of them are booleans, so\n\
         # turning imported terms into ones a guard can range over is work this cannot do for you.\n\n\
         schema_version = \"1.0\"\n",
    );

    let mut written = 0usize;
    for (identifier, entry) in collected {
        // Only things the source called a concept. A scheme, a collection, and the dozens of
        // annotation subjects a real vocabulary carries are not terms.
        if !concepts.contains(identifier) {
            continue;
        }
        written += 1;
        out.push_str(&format!("\n[terms.{identifier}]\n"));
        out.push_str(&format!(
            "label = {}\n",
            toml_string(entry.label.as_deref().unwrap_or(identifier))
        ));
        out.push_str(&format!(
            "definition = {}\n",
            toml_string(entry.definition.as_deref().unwrap_or(""))
        ));
        out.push_str(&format!("domain = {}\n", domain_of(entry)));
        if !entry.broader.is_empty() {
            let items: Vec<String> = entry.broader.iter().map(|b| format!("\"{b}\"")).collect();
            out.push_str(&format!("broader = [{}]\n", items.join(", ")));
        }
        if !entry.alt_labels.is_empty() {
            let items: Vec<String> = entry.alt_labels.iter().map(|a| toml_string(a)).collect();
            out.push_str(&format!("alt_labels = [{}]\n", items.join(", ")));
        }
        if entry.deprecated {
            out.push_str("deprecated = true\n");
        }
        if let Some(replacement) = &entry.replaced_by {
            out.push_str(&format!("replaced_by = \"{replacement}\"\n"));
        }
    }

    if written == 0 {
        out.push_str(
            "\n# No skos:Concept found. The file may describe a collection or a scheme without\n\
             # concepts, or use a vocabulary other than SKOS.\n",
        );
    }
    out
}

fn domain_of(entry: &Collected) -> String {
    match entry.domain_kind.as_deref() {
        Some("bool") => "{ kind = \"bool\" }".to_string(),
        Some("enum") => {
            let items: Vec<String> = entry.values.iter().map(|v| toml_string(v)).collect();
            format!("{{ kind = \"enum\", values = [{}] }}", items.join(", "))
        }
        Some("int") => format!(
            "{{ kind = \"int\", min = {}, max = {} }}",
            entry.minimum.unwrap_or(0),
            entry.maximum.unwrap_or(0)
        ),
        _ => "{ kind = \"entity\" }".to_string(),
    }
}

/// A TOML basic string. Multi-line definitions become literal blocks rather than one long line with
/// escaped newlines, because a definition is prose someone has to read in a diff.
fn toml_string(value: &str) -> String {
    if value.contains('\n') {
        format!("\"\"\"\n{}\n\"\"\"", value.trim_end())
    } else {
        format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""))
    }
}

fn literal_text(term: &RdfTerm) -> Option<String> {
    match term {
        RdfTerm::Literal(value) => Some(value.value().to_string()),
        RdfTerm::NamedNode(node) => local_name(node),
        _ => None,
    }
}

fn named_local(term: &RdfTerm) -> Option<String> {
    match term {
        RdfTerm::NamedNode(node) => local_name(node),
        _ => None,
    }
}

/// The identifier at the end of an IRI.
///
/// Vocabularies split their namespace with either a hash or a slash, and a term's identity here is
/// the local name rather than the full IRI: the vocabulary is a join key inside one project, not a
/// claim about global identity.
fn local_name(node: &NamedNode) -> Option<String> {
    let iri = node.as_str();
    let tail = iri
        .rsplit_once('#')
        .map(|(_, tail)| tail)
        .or_else(|| iri.rsplit_once('/').map(|(_, tail)| tail))?;
    // Published vocabularies overwhelmingly write local names in CamelCase, and this project reads
    // kebab-case. Collapsing `InsiderThreat` to `insiderthreat` is technically a valid identifier
    // and reads as a typo, which is enough to make an imported vocabulary feel foreign in every
    // requirement that tags it.
    let characters: Vec<char> = tail.chars().collect();
    let mut slug = String::new();
    for (index, character) in characters.iter().enumerate() {
        if !character.is_alphanumeric() {
            slug.push('-');
            continue;
        }
        // A boundary is an uppercase letter that follows a lowercase one, or that begins a word
        // after an acronym. The second case is what keeps `HTTPSProxy` from becoming `h-t-t-p-s`.
        let previous = index.checked_sub(1).and_then(|i| characters.get(i));
        let next = characters.get(index + 1);
        let starts_word = character.is_uppercase()
            && match (previous, next) {
                (Some(before), _) if before.is_lowercase() || before.is_numeric() => true,
                (Some(before), Some(after)) => before.is_uppercase() && after.is_lowercase(),
                _ => false,
            };
        if starts_word && !slug.is_empty() {
            slug.push('-');
        }
        slug.extend(character.to_lowercase());
    }
    let slug = slug
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-");
    (!slug.is_empty()).then_some(slug)
}

/// The identifier at the end of an IRI.
///
/// Vocabularies split their namespace with either a hash or a slash, and a term's identity here is
/// the local name rather than the full IRI: the vocabulary is a join key inside one project, not a
/// claim about global identity.
#[cfg(test)]
mod tests {
    use super::*;

    fn term(label: &str, definition: &str, domain: Domain) -> (Term, String) {
        (
            Term {
                label: label.to_string(),
                definition: definition.to_string(),
                domain,
                broader: Vec::new(),
                alt_labels: Vec::new(),
                deprecated: false,
                replaced_by: None,
            },
            ".specify/vocabulary.toml".to_string(),
        )
    }

    /// The fields SKOS has no term for are the ones the constraint model needs, so a round trip that
    /// loses them would leave an imported vocabulary unable to express a single guard.
    #[test]
    fn a_round_trip_keeps_the_domains_skos_cannot_express() {
        let mut terms = BTreeMap::new();
        terms.insert(
            "mode".to_string(),
            term(
                "Operating mode",
                "Which mode the service runs in.",
                Domain::Enum {
                    values: vec!["normal".into(), "degraded".into()],
                },
            ),
        );
        terms.insert(
            "depth".to_string(),
            term(
                "Queue depth",
                "Pending operations.",
                Domain::Int { min: 0, max: 500 },
            ),
        );
        terms.insert(
            "verified".to_string(),
            term("Verified", "Whether the digest matched.", Domain::Bool),
        );

        let turtle = export(&terms, "https://example.org/v");
        let toml = import(turtle.as_bytes()).expect("the exported turtle parses");

        assert!(
            toml.contains(r#"domain = { kind = "enum", values = ["normal", "degraded"] }"#),
            "{toml}"
        );
        assert!(
            toml.contains(r#"domain = { kind = "int", min = 0, max = 500 }"#),
            "{toml}"
        );
        assert!(toml.contains(r#"domain = { kind = "bool" }"#), "{toml}");
    }

    /// The greenfield case. A published thesaurus says which concepts exist and nothing about which
    /// of them are booleans, so every term arrives as an entity and the definitions arrive empty
    /// where the source had none. Both are honest defaults rather than guesses.
    #[test]
    fn a_foreign_vocabulary_imports_without_this_project_extension() {
        let turtle = br#"
            @prefix skos: <http://www.w3.org/2004/02/skos/core#> .
            <https://id.example.org/scheme> a skos:ConceptScheme .
            <https://id.example.org/scheme#Malware> a skos:Concept ;
                skos:prefLabel "Malware"@en ;
                skos:definition "Software intended to cause harm."@en ;
                skos:altLabel "malicious software"@en .
            <https://id.example.org/scheme#Ransomware> a skos:Concept ;
                skos:prefLabel "Ransomware"@en ;
                skos:broader <https://id.example.org/scheme#Malware> .
        "#;

        let toml = import(turtle).expect("standard SKOS parses");

        assert!(toml.contains("[terms.malware]"), "{toml}");
        assert!(toml.contains("[terms.ransomware]"), "{toml}");
        assert!(toml.contains(r#"broader = ["malware"]"#), "{toml}");
        assert!(
            toml.contains(r#"alt_labels = ["malicious software"]"#),
            "{toml}"
        );
        // Nothing said what kind of thing these are.
        assert!(toml.contains(r#"domain = { kind = "entity" }"#), "{toml}");
        // Ransomware carried no definition, and an empty one fails the gate until someone writes it.
        let block = toml.split("[terms.ransomware]").nth(1).expect("present");
        assert!(block.contains(r#"definition = """#), "{block}");
        // The scheme is not a concept and must not become a term.
        assert!(!toml.contains("[terms.scheme]"), "{toml}");
    }

    /// SKOS treats broader and narrower as inverses and a real vocabulary may assert either. The
    /// hierarchy has to come out the same whichever direction the source chose.
    #[test]
    fn narrower_is_read_as_broader_on_the_other_concept() {
        let turtle = br#"
            @prefix skos: <http://www.w3.org/2004/02/skos/core#> .
            <https://id.example.org/s#Parent> a skos:Concept ;
                skos:prefLabel "Parent" ;
                skos:narrower <https://id.example.org/s#Child> .
            <https://id.example.org/s#Child> a skos:Concept ;
                skos:prefLabel "Child" .
        "#;

        let toml = import(turtle).expect("parses");
        let child = toml.split("[terms.child]").nth(1).expect("child present");
        assert!(child.contains(r#"broader = ["parent"]"#), "{child}");
    }

    /// Published vocabularies write local names in CamelCase and this project reads kebab-case.
    /// The acronym cases are the ones a naive split gets wrong in opposite directions.
    #[test]
    fn camel_case_identifiers_become_kebab_case() {
        let cases = [
            ("https://e.org/s#InsiderThreat", "insider-threat"),
            ("https://e.org/s#Mitigation", "mitigation"),
            ("https://e.org/s#HTTPSProxy", "https-proxy"),
            ("https://e.org/s#TLS", "tls"),
            ("https://e.org/s#already-kebab", "already-kebab"),
            ("https://e.org/s/trailing/slash", "slash"),
            ("https://e.org/s#Level2Cache", "level2-cache"),
        ];
        for (iri, expected) in cases {
            let node = NamedNode::new(iri).expect("a valid IRI");
            assert_eq!(local_name(&node).as_deref(), Some(expected), "for {iri}");
        }
    }

    #[test]
    fn a_malformed_document_reports_rather_than_returning_half_a_vocabulary() {
        let broken = b"@prefix skos: <http://www.w3.org/2004/02/skos/core#>\n<#a> a skos:Concept .";
        assert!(import(broken).is_err());
    }
}
