//! Numeric builtins: abs, sign, min, max, clamp, floor, ceil, round, trunc,
//! sqrt, cbrt, pow, exp, ln, log10, log2, hypot, trig, lerp, clamp01.

use crate::interpreter::{Interpreter, Value};
use crate::parser::AstNode;

fn as_f64(v: &Value) -> Option<f64> {
    match v {
        Value::Integer(i) => Some(*i as f64),
        Value::Float(f) => Some(*f),
        _ => None,
    }
}

pub(super) fn try_call(
    interpreter: &mut Interpreter,
    name: &str,
    args: &[AstNode],
) -> Result<Option<Value>, String> {
    let v = match name {
        "abs" => Some(call_abs(interpreter, args)?),
        "sign" => Some(call_sign(interpreter, args)?),
        "min" => Some(call_min(interpreter, args)?),
        "max" => Some(call_max(interpreter, args)?),
        "clamp" => Some(call_clamp(interpreter, args)?),
        "sqrt" => Some(call_sqrt(interpreter, args)?),
        "cbrt" => Some(call_cbrt(interpreter, args)?),
        "pow" => Some(call_pow(interpreter, args)?),
        "ceil" => Some(call_ceil(interpreter, args)?),
        "floor" => Some(call_floor(interpreter, args)?),
        "round" => Some(call_round(interpreter, args)?),
        "trunc" => Some(call_trunc(interpreter, args)?),
        "exp" => Some(call_exp(interpreter, args)?),
        "ln" => Some(call_ln(interpreter, args)?),
        "log10" => Some(call_log10(interpreter, args)?),
        "log2" => Some(call_log2(interpreter, args)?),
        "hypot" => Some(call_hypot(interpreter, args)?),
        "sin" => Some(call_sin(interpreter, args)?),
        "cos" => Some(call_cos(interpreter, args)?),
        "tan" => Some(call_tan(interpreter, args)?),
        "asin" => Some(call_asin(interpreter, args)?),
        "acos" => Some(call_acos(interpreter, args)?),
        "atan" => Some(call_atan(interpreter, args)?),
        "atan2" => Some(call_atan2(interpreter, args)?),
        "sinh" => Some(call_sinh(interpreter, args)?),
        "cosh" => Some(call_cosh(interpreter, args)?),
        "tanh" => Some(call_tanh(interpreter, args)?),
        "lerp" => Some(call_lerp(interpreter, args)?),
        "clamp01" => Some(call_clamp01(interpreter, args)?),
        _ => None,
    };
    Ok(v)
}

fn call_abs(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("abs() expects exactly 1 argument".to_string());
    }
    let val = interpreter.eval_node(&args[0])?;
    match val {
        Value::Integer(i) => Ok(Value::Integer(i.abs())),
        Value::Float(f) => Ok(Value::Float(f.abs())),
        _ => Err("abs() can only be called on numbers".to_string()),
    }
}

fn call_sqrt(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("sqrt() expects exactly 1 argument".to_string());
    }
    let val = interpreter.eval_node(&args[0])?;
    match val {
        Value::Integer(i) => Ok(Value::Float((i as f64).sqrt())),
        Value::Float(f) => Ok(Value::Float(f.sqrt())),
        _ => Err("sqrt() can only be called on numbers".to_string()),
    }
}

fn call_pow(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("pow() expects exactly 2 arguments".to_string());
    }
    let base_val = interpreter.eval_node(&args[0])?;
    let exp_val = interpreter.eval_node(&args[1])?;
    match (base_val, exp_val) {
        (Value::Integer(base), Value::Integer(exp)) => {
            if exp >= 0 {
                if exp > u32::MAX as i64 {
                    return Err("Exponent too large".to_string());
                }
                base.checked_pow(exp as u32)
                    .map(Value::Integer)
                    .ok_or_else(|| format!("Integer overflow: {} ** {}", base, exp))
            } else {
                Ok(Value::Float((base as f64).powf(exp as f64)))
            }
        }
        (Value::Float(base), Value::Integer(exp)) => Ok(Value::Float(base.powf(exp as f64))),
        (Value::Integer(base), Value::Float(exp)) => Ok(Value::Float((base as f64).powf(exp))),
        (Value::Float(base), Value::Float(exp)) => Ok(Value::Float(base.powf(exp))),
        _ => Err("pow() expects numeric arguments".to_string()),
    }
}

fn call_ceil(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("ceil() expects exactly 1 argument".to_string());
    }
    let val = interpreter.eval_node(&args[0])?;
    match val {
        Value::Float(f) => Ok(Value::Integer(f.ceil() as i64)),
        Value::Integer(i) => Ok(Value::Integer(i)),
        _ => Err("ceil() can only be called on numbers".to_string()),
    }
}

