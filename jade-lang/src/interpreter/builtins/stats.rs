//! Statistics builtins: mean, median, mode, variance, stddev, normal_pdf, normal_cdf, normal_quantile.

use crate::interpreter::{Interpreter, Value};
use crate::parser::AstNode;
use std::f64::consts::PI;

fn as_f64(v: &Value) -> Option<f64> {
    match v {
        Value::Integer(i) => Some(*i as f64),
        Value::Float(f) => Some(*f),
        _ => None,
    }
}

fn list_of_f64(list: &[Value]) -> Result<Vec<f64>, String> {
    list.iter()
        .map(|v| as_f64(v).ok_or("stats: list must contain numbers".to_string()))
        .collect()
}

pub(super) fn try_call(
    interpreter: &mut Interpreter,
    name: &str,
    args: &[AstNode],
) -> Result<Option<Value>, String> {
    let v = match name {
        "mean" => Some(call_mean(interpreter, args)?),
        "median" => Some(call_median(interpreter, args)?),
        "mode" => Some(call_mode(interpreter, args)?),
        "variance" => Some(call_variance(interpreter, args)?),
        "stddev" => Some(call_stddev(interpreter, args)?),
        "normal_pdf" => Some(call_normal_pdf(interpreter, args)?),
        "normal_cdf" => Some(call_normal_cdf(interpreter, args)?),
        "normal_quantile" => Some(call_normal_quantile(interpreter, args)?),
        _ => None,
    };
    Ok(v)
}

fn call_mean(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("mean(list) expects exactly 1 argument".to_string());
    }
    let list_val = interpreter.eval_node(&args[0])?;
    let list = match list_val {
        Value::List(ref l) => list_of_f64(l)?,
        _ => return Err("mean(list) expects a list of numbers".to_string()),
    };
    if list.is_empty() {
        return Err("mean() of empty list undefined".to_string());
    }
    let sum: f64 = list.iter().sum();
    Ok(Value::Float(sum / list.len() as f64))
}

fn call_median(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("median(list) expects exactly 1 argument".to_string());
    }
    let list_val = interpreter.eval_node(&args[0])?;
    let mut list = match list_val {
        Value::List(ref l) => list_of_f64(l)?,
        _ => return Err("median(list) expects a list of numbers".to_string()),
    };
    if list.is_empty() {
        return Err("median() of empty list undefined".to_string());
    }
    list.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let mid = list.len() / 2;
    let med = if list.len() % 2 == 1 {
        list[mid]
    } else {
        (list[mid - 1] + list[mid]) / 2.0
    };
    Ok(Value::Float(med))
}

fn call_mode(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("mode(list) expects exactly 1 argument".to_string());
    }
    let list_val = interpreter.eval_node(&args[0])?;
    let list = match list_val {
        Value::List(ref l) => l,
        _ => return Err("mode(list) expects a list".to_string()),
    };
    if list.is_empty() {
        return Err("mode() of empty list undefined".to_string());
    }
    use std::collections::HashMap;
    let mut counts: HashMap<String, (Value, i64)> = HashMap::new();
    for v in list {
        let key = v.to_string();
        let entry = counts.entry(key).or_insert_with(|| (v.clone(), 0));
        entry.1 += 1;
    }
    let (_, (mode_val, _)) = counts
        .into_iter()
        .max_by_key(|(_, (_, c))| *c)
        .ok_or("mode() failed".to_string())?;
    Ok(mode_val)
}

fn call_variance(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("variance(list) expects exactly 1 argument".to_string());
    }
    let list_val = interpreter.eval_node(&args[0])?;
    let list = match list_val {
        Value::List(ref l) => list_of_f64(l)?,
        _ => return Err("variance(list) expects a list of numbers".to_string()),
    };
    if list.len() < 2 {
        return Err("variance() needs at least 2 elements".to_string());
    }
    let mean: f64 = list.iter().sum::<f64>() / list.len() as f64;
    let var = list.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (list.len() - 1) as f64;
    Ok(Value::Float(var))
}

fn call_stddev(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("stddev(list) expects exactly 1 argument".to_string());
    }
    let list_val = interpreter.eval_node(&args[0])?;
    let list = match list_val {
        Value::List(ref l) => list_of_f64(l)?,
        _ => return Err("stddev(list) expects a list of numbers".to_string()),
    };
    if list.len() < 2 {
        return Err("stddev() needs at least 2 elements".to_string());
    }
    let mean: f64 = list.iter().sum::<f64>() / list.len() as f64;
    let var = list.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (list.len() - 1) as f64;
    Ok(Value::Float(var.sqrt()))
}

