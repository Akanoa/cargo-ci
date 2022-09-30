use cargo_ci::{run, to_value};
use std::fs::remove_file;
use std::path::Path;

#[test]
fn transform() {
    let expected = to_value("tests/assets/project1/Cargo.expected.toml").ok();
    let dest = "tests/assets/project1/Cargo.test.toml";

    run(
        "token123".to_string(),
        "tests/assets/project1/Cargo.toml".to_string(),
        dest.to_string(),
        vec!["toto".to_string()],
        vec!["titi".to_string()],
    )
    .expect("Unable to run conversion");

    let result_data = to_value("tests/assets/project1/Cargo.test.toml").ok();

    if Path::new(dest).exists() {
        remove_file("tests/assets/project1/Cargo.test.toml").unwrap();
    }

    assert_eq!(result_data, expected)
}