fn call_floor(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("floor() expects exactly 1 argument".to_string());
    }
    let val = interpreter.eval_node(&args[0])?;
    match val {
        Value::Float(f) => Ok(Value::Integer(f.floor() as i64)),
        Value::Integer(i) => Ok(Value::Integer(i)),
        _ => Err("floor() can only be called on numbers".to_string()),
    }
}

fn call_sign(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("sign() expects exactly 1 argument".to_string());
    }
    let val = interpreter.eval_node(&args[0])?;
    let n = as_f64(&val).ok_or("sign() expects a number".to_string())?;
    Ok(Value::Integer(if n > 0.0 { 1 } else if n < 0.0 { -1 } else { 0 }))
}

fn call_min(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.is_empty() {
        return Err("min() expects at least 1 argument".to_string());
    }
    let mut acc = interpreter.eval_node(&args[0])?;
    for a in args.iter().skip(1) {
        let v = interpreter.eval_node(a)?;
        let (a_f, v_f) = (as_f64(&acc), as_f64(&v));
        if let (Some(x), Some(y)) = (a_f, v_f) {
            acc = if x <= y { acc } else { v };
        } else {
            return Err("min() expects numbers".to_string());
        }
    }
    Ok(acc)
}

fn call_max(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.is_empty() {
        return Err("max() expects at least 1 argument".to_string());
    }
    let mut acc = interpreter.eval_node(&args[0])?;
    for a in args.iter().skip(1) {
        let v = interpreter.eval_node(a)?;
        let (a_f, v_f) = (as_f64(&acc), as_f64(&v));
        if let (Some(x), Some(y)) = (a_f, v_f) {
            acc = if x >= y { acc } else { v };
        } else {
            return Err("max() expects numbers".to_string());
        }
    }
    Ok(acc)
}

fn call_clamp(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 3 {
        return Err("clamp(x, lo, hi) expects exactly 3 arguments".to_string());
    }
    let x = as_f64(&interpreter.eval_node(&args[0])?).ok_or("clamp expects numbers".to_string())?;
    let lo = as_f64(&interpreter.eval_node(&args[1])?).ok_or("clamp expects numbers".to_string())?;
    let hi = as_f64(&interpreter.eval_node(&args[2])?).ok_or("clamp expects numbers".to_string())?;
    let r = x.clamp(lo, hi);
    Ok(if r.fract() == 0.0 && r.abs() <= i64::MAX as f64 {
        Value::Integer(r as i64)
    } else {
        Value::Float(r)
    })
}

fn call_round(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("round() expects exactly 1 argument".to_string());
    }
    let val = interpreter.eval_node(&args[0])?;
    let f = as_f64(&val).ok_or("round() expects a number".to_string())?;
    Ok(Value::Integer(f.round() as i64))
}

fn call_trunc(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("trunc() expects exactly 1 argument".to_string());
    }
    let val = interpreter.eval_node(&args[0])?;
    let f = as_f64(&val).ok_or("trunc() expects a number".to_string())?;
    Ok(Value::Integer(f.trunc() as i64))
}

fn call_cbrt(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("cbrt() expects exactly 1 argument".to_string());
    }
    let val = interpreter.eval_node(&args[0])?;
    let f = as_f64(&val).ok_or("cbrt() expects a number".to_string())?;
    Ok(Value::Float(f.cbrt()))
}

fn call_exp(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("exp() expects exactly 1 argument".to_string());
    }
    let val = interpreter.eval_node(&args[0])?;
    let f = as_f64(&val).ok_or("exp() expects a number".to_string())?;
    Ok(Value::Float(f.exp()))
}

fn call_ln(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("ln() expects exactly 1 argument".to_string());
    }
    let val = interpreter.eval_node(&args[0])?;
    let f = as_f64(&val).ok_or("ln() expects a number".to_string())?;
    if f <= 0.0 {
        return Err("ln() argument must be positive".to_string());
    }
    Ok(Value::Float(f.ln()))
}

fn call_log10(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("log10() expects exactly 1 argument".to_string());
    }
    let val = interpreter.eval_node(&args[0])?;
    let f = as_f64(&val).ok_or("log10() expects a number".to_string())?;
    if f <= 0.0 {
        return Err("log10() argument must be positive".to_string());
    }
    Ok(Value::Float(f.log10()))
}

