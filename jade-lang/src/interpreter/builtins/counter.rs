//! Counter builtins: most_common, total.

use crate::interpreter::{Interpreter, Value};
use crate::parser::AstNode;

pub(super) fn try_call(
    interpreter: &mut Interpreter,
    name: &str,
    args: &[AstNode],
) -> Result<Option<Value>, String> {
    let v = match name {
        "most_common" => Some(call_most_common(interpreter, args)?),
        "total" => Some(call_total(interpreter, args)?),
        _ => None,
    };
    Ok(v)
}

fn call_most_common(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.is_empty() || args.len() > 2 {
        return Err("most_common() expects 1 or 2 arguments".to_string());
    }
    let counter_val = interpreter.eval_node(&args[0])?;
    let n = if args.len() == 2 {
        match interpreter.eval_node(&args[1])? {
            Value::Integer(i) => i as usize,
            _ => return Err("most_common() second argument must be an integer".to_string()),
        }
    } else {
        usize::MAX
    };
    match counter_val {
        Value::Counter(counter) => {
            let mut items: Vec<_> = counter.iter().collect();
            items.sort_by(|a, b| b.1.cmp(a.1));
            let result: Vec<Value> = items
                .iter()
                .take(n)
                .map(|(key, count)| {
                    Value::Tuple(vec![
                        Value::String(key.to_string()),
                        Value::Integer(**count),
                    ])
                })
                .collect();
            Ok(Value::List(result))
        }
        _ => Err("most_common() can only be called on counters".to_string()),
    }
}

fn call_total(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("total() expects exactly 1 argument".to_string());
    }
    let counter_val = interpreter.eval_node(&args[0])?;
    match counter_val {
        Value::Counter(counter) => {
            let total: i64 = counter.values().sum();
            Ok(Value::Integer(total))
        }
        _ => Err("total() can only be called on counters".to_string()),
    }
}
