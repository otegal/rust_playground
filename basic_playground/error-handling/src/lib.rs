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

fn file_open_return_result_type(path: &str) -> std::io::Result<String> {
    fs::read_to_string(path)
}

#[test]
fn test_file_open_return_result_type() {
    // failed
    let not_exist_file_path = "s.txt";

    let fail_result = file_open_return_result_type(not_exist_file_path);
    assert_eq!(fail_result.is_err(), true);
    assert_eq!(fail_result.is_ok(), false);

    let unwrap = match file_open_return_result_type(not_exist_file_path) {
        Ok(_s) => "ok",
        Err(_e) => "err",
    };
    assert_eq!(unwrap, "err");

    let method_chain_unwrap = file_open_return_result_type(not_exist_file_path)
        .map(|_s| 0)
        .unwrap_or_else(|_err| 1);
    assert_eq!(method_chain_unwrap, 1);

    // success
    let exist_file_path = "src/lib.rs";

    let fail_result = file_open_return_result_type(exist_file_path);
    assert_eq!(fail_result.is_err(), false);
    assert_eq!(fail_result.is_ok(), true);

    let unwrap = match file_open_return_result_type(exist_file_path) {
        Ok(_s) => "ok",
        Err(_e) => "err",
    };
    assert_eq!(unwrap, "ok");

    let method_chain_unwrap = file_open_return_result_type(exist_file_path)
        .map(|_s| 0)
        .unwrap_or_else(|_err| 1);
    assert_eq!(method_chain_unwrap, 0);
}
