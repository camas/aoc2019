mod common;

#[macro_use]
extern crate impl_ops;

use crate::common::read_file;
use clap::{App, AppSettings, Arg};
use std::collections::HashMap;
use std::path::Path;

// Import generated get_questions()
include!(concat!(env!("OUT_DIR"), "/questions.rs"));

fn main() {
    // Parse args
    let matches = App::new("solve")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::with_name("all").long("all"))
        .arg(Arg::with_name("questions").multiple(true))
        .get_matches();

    // Run all solutions
    if matches.is_present("all") {
        run_all();
        return;
    }

    // Run specified solutions
    let qs = matches.values_of_lossy("questions");
    if let Some(x) = qs {
        run_solutions(x);
    }
}

fn run_all() {
    // Get available solutions by checking for input.txt
    let mut available = Vec::new();
    for i in 1..=25 {
        let input_str = format!("{}/input.txt", i);
        let input_path = Path::new(&input_str);
        if input_path.exists() {
            available.push(i);
        }
    }
    run_solutions(available.iter().map(|x| x.to_string()).collect());
}

fn run_solutions(qs: Vec<String>) {
    println!("Running {} solutions", qs.len());
    let questions = get_questions();
    for q in &qs {
        // Load input.txt
        let input_path = format!("{}/input.txt", q);
        let input = read_file(&input_path);
        // Convert to ref. Lets tests use static strings as input
        let input_ref = input.iter().map(AsRef::as_ref).collect();
        // Run solution
        let solution = questions.get(q).unwrap()(input_ref);
        println!("Solution to {} is {}", q, solution);
        // Copy to clipboard
        if qs.len() == 1 {
            set_clipboard(&solution);
            println!("Copied to clipboard");
        }
    }
}

#[cfg(target_os = "linux")]
// Copies to clipboard using xclip
fn set_clipboard(text: &str) {
    use std::io::{BufWriter, Write};
    use std::process::{Command, Stdio};

    let xclip = Command::new("xclip")
        .arg("-selection")
        .arg("clipboard")
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();
    let xclip_in = xclip.stdin.unwrap();
    let mut w = BufWriter::new(xclip_in);
    w.write_all(text.as_bytes()).unwrap();
}

#[cfg(not(target_os = "linux"))]
// Copies to clipboard using clipboard library
fn set_clipboard(text: &str) {
    use clipboard::{ClipboardContext, ClipboardProvider};
    let mut clip: ClipboardContext = ClipboardProvider::new().unwrap();
    clip.set_contents(solution).unwrap();
}
