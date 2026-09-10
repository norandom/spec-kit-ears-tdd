//! Scratch harness: run the cc-sdd parser over every spec in a `.kiro/specs` tree and report
//! extraction coverage and structural notes. This is a development aid for validating the parser
//! against real cc-sdd projects; it is not part of the shipped CLI.
//!
//! Usage: cargo run --example ccsdd-scan -- /path/to/project/.kiro/specs

use ears_sdd::ccsdd::Parser;
use std::path::PathBuf;

fn main() {
    let root = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(".kiro/specs"));
    if !root.is_dir() {
        eprintln!("not a directory: {}", root.display());
        std::process::exit(2);
    }

    let mut specs: Vec<PathBuf> = std::fs::read_dir(&root)
        .expect("read specs dir")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.is_dir())
        .collect();
    specs.sort();

    let mut total = 0usize;
    let mut grand_notes = 0usize;
    for spec in &specs {
        let feature = spec.file_name().unwrap().to_string_lossy().to_string();
        let req_file = spec.join("requirements.md");
        if !req_file.is_file() {
            println!("{feature:<46} (no requirements.md)");
            continue;
        }
        let text = std::fs::read_to_string(&req_file).expect("read requirements.md");
        let parser = Parser::new(&feature, &req_file);
        let (reqs, notes) = parser.parse(&text);
        total += reqs.len();
        grand_notes += notes.len();

        let baseline = reqs
            .iter()
            .filter(|r| r.kind == ears_sdd::ccsdd::ReqKind::Baseline)
            .count();
        let amend = reqs.len() - baseline;
        println!(
            "{feature:<46} {:>3} criteria ({baseline} baseline, {amend} amendment){}",
            reqs.len(),
            if notes.is_empty() {
                String::new()
            } else {
                format!("  — {} notes", notes.len())
            }
        );
        for n in &notes {
            println!("    note [{}] line {}: {}", n.code, n.line, n.message);
        }
        // Spot-check: print the first and last criterion id to verify ordering and id shape.
        if let (Some(first), Some(last)) = (reqs.first(), reqs.last()) {
            println!("    ids: {} … {}", first.id, last.id);
        }
        // Detail dump for amendments and any spec named by VERBOSE env.
        let verbose = std::env::var("VERBOSE").ok();
        if verbose.as_deref() == Some(feature.as_str()) {
            for r in &reqs {
                let ext = if r.extends.is_empty() {
                    String::new()
                } else {
                    format!("  extends=[{}]", r.extends.join(","))
                };
                println!(
                    "    {:>6} [{:?}] L{}-{} {}{}\n           {}",
                    r.id,
                    r.kind,
                    r.line,
                    r.end_line,
                    r.title,
                    ext,
                    &r.ears_text.chars().take(90).collect::<String>()
                );
            }
        }
    }
    println!(
        "\nTOTAL: {total} criteria across {} specs, {grand_notes} notes",
        specs.len()
    );
}
