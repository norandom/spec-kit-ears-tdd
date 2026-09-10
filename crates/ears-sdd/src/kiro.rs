//! Kiro / cc-sdd trees: `.kiro/specs/<feature>/requirements.md` plus optional phase metadata.
//!
//! Discovery lists feature directories. Evaluation parses criteria, computes the active baseline
//! from phase metadata, and checks that every superseded specification has a live successor.
//! Spec Kit identifier and EARS-form checks are applied elsewhere, and not to this family.

use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::path::{Path, PathBuf};

use crate::ccsdd::{ParseNote, Parser};
use crate::report::{relative, Finding, Severity};

pub const TREE: &str = ".kiro/specs";
pub const REQUIREMENTS_FILE: &str = "requirements.md";
pub const METADATA_FILE: &str = "spec.json";
pub const SCOPE_GLOB: &str = ".kiro/specs/*/requirements.md";

#[derive(Debug, Clone)]
pub struct Listed {
    pub feature: String,
    pub path: PathBuf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PhaseKind {
    Superseded,
    Reserved,
    Initialized,
    Other,
    Absent,
}

#[derive(Debug, Clone)]
pub struct EvaluatedFeature {
    pub feature: String,
    pub spec: PathBuf,
    pub requirements: usize,
    pub included: bool,
    pub exclusion_reason: Option<&'static str>,
}

#[derive(Debug, Clone)]
pub struct Evaluation {
    pub features: Vec<EvaluatedFeature>,
    pub findings: Vec<Finding>,
    pub included: usize,
    pub excluded: usize,
}

#[derive(Deserialize)]
struct SpecJson {
    #[serde(default)]
    phase: Option<String>,
    #[serde(default)]
    notes: Option<String>,
}

struct Loaded {
    feature: String,
    directory: PathBuf,
    spec_path: PathBuf,
    phase: PhaseKind,
    notes: String,
    metadata_error: bool,
    criteria_lines: HashSet<usize>,
    body: String,
}

pub fn tree_dir(root: &Path) -> PathBuf {
    root.join(".kiro").join("specs")
}

pub fn tree_present(root: &Path) -> bool {
    tree_dir(root).is_dir()
}

/// Child directories of `.kiro/specs`. `path` is `requirements.md` when that file exists,
/// otherwise the directory itself so a missing document is still a discovered specification.
pub fn list(root: &Path) -> Vec<Listed> {
    let dir = tree_dir(root);
    let Ok(entries) = std::fs::read_dir(&dir) else {
        return Vec::new();
    };
    let mut out: Vec<Listed> = entries
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .map(|directory| {
            let feature = basename(&directory);
            let requirements = directory.join(REQUIREMENTS_FILE);
            Listed {
                feature,
                path: if requirements.is_file() {
                    requirements
                } else {
                    directory
                },
            }
        })
        .collect();
    out.sort_by(|a, b| a.path.cmp(&b.path));
    out
}

pub fn evaluate(root: &Path, listed: &[Listed]) -> Evaluation {
    let names: Vec<String> = {
        let mut names: Vec<String> = listed.iter().map(|item| item.feature.clone()).collect();
        names.sort_by(|a, b| b.len().cmp(&a.len()).then(a.cmp(b)));
        names
    };

    let mut loaded: Vec<Loaded> = Vec::new();
    let mut features: Vec<EvaluatedFeature> = Vec::new();
    let mut findings: Vec<Finding> = Vec::new();
    let mut criteria_by_feature: BTreeMap<String, usize> = BTreeMap::new();

    for item in listed {
        let (spec, more_findings, count) = load_one(root, item);
        findings.extend(more_findings);
        criteria_by_feature.insert(item.feature.clone(), count);
        loaded.push(spec);
    }

    let membership: BTreeMap<String, (&'static str, bool)> = loaded
        .iter()
        .map(|spec| {
            let (reason, included) = membership(spec);
            (spec.feature.clone(), (reason, included))
        })
        .collect();

    let mut links: BTreeSet<(String, String)> = BTreeSet::new();

    for spec in &loaded {
        if spec.phase != PhaseKind::Superseded || spec.metadata_error {
            continue;
        }
        for successor in names_in(&spec.notes, &names) {
            if membership
                .get(&successor)
                .is_some_and(|(_, included)| *included)
            {
                links.insert((successor, spec.feature.clone()));
            }
        }
    }

    for spec in &loaded {
        let included = membership
            .get(&spec.feature)
            .is_some_and(|(_, included)| *included);
        if !included {
            continue;
        }
        for predecessor in claims_in(&spec.body, &spec.criteria_lines, &names) {
            if predecessor == spec.feature {
                continue;
            }
            if !membership.contains_key(&predecessor) {
                continue;
            }
            links.insert((spec.feature.clone(), predecessor.clone()));
            let predecessor_superseded = loaded
                .iter()
                .find(|other| other.feature == predecessor)
                .is_some_and(|other| other.phase == PhaseKind::Superseded && !other.metadata_error);
            if !predecessor_superseded {
                findings.push(
                    Finding::new(
                        "KIRO_SUPERSESSION_UNRECIPROCATED",
                        format!(
                            "`{}` claims to supersede `{}`, which is still in the active baseline.",
                            spec.feature, predecessor
                        ),
                        relative(&spec.spec_path, root),
                    )
                    .feature(spec.feature.clone())
                    .detail("predecessor", predecessor),
                );
            }
        }
    }

    for spec in &loaded {
        if spec.phase != PhaseKind::Superseded || spec.metadata_error {
            continue;
        }
        let has_live = links
            .iter()
            .any(|(_, predecessor)| predecessor == &spec.feature);
        if !has_live {
            findings.push(
                Finding::new(
                    "KIRO_SUPERSESSION_DANGLING",
                    format!(
                        "`{}` is superseded and names no successor in the active baseline.",
                        spec.feature
                    ),
                    relative(&spec.directory.join(METADATA_FILE), root),
                )
                .feature(spec.feature.clone()),
            );
        }
    }

    let mut included = 0usize;
    let mut excluded = 0usize;
    for spec in &loaded {
        let (reason, is_included) = membership[&spec.feature];
        if is_included {
            included += 1;
        } else {
            excluded += 1;
        }
        features.push(EvaluatedFeature {
            feature: spec.feature.clone(),
            spec: spec.spec_path.clone(),
            requirements: criteria_by_feature[&spec.feature],
            included: is_included,
            exclusion_reason: if is_included { None } else { Some(reason) },
        });
    }

    Evaluation {
        features,
        findings,
        included,
        excluded,
    }
}

fn load_one(root: &Path, item: &Listed) -> (Loaded, Vec<Finding>, usize) {
    let directory = if item.path.is_dir() {
        item.path.clone()
    } else {
        item.path
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| item.path.clone())
    };
    let requirements_path = directory.join(REQUIREMENTS_FILE);
    let metadata_path = directory.join(METADATA_FILE);
    let spec_path = if requirements_path.is_file() {
        requirements_path.clone()
    } else {
        directory.clone()
    };

    let mut findings = Vec::new();
    let mut metadata_error = false;
    let mut phase = PhaseKind::Absent;
    let mut notes = String::new();

    if metadata_path.is_file() {
        match std::fs::read_to_string(&metadata_path)
            .map_err(|error| error.to_string())
            .and_then(|text| {
                serde_json::from_str::<SpecJson>(&text).map_err(|error| error.to_string())
            }) {
            Ok(meta) => {
                phase = classify_phase(meta.phase.as_deref());
                notes = meta.notes.unwrap_or_default();
            }
            Err(_) => {
                metadata_error = true;
                findings.push(
                    Finding::new(
                        "KIRO_METADATA",
                        "Phase metadata could not be read.",
                        relative(&metadata_path, root),
                    )
                    .feature(item.feature.clone()),
                );
            }
        }
    }

    let mut body = String::new();
    let mut criteria_lines = HashSet::new();
    let mut count = 0usize;

    if requirements_path.is_file() {
        match std::fs::read_to_string(&requirements_path) {
            Ok(text) => {
                body = text;
                let parser = Parser::new(&item.feature, &requirements_path);
                let (criteria, notes_parse) = parser.parse(&body);
                count = criteria.len();
                for criterion in &criteria {
                    for line in criterion.line..=criterion.end_line {
                        criteria_lines.insert(line);
                    }
                }
                for note in notes_parse {
                    findings.push(note_to_finding(
                        note,
                        root,
                        &item.feature,
                        &requirements_path,
                    ));
                }
            }
            Err(error) => findings.push(
                Finding::new(
                    "SPEC_UNREADABLE",
                    error.to_string(),
                    relative(&requirements_path, root),
                )
                .feature(item.feature.clone()),
            ),
        }
    } else {
        findings.push(
            Finding::new(
                "KIRO_NO_REQUIREMENTS",
                "This specification has no requirements document.",
                relative(&directory, root),
            )
            .feature(item.feature.clone())
            .severity(Severity::Warning),
        );
    }

    (
        Loaded {
            feature: item.feature.clone(),
            directory,
            spec_path,
            phase,
            notes,
            metadata_error,
            criteria_lines,
            body,
        },
        findings,
        count,
    )
}

fn note_to_finding(note: ParseNote, root: &Path, feature: &str, path: &Path) -> Finding {
    let code = match note.code {
        "REQ_NO_CRITERIA" => "KIRO_NO_CRITERIA",
        other => other,
    };
    Finding::new(code, note.message, relative(path, root))
        .feature(feature)
        .line(note.line)
        .severity(Severity::Warning)
}

fn membership(spec: &Loaded) -> (&'static str, bool) {
    if spec.metadata_error {
        return ("unreadable-metadata", false);
    }
    match spec.phase {
        PhaseKind::Superseded => ("superseded", false),
        PhaseKind::Reserved => ("reserved", false),
        PhaseKind::Initialized => ("initialized", false),
        PhaseKind::Other | PhaseKind::Absent => ("", true),
    }
}

fn classify_phase(raw: Option<&str>) -> PhaseKind {
    match raw.map(str::trim).filter(|value| !value.is_empty()) {
        None => PhaseKind::Absent,
        Some(value) => match value.to_ascii_lowercase().as_str() {
            "superseded" => PhaseKind::Superseded,
            "reserved" => PhaseKind::Reserved,
            "initialized" => PhaseKind::Initialized,
            _ => PhaseKind::Other,
        },
    }
}

fn names_in(text: &str, names: &[String]) -> Vec<String> {
    names
        .iter()
        .filter(|name| contains_name(text, name))
        .cloned()
        .collect()
}

fn claims_in(body: &str, criteria_lines: &HashSet<usize>, names: &[String]) -> Vec<String> {
    let mut claimed = BTreeSet::new();
    let mut fence = Fence::default();
    for (index, line) in body.lines().enumerate() {
        let number = index + 1;
        if fence.consume(line) || fence.inside() {
            continue;
        }
        if criteria_lines.contains(&number) {
            continue;
        }
        if !has_supersede_lemma(line) {
            continue;
        }
        for name in names_in(line, names) {
            claimed.insert(name);
        }
    }
    claimed.into_iter().collect()
}

fn has_supersede_lemma(line: &str) -> bool {
    let lower = line.to_ascii_lowercase();
    lower.contains("supersede")
}

fn contains_name(text: &str, name: &str) -> bool {
    let mut from = 0;
    while let Some(offset) = text[from..].find(name) {
        let at = from + offset;
        let before_ok = at == 0
            || text[..at]
                .chars()
                .last()
                .is_some_and(|ch| !is_name_char(ch));
        let after = at + name.len();
        let after_ok = after >= text.len()
            || text[after..]
                .chars()
                .next()
                .is_some_and(|ch| !is_name_char(ch));
        if before_ok && after_ok {
            return true;
        }
        from = at + 1;
        if from >= text.len() {
            break;
        }
    }
    false
}

fn is_name_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '-' || ch == '_'
}

fn basename(path: &Path) -> String {
    path.file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| "<root>".to_string())
}

#[derive(Default)]
struct Fence {
    marker: Option<(char, usize)>,
}

impl Fence {
    fn consume(&mut self, line: &str) -> bool {
        let trimmed = line.trim_start();
        let ch = trimmed.chars().next().filter(|c| *c == '`' || *c == '~');
        let Some(ch) = ch else {
            return self.marker.is_some();
        };
        let run = trimmed.chars().take_while(|c| *c == ch).count();
        if run < 3 {
            return self.marker.is_some();
        }
        match self.marker {
            None => {
                self.marker = Some((ch, run));
                true
            }
            Some((open_ch, open_run)) if open_ch == ch && run >= open_run => {
                self.marker = None;
                true
            }
            Some(_) => true,
        }
    }

    fn inside(&self) -> bool {
        self.marker.is_some()
    }
}
