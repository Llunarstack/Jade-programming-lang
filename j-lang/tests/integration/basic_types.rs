use std::process::Command;

#[test]
fn test_integer_operations() {
    let output = Command::new("cargo")
        .args(&["run", "--", "run", "tests/fixtures/basic_types.j"])
        .output()
        .expect("Failed to execute");

    assert!(output.status.success());
}

#[test]
fn test_string_operations() {
    let output = Command::new("cargo")
        .args(&["run", "--", "run", "tests/fixtures/strings.j"])
        .output()
        .expect("Failed to execute");

    assert!(output.status.success());
}
