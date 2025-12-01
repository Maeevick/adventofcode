use std::fs;
use std::path::PathBuf;

pub fn read_input(filename: &str) -> Vec<String> {
    let path = PathBuf::from("..")
        .join("inputs")
        .join(format!("{}.txt", filename));

    let content = fs::read_to_string(path).expect("Should have been able to read the file");

    content.lines().map(String::from).collect()
}
