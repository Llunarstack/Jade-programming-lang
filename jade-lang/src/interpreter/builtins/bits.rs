//! Bit manipulation builtins: bit_set, set_bit, clear_bit, toggle_bit, count_bits/bit_count,
//! leading_zeros, trailing_zeros, highest_set_bit, lowest_set_bit, is_power_of_two,
//! next_power_of_two, log2_floor, log2_ceil, exp2, log10_floor, digits.

use crate::interpreter::{Interpreter, Value};
use crate::parser::AstNode;

pub(super) fn try_call(
    interpreter: &mut Interpreter,
    name: &str,
    args: &[AstNode],
) -> Result<Option<Value>, String> {
    let v = match name {
        "bit_set" => Some(call_bit_set(interpreter, args)?),
        "set_bit" => Some(call_set_bit(interpreter, args)?),
        "clear_bit" => Some(call_clear_bit(interpreter, args)?),
        "toggle_bit" => Some(call_toggle_bit(interpreter, args)?),
        "count_bits" | "bit_count" => Some(call_count_bits(interpreter, args)?),
        "leading_zeros" => Some(call_leading_zeros(interpreter, args)?),
        "trailing_zeros" => Some(call_trailing_zeros(interpreter, args)?),
        "highest_set_bit" => Some(call_highest_set_bit(interpreter, args)?),
        "lowest_set_bit" => Some(call_lowest_set_bit(interpreter, args)?),
        "is_power_of_two" => Some(call_is_power_of_two(interpreter, args)?),
        "next_power_of_two" => Some(call_next_power_of_two(interpreter, args)?),
        "log2_floor" | "ilog2" => Some(call_log2_floor(interpreter, args)?),
        "log2_ceil" => Some(call_log2_ceil(interpreter, args)?),
        "exp2" => Some(call_exp2(interpreter, args)?),
        "log10_floor" => Some(call_log10_floor(interpreter, args)?),
        "digits" => Some(call_digits(interpreter, args)?),
        _ => None,
    };
    Ok(v)
}

fn int_pos(n: i64, pos_val: i64) -> Result<(u64, u32), String> {
    if pos_val < 0 || pos_val > 63 {
        return Err("bit position must be 0..63".to_string());
    }
    let pos = pos_val as u32;
    let n_bits = n as u64;
    Ok((n_bits, pos))
}

fn call_bit_set(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("bit_set(n, pos) expects exactly 2 arguments".to_string());
    }
    let n_val = interpreter.eval_node(&args[0])?;
    let pos_val = interpreter.eval_node(&args[1])?;
    let (n, pos) = match (n_val, pos_val) {
        (Value::Integer(n), Value::Integer(p)) => int_pos(n, p)?,
        _ => return Err("bit_set(n, pos) expects two integers".to_string()),
    };
    Ok(Value::Boolean((n & (1 << pos)) != 0))
}

fn call_set_bit(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("set_bit(n, pos) expects exactly 2 arguments".to_string());
    }
    let n_val = interpreter.eval_node(&args[0])?;
    let pos_val = interpreter.eval_node(&args[1])?;
    let (n, pos) = match (n_val, pos_val) {
        (Value::Integer(n), Value::Integer(p)) => int_pos(n, p)?,
        _ => return Err("set_bit(n, pos) expects two integers".to_string()),
    };
    let result = n | (1 << pos);
    Ok(Value::Integer(result as i64))
}

fn call_clear_bit(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("clear_bit(n, pos) expects exactly 2 arguments".to_string());
    }
    let n_val = interpreter.eval_node(&args[0])?;
    let pos_val = interpreter.eval_node(&args[1])?;
    let (n, pos) = match (n_val, pos_val) {
        (Value::Integer(n), Value::Integer(p)) => int_pos(n, p)?,
        _ => return Err("clear_bit(n, pos) expects two integers".to_string()),
    };
    let result = n & !(1 << pos);
    Ok(Value::Integer(result as i64))
}

