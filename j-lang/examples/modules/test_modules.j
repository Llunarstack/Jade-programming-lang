// Test module system

// This file will be imported by test_modules_main.j
fn | add(int | a, int | b) > a + b

fn | multiply(int | a, int | b) > a * b

int | PI -> 3

out("Module loaded: test_modules.j")
