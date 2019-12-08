mod common;

use clap::{App, AppSettings, Arg};
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

// Import generated get_questions()
include!(concat!(env!("OUT_DIR"), "/questions.rs"));

fn main() {
    // Parse args
    let matches = App::new("solve")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::with_name("questions").multiple(true))
        .get_matches();

    let qs = matches.values_of_lossy("questions");
    match qs {
        Some(x) => run_solutions(x),
        None => (),
    }
}

fn run_solutions(qs: Vec<String>) {
    println!("Running {} solutions", qs.len());
    let questions = get_questions();
    for q in &qs {
        let solution = questions.get(q).unwrap()(read_input(q).unwrap());
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

// Reads the input.txt for a given question
fn read_input(question: &str) -> io::Result<Vec<String>> {
    let input_str = format!("{}/input.txt", question);
    let input_path = Path::new(&input_str);
    let reader = BufReader::new(File::open(input_path)?);
    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }
    return Ok(lines);
}
