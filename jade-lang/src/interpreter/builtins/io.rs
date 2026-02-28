//! File I/O builtins: read, write, read_lines, write_lines.

use crate::interpreter::{Interpreter, Value};
use crate::parser::AstNode;

pub(super) fn try_call(
    interpreter: &mut Interpreter,
    name: &str,
    args: &[AstNode],
) -> Result<Option<Value>, String> {
    let v = match name {
        "read" => Some(call_read(interpreter, args)?),
        "write" => Some(call_write(interpreter, args)?),
        "read_lines" => Some(call_read_lines(interpreter, args)?),
        "write_lines" => Some(call_write_lines(interpreter, args)?),
        _ => None,
    };
    Ok(v)
}

fn call_read(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("read() expects exactly 1 argument: read(filename)".to_string());
    }
    let filename_val = interpreter.eval_node(&args[0])?;
    let filename = match filename_val {
        Value::String(s) => s,
        _ => return Err("read() filename must be a string".to_string()),
    };
    std::fs::read_to_string(&filename)
        .map(Value::String)
        .map_err(|e| format!("Failed to read file '{}': {}", filename, e))
}

fn call_write(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(
            "write() expects exactly 2 arguments: write(filename, content)".to_string(),
        );
    }
    let filename_val = interpreter.eval_node(&args[0])?;
    let content_val = interpreter.eval_node(&args[1])?;
    let filename = match filename_val {
        Value::String(s) => s,
        _ => return Err("write() filename must be a string".to_string()),
    };
    let content = content_val.to_string();
    std::fs::write(&filename, content)
        .map(|_| Value::Boolean(true))
        .map_err(|e| format!("Failed to write file '{}': {}", filename, e))
}

fn call_read_lines(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(
            "read_lines() expects exactly 1 argument: read_lines(filename)".to_string(),
        );
    }
    let filename_val = interpreter.eval_node(&args[0])?;
    let filename = match filename_val {
        Value::String(s) => s,
        _ => return Err("read_lines() filename must be a string".to_string()),
    };
    let content = std::fs::read_to_string(&filename)
        .map_err(|e| format!("Failed to read file '{}': {}", filename, e))?;
    let lines: Vec<Value> = content
        .lines()
        .map(|line| Value::String(line.to_string()))
        .collect();
    Ok(Value::List(lines))
}

fn call_write_lines(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(
            "write_lines() expects exactly 2 arguments: write_lines(filename, lines)"
                .to_string(),
        );
    }
    let filename_val = interpreter.eval_node(&args[0])?;
    let lines_val = interpreter.eval_node(&args[1])?;
    let filename = match filename_val {
        Value::String(s) => s,
        _ => return Err("write_lines() filename must be a string".to_string()),
    };
    let lines = match lines_val {
        Value::List(list) => list
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join("\n"),
        _ => return Err("write_lines() lines must be a list".to_string()),
    };
    std::fs::write(&filename, lines)
        .map(|_| Value::Boolean(true))
        .map_err(|e| format!("Failed to write file '{}': {}", filename, e))
}