fn call_toggle_bit(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("toggle_bit(n, pos) expects exactly 2 arguments".to_string());
    }
    let n_val = interpreter.eval_node(&args[0])?;
    let pos_val = interpreter.eval_node(&args[1])?;
    let (n, pos) = match (n_val, pos_val) {
        (Value::Integer(n), Value::Integer(p)) => int_pos(n, p)?,
        _ => return Err("toggle_bit(n, pos) expects two integers".to_string()),
    };
    let result = n ^ (1 << pos);
    Ok(Value::Integer(result as i64))
}

fn call_count_bits(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("count_bits(n) expects exactly 1 argument".to_string());
    }
    let n_val = interpreter.eval_node(&args[0])?;
    let n = match n_val {
        Value::Integer(n) => n,
        _ => return Err("count_bits(n) expects an integer".to_string()),
    };
    let count = (n as u64).count_ones();
    Ok(Value::Integer(count as i64))
}

fn get_int_arg(interpreter: &mut Interpreter, args: &[AstNode], name: &str) -> Result<i64, String> {
    if args.len() != 1 {
        return Err(format!("{}(n) expects exactly 1 argument", name));
    }
    match interpreter.eval_node(&args[0])? {
        Value::Integer(n) => Ok(n),
        _ => Err(format!("{}(n) expects an integer", name)),
    }
}

fn call_leading_zeros(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    let n = get_int_arg(interpreter, args, "leading_zeros")?;
    Ok(Value::Integer((n as u64).leading_zeros() as i64))
}

fn call_trailing_zeros(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    let n = get_int_arg(interpreter, args, "trailing_zeros")?;
    Ok(Value::Integer((n as u64).trailing_zeros() as i64))
}

fn call_highest_set_bit(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    let n = get_int_arg(interpreter, args, "highest_set_bit")?;
    if n <= 0 {
        return Err("highest_set_bit(n) requires n > 0".to_string());
    }
    let u = n as u64;
    Ok(Value::Integer(63 - u.leading_zeros() as i64))
}

fn call_lowest_set_bit(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    let n = get_int_arg(interpreter, args, "lowest_set_bit")?;
    if n == 0 {
        return Err("lowest_set_bit(0) undefined".to_string());
    }
    Ok(Value::Integer((n as u64).trailing_zeros() as i64))
}

fn call_is_power_of_two(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    let n = get_int_arg(interpreter, args, "is_power_of_two")?;
    Ok(Value::Boolean(n > 0 && (n & (n - 1)) == 0))
}

fn call_next_power_of_two(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    let n = get_int_arg(interpreter, args, "next_power_of_two")?;
    if n <= 0 {
        return Ok(Value::Integer(1));
    }
    let u = (n as u64).next_power_of_two();
    Ok(Value::Integer(u.min(i64::MAX as u64) as i64))
}

fn call_log2_floor(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    let n = get_int_arg(interpreter, args, "log2_floor")?;
    if n <= 0 {
        return Err("log2_floor(n) requires n > 0".to_string());
    }
    Ok(Value::Integer(63 - (n as u64).leading_zeros() as i64))
}

fn call_log2_ceil(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    let n = get_int_arg(interpreter, args, "log2_ceil")?;
    if n <= 0 {
        return Err("log2_ceil(n) requires n > 0".to_string());
    }
    let u = n as u64;
    let floor = 63 - u.leading_zeros() as i64;
    Ok(Value::Integer(if u.is_power_of_two() { floor } else { floor + 1 }))
}

fn call_exp2(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    let k = get_int_arg(interpreter, args, "exp2")?;
    if k < 0 || k > 63 {
        return Err("exp2(k) requires 0 <= k <= 63".to_string());
    }
    Ok(Value::Integer(1i64 << k))
}

fn call_log10_floor(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    let n = get_int_arg(interpreter, args, "log10_floor")?;
    if n <= 0 {
        return Err("log10_floor(n) requires n > 0".to_string());
    }
    let mut d = 0i64;
    let mut x = n.abs();
    while x >= 10 {
        x /= 10;
        d += 1;
    }
    Ok(Value::Integer(d))
}

fn call_digits(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    let n = get_int_arg(interpreter, args, "digits")?;
    if n == 0 {
        return Ok(Value::Integer(1));
    }
    let mut d = 0i64;
    let mut x = n.abs();
    while x > 0 {
        x /= 10;
        d += 1;
    }
    Ok(Value::Integer(d))
}
