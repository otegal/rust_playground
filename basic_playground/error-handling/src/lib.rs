use std::fs;

pub fn main() {}

fn file_open(path: &str) -> &str {
    match fs::read_to_string(path) {
        Ok(s) => "ok",
        Err(err) => "error",
    }
}

#[test]
fn test_file_open() {
    assert_eq!(file_open("s.txt"), "error");
    assert_eq!(file_open("src/lib.rs"), "ok");
}
