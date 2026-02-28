//! Basic types and interpreter smoke test.

use j_lang::run_source_to_string;

#[test]
fn run_integer_and_string() {
    let out = std::thread::Builder::new()
        .stack_size(4 * 1024 * 1024)
        .spawn(|| {
            run_source_to_string(r#"int: x = 42
str: s = "ok"
out(x)
out(s)"#)
        })
        .unwrap()
        .join()
        .unwrap()
        .unwrap();
    assert!(out.contains("42") && out.contains("ok"), "output: {}", out);
}

#[test]
fn run_simple_expr() {
    // Run in a thread with larger stack (interpreter can be recursion-heavy)
    let out = std::thread::Builder::new()
        .stack_size(4 * 1024 * 1024)
        .spawn(|| run_source_to_string("out(1 + 2)"))
        .unwrap()
        .join()
        .unwrap()
        .unwrap();
    assert_eq!(out.trim(), "3");
}