fn normal_pdf(x: f64, mean: f64, std: f64) -> f64 {
    if std <= 0.0 {
        return f64::NAN;
    }
    let z = (x - mean) / std;
    (-z * z / 2.0).exp() / (std * (2.0 * PI).sqrt())
}

fn call_normal_pdf(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.is_empty() || args.len() > 3 {
        return Err("normal_pdf(x, [mean=0], [std=1]) expects 1 to 3 arguments".to_string());
    }
    let x = as_f64(&interpreter.eval_node(&args[0])?).ok_or("normal_pdf expects numbers".to_string())?;
    let mean = if args.len() >= 2 {
        as_f64(&interpreter.eval_node(&args[1])?).ok_or("normal_pdf expects numbers".to_string())?
    } else {
        0.0
    };
    let std = if args.len() >= 3 {
        as_f64(&interpreter.eval_node(&args[2])?).ok_or("normal_pdf expects numbers".to_string())?
    } else {
        1.0
    };
    Ok(Value::Float(normal_pdf(x, mean, std)))
}

fn erf_approx(x: f64) -> f64 {
    let a1 = 0.254829592;
    let a2 = -0.284496736;
    let a3 = 1.421413741;
    let a4 = -1.453152027;
    let a5 = 1.061405429;
    let p = 0.3275911;

    let x = x.abs();
    let t = 1.0 / (1.0 + p * x);
    let y = 1.0 - ((((a5 * t + a4) * t + a3) * t + a2) * t + a1) * t * (-x * x).exp();
    if x >= 0.0 {
        y
    } else {
        -y
    }
}

fn call_normal_cdf(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.is_empty() || args.len() > 3 {
        return Err("normal_cdf(x, [mean=0], [std=1]) expects 1 to 3 arguments".to_string());
    }
    let x = as_f64(&interpreter.eval_node(&args[0])?).ok_or("normal_cdf expects numbers".to_string())?;
    let mean = if args.len() >= 2 {
        as_f64(&interpreter.eval_node(&args[1])?).ok_or("normal_cdf expects numbers".to_string())?
    } else {
        0.0
    };
    let std = if args.len() >= 3 {
        as_f64(&interpreter.eval_node(&args[2])?).ok_or("normal_cdf expects numbers".to_string())?
    } else {
        1.0
    };
    if std <= 0.0 {
        return Err("normal_cdf: std must be positive".to_string());
    }
    let z = (x - mean) / std;
    let cdf = 0.5 * (1.0 + erf_approx(z / 2.0_f64.sqrt()));
    Ok(Value::Float(cdf))
}

fn normal_quantile_approx(p: f64) -> f64 {
    if p <= 0.0 || p >= 1.0 {
        return f64::NAN;
    }
    let a = [
        -3.969683028665376e1,
        2.209460984245205e2,
        -2.759285104469687e2,
        1.383577518672690e2,
        -3.066479806614716e1,
        2.506628277459239e0,
    ];
    let b = [
        -5.447609879822406e1,
        1.615858368580409e2,
        -1.556989798598866e2,
        6.680131188771972e1,
        -1.328068155288572e1,
    ];
    let p = p - 0.5;
    if p.abs() < 0.425 {
        let r = 0.180625 - p * p;
        let num = ((((a[0] * r + a[1]) * r + a[2]) * r + a[3]) * r + a[4]) * r + a[5];
        let den = ((((b[0] * r + b[1]) * r + b[2]) * r + b[3]) * r + b[4]) * r + 1.0;
        return p * num / den;
    }
    let r = if p < 0.0 { p + 0.5 } else { 0.5 - p };
    let r = (r.min(1.0)).max(0.0);
    let r = (-r.ln().ln()).sqrt();
    let c = [
        1.0 / 2.0,
        1.0 / 24.0,
        7.0 / 960.0,
        127.0 / 80640.0,
        4369.0 / 11612160.0,
        34807.0 / 364953600.0,
    ];
    let mut x = c[0];
    for i in 1..6 {
        x += c[i] * r.powi(-2 * i as i32);
    }
    let x = r * x;
    if p < 0.0 {
        x
    } else {
        -x
    }
}

fn call_normal_quantile(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("normal_quantile(p) expects exactly 1 argument".to_string());
    }
    let p = as_f64(&interpreter.eval_node(&args[0])?).ok_or("normal_quantile expects a number".to_string())?;
    Ok(Value::Float(normal_quantile_approx(p)))
}
