//! Core builtins J needs: replicate, assert, iota, identity, default.

use crate::interpreter::{Interpreter, Value};
use crate::parser::AstNode;

pub(super) fn try_call(
    interpreter: &mut Interpreter,
    name: &str,
    args: &[AstNode],
) -> Result<Option<Value>, String> {
    let v = match name {
        "replicate" | "repeat_n" => Some(call_replicate(interpreter, args)?),
        "assert" => Some(call_assert(interpreter, args)?),
        "iota" => Some(call_iota(interpreter, args)?),
        "identity" | "id" => Some(call_identity(interpreter, args)?),
        "default" => Some(call_default(interpreter, args)?),
        _ => None,
    };
    Ok(v)
}

fn call_replicate(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("replicate(n, x) expects exactly 2 arguments".to_string());
    }
    let n_val = interpreter.eval_node(&args[0])?;
    let x_val = interpreter.eval_node(&args[1])?;
    let n = match n_val {
        Value::Integer(i) if i >= 0 => i as usize,
        Value::Integer(_) => return Err("replicate(n, x) requires n >= 0".to_string()),
        _ => return Err("replicate(n, x) requires integer n".to_string()),
    };
    Ok(Value::List(vec![x_val; n]))
}

fn call_assert(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.is_empty() {
        return Err("assert(condition) expects at least 1 argument".to_string());
    }
    let cond = interpreter.eval_node(&args[0])?;
    let ok = match &cond {
        Value::Boolean(b) => *b,
        _ => return Err("assert(condition) expects boolean condition".to_string()),
    };
    if !ok {
        let msg = if args.len() >= 2 {
            interpreter.eval_node(&args[1])?.to_string()
        } else {
            "assertion failed".to_string()
        };
        return Err(msg);
    }
    Ok(Value::None)
}

fn call_iota(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("iota(n) expects exactly 1 argument".to_string());
    }
    let n_val = interpreter.eval_node(&args[0])?;
    let n = match n_val {
        Value::Integer(i) if i >= 0 => i as usize,
        Value::Integer(_) => return Err("iota(n) requires n >= 0".to_string()),
        _ => return Err("iota(n) requires integer n".to_string()),
    };
    Ok(Value::List(
        (0..n).map(|i| Value::Integer(i as i64)).collect(),
    ))
}

fn call_identity(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("identity(x) expects exactly 1 argument".to_string());
    }
    interpreter.eval_node(&args[0])
}

fn call_default(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("default(value, fallback) expects exactly 2 arguments".to_string());
    }
    let val = interpreter.eval_node(&args[0])?;
    if matches!(val, Value::None) {
        interpreter.eval_node(&args[1])
    } else {
        Ok(val)
    }
}
