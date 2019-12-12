use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest = Path::new(&out_dir).join("questions.rs");
    let mut f = File::create(&dest).unwrap();

    // Find solution files
    let mut sols = Vec::new();
    for i in 1..26 {
        let name = format!("{}/solution.rs", i);
        if Path::new(&name).exists() {
            sols.push(i);
        }
    }

    // Write includes
    for sol in &sols {
        let line = format!(
            "include!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/{}/solution.rs\"));",
            sol
        );
        f.write_all(line.as_bytes()).unwrap();
    }
    f.write(b"\n").unwrap();

    // Write get_questions()
    f.write_all(b"pub fn get_questions() -> HashMap<String, fn(Vec<&str>) -> String> {\n")
        .unwrap();
    f.write_all(
        b"    let mut questions: HashMap<String, fn(Vec<&str>) -> String> = HashMap::new();\n",
    )
    .unwrap();
    for sol in &sols {
        let line = format!(
            "    questions.insert(\"{}\".to_string(), question{}::solve);\n",
            sol, sol
        );
        f.write_all(line.as_bytes()).unwrap();
    }
    f.write_all(b"    questions\n}\n").unwrap();
}