fn call_log2(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("log2() expects exactly 1 argument".to_string());
    }
    let val = interpreter.eval_node(&args[0])?;
    let f = as_f64(&val).ok_or("log2() expects a number".to_string())?;
    if f <= 0.0 {
        return Err("log2() argument must be positive".to_string());
    }
    Ok(Value::Float(f.log2()))
}

fn call_hypot(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("hypot(x, y) expects exactly 2 arguments".to_string());
    }
    let x = as_f64(&interpreter.eval_node(&args[0])?).ok_or("hypot expects numbers".to_string())?;
    let y = as_f64(&interpreter.eval_node(&args[1])?).ok_or("hypot expects numbers".to_string())?;
    Ok(Value::Float(x.hypot(y)))
}

fn call_sin(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("sin() expects exactly 1 argument".to_string());
    }
    let f = as_f64(&interpreter.eval_node(&args[0])?).ok_or("sin() expects a number".to_string())?;
    Ok(Value::Float(f.sin()))
}

fn call_cos(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("cos() expects exactly 1 argument".to_string());
    }
    let f = as_f64(&interpreter.eval_node(&args[0])?).ok_or("cos() expects a number".to_string())?;
    Ok(Value::Float(f.cos()))
}

fn call_tan(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("tan() expects exactly 1 argument".to_string());
    }
    let f = as_f64(&interpreter.eval_node(&args[0])?).ok_or("tan() expects a number".to_string())?;
    Ok(Value::Float(f.tan()))
}

fn call_asin(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("asin() expects exactly 1 argument".to_string());
    }
    let f = as_f64(&interpreter.eval_node(&args[0])?).ok_or("asin() expects a number".to_string())?;
    if f < -1.0 || f > 1.0 {
        return Err("asin() argument must be in [-1, 1]".to_string());
    }
    Ok(Value::Float(f.asin()))
}

fn call_acos(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("acos() expects exactly 1 argument".to_string());
    }
    let f = as_f64(&interpreter.eval_node(&args[0])?).ok_or("acos() expects a number".to_string())?;
    if f < -1.0 || f > 1.0 {
        return Err("acos() argument must be in [-1, 1]".to_string());
    }
    Ok(Value::Float(f.acos()))
}

fn call_atan(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("atan() expects exactly 1 argument".to_string());
    }
    let f = as_f64(&interpreter.eval_node(&args[0])?).ok_or("atan() expects a number".to_string())?;
    Ok(Value::Float(f.atan()))
}

fn call_atan2(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("atan2(y, x) expects exactly 2 arguments".to_string());
    }
    let y = as_f64(&interpreter.eval_node(&args[0])?).ok_or("atan2 expects numbers".to_string())?;
    let x = as_f64(&interpreter.eval_node(&args[1])?).ok_or("atan2 expects numbers".to_string())?;
    Ok(Value::Float(y.atan2(x)))
}

fn call_sinh(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("sinh() expects exactly 1 argument".to_string());
    }
    let f = as_f64(&interpreter.eval_node(&args[0])?).ok_or("sinh() expects a number".to_string())?;
    Ok(Value::Float(f.sinh()))
}

fn call_cosh(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("cosh() expects exactly 1 argument".to_string());
    }
    let f = as_f64(&interpreter.eval_node(&args[0])?).ok_or("cosh() expects a number".to_string())?;
    Ok(Value::Float(f.cosh()))
}

fn call_tanh(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("tanh() expects exactly 1 argument".to_string());
    }
    let f = as_f64(&interpreter.eval_node(&args[0])?).ok_or("tanh() expects a number".to_string())?;
    Ok(Value::Float(f.tanh()))
}

fn call_lerp(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 3 {
        return Err("lerp(a, b, t) expects exactly 3 arguments".to_string());
    }
    let a = as_f64(&interpreter.eval_node(&args[0])?).ok_or("lerp expects numbers".to_string())?;
    let b = as_f64(&interpreter.eval_node(&args[1])?).ok_or("lerp expects numbers".to_string())?;
    let t = as_f64(&interpreter.eval_node(&args[2])?).ok_or("lerp expects numbers".to_string())?;
    let r = a + t * (b - a);
    Ok(if r.fract() == 0.0 && r.abs() <= i64::MAX as f64 {
        Value::Integer(r as i64)
    } else {
        Value::Float(r)
    })
}

fn call_clamp01(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("clamp01(x) expects exactly 1 argument".to_string());
    }
    let x = as_f64(&interpreter.eval_node(&args[0])?).ok_or("clamp01 expects a number".to_string())?;
    Ok(Value::Float(x.clamp(0.0, 1.0)))
}
