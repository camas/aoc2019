use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub mod machine;

// https://stackoverflow.com/a/41536521
pub fn get_digits(n: i64) -> Vec<i64> {
    fn x_inner(n: i64, xs: &mut Vec<i64>) {
        if n >= 10 {
            x_inner(n / 10, xs);
        }
        xs.push(n % 10);
    }
    let mut xs = Vec::new();
    x_inner(n, &mut xs);
    xs
}

pub fn read_file(path: &str) -> Vec<String> {
    let input_path = Path::new(path);
    let reader = BufReader::new(File::open(input_path).unwrap());
    let mut lines: Vec<_> = Vec::new();
    for line in reader.lines() {
        lines.push(line.unwrap());
    }
    lines
}
