//! String builtins: split, join, substring, upper, lower, trim, replace, starts_with, ends_with, repeat, format.

use crate::interpreter::{Interpreter, Value};
use crate::parser::AstNode;

pub(super) fn try_call(
    interpreter: &mut Interpreter,
    name: &str,
    args: &[AstNode],
) -> Result<Option<Value>, String> {
    let v = match name {
        "split" => Some(call_split(interpreter, args)?),
        "join" => Some(call_join(interpreter, args)?),
        "substring" => Some(call_substring(interpreter, args)?),
        "upper" => Some(call_upper(interpreter, args)?),
        "lower" => Some(call_lower(interpreter, args)?),
        "trim" => Some(call_trim(interpreter, args)?),
        "replace" => Some(call_replace(interpreter, args)?),
        "starts_with" => Some(call_starts_with(interpreter, args)?),
        "ends_with" => Some(call_ends_with(interpreter, args)?),
        "repeat" => Some(call_repeat(interpreter, args)?),
        "format" => Some(call_format(interpreter, args)?),
        _ => None,
    };
    Ok(v)
}

fn call_split(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("split() expects exactly 2 arguments".to_string());
    }
    let string_val = interpreter.eval_node(&args[0])?;
    let delimiter_val = interpreter.eval_node(&args[1])?;
    match (string_val, delimiter_val) {
        (Value::String(s), Value::String(delim)) => {
            let parts: Vec<Value> = s
                .split(&delim)
                .map(|part| Value::String(part.to_string()))
                .collect();
            Ok(Value::List(parts))
        }
        _ => Err("split() expects string and delimiter arguments".to_string()),
    }
}

fn call_join(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("join() expects exactly 2 arguments".to_string());
    }
    let list_val = interpreter.eval_node(&args[0])?;
    let separator_val = interpreter.eval_node(&args[1])?;
    match (list_val, separator_val) {
        (Value::List(list), Value::String(sep)) => {
            let strings: Vec<String> = list
                .iter()
                .map(|v| match v {
                    Value::String(s) => s.clone(),
                    _ => v.to_string(),
                })
                .collect();
            Ok(Value::String(strings.join(&sep)))
        }
        _ => Err("join() expects list and separator string arguments".to_string()),
    }
}

fn call_substring(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    match args.len() {
        2 => {
            let string_val = interpreter.eval_node(&args[0])?;
            let start_val = interpreter.eval_node(&args[1])?;
            match (string_val, start_val) {
                (Value::String(s), Value::Integer(start)) => {
                    let chars: Vec<char> = s.chars().collect();
                    let start_idx = if start < 0 {
                        (chars.len() as i64 + start).max(0) as usize
                    } else {
                        (start as usize).min(chars.len())
                    };
                    let result: String = chars[start_idx..].iter().collect();
                    Ok(Value::String(result))
                }
                _ => Err("substring() expects string and integer arguments".to_string()),
            }
        }
        3 => {
            let string_val = interpreter.eval_node(&args[0])?;
            let start_val = interpreter.eval_node(&args[1])?;
            let end_val = interpreter.eval_node(&args[2])?;
            match (string_val, start_val, end_val) {
                (Value::String(s), Value::Integer(start), Value::Integer(end)) => {
                    let chars: Vec<char> = s.chars().collect();
                    let start_idx = if start < 0 {
                        (chars.len() as i64 + start).max(0) as usize
                    } else {
                        (start as usize).min(chars.len())
                    };
                    let end_idx = if end < 0 {
                        (chars.len() as i64 + end).max(0) as usize
                    } else {
                        (end as usize).min(chars.len())
                    };
                    let result = if start_idx <= end_idx {
                        chars[start_idx..end_idx].iter().collect()
                    } else {
                        String::new()
                    };
                    Ok(Value::String(result))
                }
                _ => Err("substring() expects string and integer arguments".to_string()),
            }
        }
        _ => Err("substring() expects 2 or 3 arguments".to_string()),
    }
}

fn call_upper(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("upper() expects exactly 1 argument".to_string());
    }
    let val = interpreter.eval_node(&args[0])?;
    match val {
        Value::String(s) => Ok(Value::String(s.to_uppercase())),
        _ => Err("upper() can only be called on strings".to_string()),
    }
}

fn call_lower(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("lower() expects exactly 1 argument".to_string());
    }
    let val = interpreter.eval_node(&args[0])?;
    match val {
        Value::String(s) => Ok(Value::String(s.to_lowercase())),
        _ => Err("lower() can only be called on strings".to_string()),
    }
}

fn call_trim(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("trim() expects exactly 1 argument".to_string());
    }
    let val = interpreter.eval_node(&args[0])?;
    match val {
        Value::String(s) => Ok(Value::String(s.trim().to_string())),
        _ => Err("trim() can only be called on strings".to_string()),
    }
}

fn call_replace(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 3 {
        return Err("replace() expects exactly 3 arguments".to_string());
    }
    let string_val = interpreter.eval_node(&args[0])?;
    let from_val = interpreter.eval_node(&args[1])?;
    let to_val = interpreter.eval_node(&args[2])?;
    match (string_val, from_val, to_val) {
        (Value::String(s), Value::String(from), Value::String(to)) => {
            Ok(Value::String(s.replace(&from, &to)))
        }
        _ => Err("replace() expects string arguments".to_string()),
    }
}

fn call_starts_with(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("starts_with() expects exactly 2 arguments".to_string());
    }
    let string_val = interpreter.eval_node(&args[0])?;
    let prefix_val = interpreter.eval_node(&args[1])?;
    match (string_val, prefix_val) {
        (Value::String(s), Value::String(prefix)) => Ok(Value::Boolean(s.starts_with(&prefix))),
        _ => Err("starts_with() expects string arguments".to_string()),
    }
}

fn call_ends_with(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("ends_with() expects exactly 2 arguments".to_string());
    }
    let string_val = interpreter.eval_node(&args[0])?;
    let suffix_val = interpreter.eval_node(&args[1])?;
    match (string_val, suffix_val) {
        (Value::String(s), Value::String(suffix)) => Ok(Value::Boolean(s.ends_with(&suffix))),
        _ => Err("ends_with() expects string arguments".to_string()),
    }
}

fn call_repeat(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("repeat() expects exactly 2 arguments".to_string());
    }
    let string_val = interpreter.eval_node(&args[0])?;
    let count_val = interpreter.eval_node(&args[1])?;
    match (string_val, count_val) {
        (Value::String(s), Value::Integer(count)) => {
            if count < 0 {
                return Err("repeat() count must be non-negative".to_string());
            }
            Ok(Value::String(s.repeat(count as usize)))
        }
        _ => Err("repeat() expects string and integer arguments".to_string()),
    }
}

fn call_format(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.is_empty() {
        return Err("format() expects at least 1 argument".to_string());
    }
    let template_val = interpreter.eval_node(&args[0])?;
    let template = match template_val {
        Value::String(s) => s,
        _ => return Err("format() first argument must be a string".to_string()),
    };
    let mut result = template;
    for arg in args.iter().skip(1) {
        let val = interpreter.eval_node(arg)?;
        let placeholder = "{}";
        if result.contains(placeholder) {
            result = result.replacen(placeholder, &val.to_string(), 1);
        }
    }
    Ok(Value::String(result))
}
