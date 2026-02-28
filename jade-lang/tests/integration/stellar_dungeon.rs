//! Stellar dungeon / game logic smoke test.

use j_lang::run_source_to_string;

#[test]
fn run_minimal_script() {
    let out = std::thread::Builder::new()
        .stack_size(4 * 1024 * 1024)
        .spawn(|| {
            run_source_to_string(
                r#"str: name = "Jade"
out("Hello, " + name)"#,
            )
        })
        .unwrap()
        .join()
        .unwrap()
        .unwrap();
    assert!(out.contains("Hello") && out.contains("Jade"), "output: {}", out);
}
