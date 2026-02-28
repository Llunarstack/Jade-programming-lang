//! Random builtins: rand, rand_int, rand_choice, shuffle, rand_uniform.

use crate::interpreter::{Interpreter, Value};
use crate::parser::AstNode;

fn as_i64(v: &Value) -> Option<i64> {
    match v {
        Value::Integer(i) => Some(*i),
        _ => None,
    }
}

pub(super) fn try_call(
    interpreter: &mut Interpreter,
    name: &str,
    args: &[AstNode],
) -> Result<Option<Value>, String> {
    let v = match name {
        "rand" => Some(call_rand(interpreter, args)?),
        "rand_int" => Some(call_rand_int(interpreter, args)?),
        "rand_choice" => Some(call_rand_choice(interpreter, args)?),
        "shuffle" => Some(call_shuffle(interpreter, args)?),
        "rand_uniform" => Some(call_rand_uniform(interpreter, args)?),
        _ => None,
    };
    Ok(v)
}

fn call_rand(_interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if !args.is_empty() {
        return Err("rand() expects no arguments".to_string());
    }
    Ok(Value::Float(rand_f64()))
}

fn rand_f64() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    (t % 1_000_000) as f64 / 1_000_000.0
}

fn rand_u64() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64
}

fn call_rand_int(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("rand_int(lo, hi) expects exactly 2 arguments (inclusive range)".to_string());
    }
    let lo = as_i64(&interpreter.eval_node(&args[0])?).ok_or("rand_int expects integers".to_string())?;
    let hi = as_i64(&interpreter.eval_node(&args[1])?).ok_or("rand_int expects integers".to_string())?;
    if lo > hi {
        return Err("rand_int(lo, hi) requires lo <= hi".to_string());
    }
    let range = (hi - lo) as u64 + 1;
    let r = (rand_u64() % range) as i64 + lo;
    Ok(Value::Integer(r))
}

fn call_rand_choice(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("rand_choice(list) expects exactly 1 argument".to_string());
    }
    let list_val = interpreter.eval_node(&args[0])?;
    let list = match list_val {
        Value::List(ref l) => l,
        _ => return Err("rand_choice(list) expects a list".to_string()),
    };
    if list.is_empty() {
        return Err("rand_choice() of empty list".to_string());
    }
    let idx = (rand_u64() as usize) % list.len();
    Ok(list[idx].clone())
}

fn call_shuffle(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("shuffle(list) expects exactly 1 argument".to_string());
    }
    let list_val = interpreter.eval_node(&args[0])?;
    let mut list = match list_val {
        Value::List(l) => l,
        _ => return Err("shuffle(list) expects a list".to_string()),
    };
    for i in (1..list.len()).rev() {
        let j = (rand_u64() as usize) % (i + 1);
        list.swap(i, j);
    }
    Ok(Value::List(list))
}

fn call_rand_uniform(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("rand_uniform(a, b) expects exactly 2 arguments".to_string());
    }
    let a_val = interpreter.eval_node(&args[0])?;
    let b_val = interpreter.eval_node(&args[1])?;
    let a = match a_val {
        Value::Integer(i) => i as f64,
        Value::Float(f) => f,
        _ => return Err("rand_uniform expects numbers".to_string()),
    };
    let b = match b_val {
        Value::Integer(i) => i as f64,
        Value::Float(f) => f,
        _ => return Err("rand_uniform expects numbers".to_string()),
    };
    let t = rand_f64();
    let r = a + t * (b - a);
    Ok(Value::Float(r))
}
