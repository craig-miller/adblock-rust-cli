//! adblock-rust-cli — convert EasyList/ABP filter rules to Apple WKContentRuleList JSON.
//!
//! Reads a filter list from argv[1] (path) or stdin if no argument is given.
//! Writes a JSON array of content-blocker rules to stdout. Progress stats
//! (rule count, unrepresentable-rule counts) go to stderr.
//!
//! Exit codes:
//!   0  success
//!   2  input read failure
//!   3  JSON serialization failure

use adblock::content_blocking::{CbRule, CbRuleEquivalent};
use adblock::lists::{parse_filters, ParseOptions};
use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    let input = match args.get(1).map(String::as_str) {
        Some("-") | None => {
            let mut buf = String::new();
            if let Err(e) = io::stdin().read_to_string(&mut buf) {
                eprintln!("adblock-rust-cli: reading stdin: {e}");
                return ExitCode::from(2);
            }
            buf
        }
        Some(path) => match fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("adblock-rust-cli: reading {path}: {e}");
                return ExitCode::from(2);
            }
        },
    };

    // debug: true is required — CbRule::TryFrom returns NeedsDebugMode
    // when raw_line is None, which is the case unless debug is set.
    let (network_filters, cosmetic_filters) =
        parse_filters(input.lines(), true, ParseOptions::default());

    let mut rules: Vec<CbRule> = Vec::new();
    let mut net_dropped = 0usize;
    let mut cos_dropped = 0usize;

    for nf in network_filters {
        match CbRuleEquivalent::try_from(nf) {
            Ok(cbre) => rules.extend(cbre),
            Err(_) => net_dropped += 1,
        }
    }

    for cf in cosmetic_filters {
        match CbRule::try_from(cf) {
            Ok(r) => rules.push(r),
            Err(_) => cos_dropped += 1,
        }
    }

    eprintln!(
        "adblock-rust-cli: emitted {} rules ({} network + {} cosmetic unrepresentable)",
        rules.len(),
        net_dropped,
        cos_dropped
    );

    let stdout = io::stdout();
    let mut out = stdout.lock();
    if let Err(e) = serde_json::to_writer(&mut out, &rules) {
        eprintln!("adblock-rust-cli: writing JSON: {e}");
        return ExitCode::from(3);
    }
    let _ = writeln!(out);
    ExitCode::SUCCESS
}
