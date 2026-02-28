//! Enum builtins: enum_name, enum_value, enum_has.

use crate::interpreter::{Interpreter, Value};
use crate::parser::AstNode;

pub(super) fn try_call(
    interpreter: &mut Interpreter,
    name: &str,
    args: &[AstNode],
) -> Result<Option<Value>, String> {
    let v = match name {
        "enum_name" => Some(call_enum_name(interpreter, args)?),
        "enum_value" => Some(call_enum_value(interpreter, args)?),
        "enum_has" => Some(call_enum_has(interpreter, args)?),
        _ => None,
    };
    Ok(v)
}

fn call_enum_name(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("enum_name() expects exactly 2 arguments: enum_name(enum, value)".to_string());
    }
    let enum_val = interpreter.eval_node(&args[0])?;
    let value_val = interpreter.eval_node(&args[1])?;
    match enum_val {
        Value::Dict(dict) => {
            for (name, value) in dict.iter() {
                if interpreter.values_equal(value, &value_val) {
                    return Ok(Value::String(name.clone()));
                }
            }
            Ok(Value::None)
        }
        _ => Err("enum_name() can only be called on enums".to_string()),
    }
}

fn call_enum_value(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("enum_value() expects exactly 2 arguments: enum_value(enum, name)".to_string());
    }
    let enum_val = interpreter.eval_node(&args[0])?;
    let name_val = interpreter.eval_node(&args[1])?;
    match (enum_val, name_val) {
        (Value::Dict(dict), Value::String(name)) => Ok(dict
            .get(&name)
            .cloned()
            .unwrap_or(Value::None)),
        _ => Err("enum_value() can only be called on enums with string name".to_string()),
    }
}

fn call_enum_has(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("enum_has() expects exactly 2 arguments: enum_has(enum, value)".to_string());
    }
    let enum_val = interpreter.eval_node(&args[0])?;
    let value_val = interpreter.eval_node(&args[1])?;
    match enum_val {
        Value::Dict(dict) => {
            let has = dict
                .values()
                .any(|v| interpreter.values_equal(v, &value_val));
            Ok(Value::Boolean(has))
        }
        _ => Err("enum_has() can only be called on enums".to_string()),
    }
}
