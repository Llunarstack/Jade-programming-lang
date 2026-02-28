//! Jade Programming Language — core library.
//!
//! This crate provides the lexer, parser, interpreter, and tooling for the Jade language.
//! The `jade` binary uses this library for REPL, run, build, check, and jolt commands.

pub mod compiler;
pub mod error;
pub mod interpreter;
pub mod jit;
pub mod jolt;
pub mod lexer;
pub mod parser;
pub mod repl;
pub mod runtime;

// Re-export main types for consumers of the library
pub use error::JError;
pub use interpreter::Interpreter;
pub use lexer::{Lexer, Token, TokenType};
pub use parser::Parser;

/// Run Jade source and return captured output as a string.
pub fn run_source_to_string(source: &str) -> Result<String, String> {
    let source = source.replace("\r\n", "\n").replace('\r', "\n");
    let mut interpreter = Interpreter::new();
    interpreter.set_output_capture(true);
    interpreter.run(&source)?;
    Ok(interpreter
        .take_captured_output()
        .unwrap_or_default())
}
