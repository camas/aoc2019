use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

// Import generated get_questions()
include!(concat!(env!("OUT_DIR"), "/questions.rs"));

fn main() {
    let questions = get_questions();
    questions.get("1").unwrap()(read_input("1").unwrap());
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
