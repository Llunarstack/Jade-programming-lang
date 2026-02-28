//! Memoization builtin: memo(f) returns a memoized version of f.

use crate::interpreter::{Interpreter, Value};
use crate::parser::AstNode;

pub(super) fn try_call(
    interpreter: &mut Interpreter,
    name: &str,
    args: &[AstNode],
) -> Result<Option<Value>, String> {
    if name != "memo" {
        return Ok(None);
    }
    if args.len() != 1 {
        return Err("memo(f) expects exactly 1 argument (a function)".to_string());
    }
    let f = interpreter.eval_node(&args[0])?;
    match &f {
        Value::Function { .. } => {}
        _ => return Err("memo(f) expects a function".to_string()),
    }
    let id = interpreter.next_memo_id;
    interpreter.next_memo_id += 1;
    Ok(Some(Value::Memoized {
        id,
        inner: Box::new(f),
    }))
}
