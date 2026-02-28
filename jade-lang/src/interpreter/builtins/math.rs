//! Math and number-theory builtins: gcd, lcm, egcd, is_prime, next_prime, prev_prime,
//! factorial, fibonacci, binomial, factors, totient, gamma, polyval.

use crate::interpreter::{Interpreter, Value};
use crate::parser::AstNode;

pub(super) fn try_call(
    interpreter: &mut Interpreter,
    name: &str,
    args: &[AstNode],
) -> Result<Option<Value>, String> {
    let v = match name {
        "gcd" => Some(call_gcd(interpreter, args)?),
        "lcm" => Some(call_lcm(interpreter, args)?),
        "egcd" => Some(call_egcd(interpreter, args)?),
        "is_prime" => Some(call_is_prime(interpreter, args)?),
        "next_prime" => Some(call_next_prime(interpreter, args)?),
        "prev_prime" => Some(call_prev_prime(interpreter, args)?),
        "factorial" => Some(call_factorial(interpreter, args)?),
        "fibonacci" => Some(call_fibonacci(interpreter, args)?),
        "binomial" => Some(call_binomial(interpreter, args)?),
        "factors" => Some(call_factors(interpreter, args)?),
        "totient" => Some(call_totient(interpreter, args)?),
        "gamma" => Some(call_gamma(interpreter, args)?),
        "polyval" => Some(call_polyval(interpreter, args)?),
        "mod_add" => Some(call_mod_add(interpreter, args)?),
        "mod_sub" => Some(call_mod_sub(interpreter, args)?),
        "mod_mul" => Some(call_mod_mul(interpreter, args)?),
        "mod_pow" => Some(call_mod_pow(interpreter, args)?),
        "mod_inv" => Some(call_mod_inv(interpreter, args)?),
        _ => None,
    };
    Ok(v)
}

fn call_gcd(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("gcd() expects exactly 2 arguments".to_string());
    }
    let a_val = interpreter.eval_node(&args[0])?;
    let b_val = interpreter.eval_node(&args[1])?;
    match (a_val, b_val) {
        (Value::Integer(mut a), Value::Integer(mut b)) => {
            a = a.abs();
            b = b.abs();
            while b != 0 {
                let temp = b;
                b = a % b;
                a = temp;
            }
            Ok(Value::Integer(a))
        }
        _ => Err("gcd() expects two integers".to_string()),
    }
}

fn call_lcm(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("lcm() expects exactly 2 arguments".to_string());
    }
    let a_val = interpreter.eval_node(&args[0])?;
    let b_val = interpreter.eval_node(&args[1])?;
    match (a_val, b_val) {
        (Value::Integer(a), Value::Integer(b)) => {
            if a == 0 || b == 0 {
                return Ok(Value::Integer(0));
            }
            let a = a.abs();
            let b = b.abs();
            let mut gcd_a = a;
            let mut gcd_b = b;
            while gcd_b != 0 {
                let temp = gcd_b;
                gcd_b = gcd_a % gcd_b;
                gcd_a = temp;
            }
            Ok(Value::Integer((a * b) / gcd_a))
        }
        _ => Err("lcm() expects two integers".to_string()),
    }
}

fn call_is_prime(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("is_prime() expects exactly 1 argument".to_string());
    }
    let val = interpreter.eval_node(&args[0])?;
    match val {
        Value::Integer(n) => {
            if n < 2 {
                return Ok(Value::Boolean(false));
            }
            if n == 2 {
                return Ok(Value::Boolean(true));
            }
            if n % 2 == 0 {
                return Ok(Value::Boolean(false));
            }
            let mut i = 3;
            while i * i <= n {
                if n % i == 0 {
                    return Ok(Value::Boolean(false));
                }
                i += 2;
            }
            Ok(Value::Boolean(true))
        }
        _ => Err("is_prime() expects an integer".to_string()),
    }
}

fn call_factorial(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("factorial() expects exactly 1 argument".to_string());
    }
    let val = interpreter.eval_node(&args[0])?;
    match val {
        Value::Integer(n) => {
            if n < 0 {
                return Err("factorial() requires non-negative integer".to_string());
            }
            if n > 20 {
                return Err("factorial() overflow: n must be <= 20".to_string());
            }
            let mut result = 1i64;
            for i in 2..=n {
                result *= i;
            }
            Ok(Value::Integer(result))
        }
        _ => Err("factorial() expects an integer".to_string()),
    }
}

fn call_fibonacci(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("fibonacci() expects exactly 1 argument".to_string());
    }
    let val = interpreter.eval_node(&args[0])?;
    match val {
        Value::Integer(n) => {
            if n < 0 {
                return Err("fibonacci() requires non-negative integer".to_string());
            }
            if n == 0 {
                return Ok(Value::Integer(0));
            }
            if n == 1 {
                return Ok(Value::Integer(1));
            }
            let mut a = 0i64;
            let mut b = 1i64;
            for _ in 2..=n {
                let temp = a + b;
                a = b;
                b = temp;
            }
            Ok(Value::Integer(b))
        }
        _ => Err("fibonacci() expects an integer".to_string()),
    }
}

fn call_egcd(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("egcd(a, b) expects exactly 2 arguments".to_string());
    }
    let a_val = interpreter.eval_node(&args[0])?;
    let b_val = interpreter.eval_node(&args[1])?;
    let (mut a, mut b) = match (a_val, b_val) {
        (Value::Integer(x), Value::Integer(y)) => (x, y),
        _ => return Err("egcd(a, b) expects two integers".to_string()),
    };
    let (mut x0, mut x1) = (1i64, 0i64);
    let (mut y0, mut y1) = (0i64, 1i64);
    while b != 0 {
        let q = a / b;
        (a, b) = (b, a - q * b);
        (x0, x1) = (x1, x0 - q * x1);
        (y0, y1) = (y1, y0 - q * y1);
    }
    Ok(Value::Tuple(vec![
        Value::Integer(a),
        Value::Integer(x0),
        Value::Integer(y0),
    ]))
}

fn is_prime_n(n: i64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

fn call_next_prime(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("next_prime(n) expects exactly 1 argument".to_string());
    }
    let val = interpreter.eval_node(&args[0])?;
    let n = match val {
        Value::Integer(n) => n,
        _ => return Err("next_prime(n) expects an integer".to_string()),
    };
    let mut k = n + 1;
    if k < 2 {
        k = 2;
    }
    if k == 2 {
        return Ok(Value::Integer(2));
    }
    if k % 2 == 0 {
        k += 1;
    }
    while !is_prime_n(k) {
        k += 2;
    }
    Ok(Value::Integer(k))
}

fn call_prev_prime(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("prev_prime(n) expects exactly 1 argument".to_string());
    }
    let val = interpreter.eval_node(&args[0])?;
    let n = match val {
        Value::Integer(n) => n,
        _ => return Err("prev_prime(n) expects an integer".to_string()),
    };
    if n <= 2 {
        return Err("prev_prime(n): no prime less than n for n <= 2".to_string());
    }
    let mut k = n - 1;
    if k % 2 == 0 {
        k -= 1;
    }
    while k >= 2 && !is_prime_n(k) {
        k -= 2;
    }
    if k < 2 {
        return Err("prev_prime(n): no prime found".to_string());
    }
    Ok(Value::Integer(k))
}

fn call_binomial(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("binomial(n, k) expects exactly 2 arguments".to_string());
    }
    let n_val = interpreter.eval_node(&args[0])?;
    let k_val = interpreter.eval_node(&args[1])?;
    let (n, k) = match (n_val, k_val) {
        (Value::Integer(n), Value::Integer(k)) => (n, k),
        _ => return Err("binomial(n, k) expects two integers".to_string()),
    };
    if n < 0 || k < 0 || k > n {
        return Err("binomial(n, k) requires 0 <= k <= n".to_string());
    }
    let n = n as u64;
    let mut k = k as u64;
    if k > n - k {
        k = n - k;
    }
    let mut result: u64 = 1;
    for i in 0..k {
        result = result
            .checked_mul(n - i)
            .and_then(|r| r.checked_div(i + 1))
            .ok_or_else(|| "binomial(n, k) overflow".to_string())?;
    }
    Ok(Value::Integer(result as i64))
}

fn call_factors(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("factors(n) expects exactly 1 argument".to_string());
    }
    let val = interpreter.eval_node(&args[0])?;
    let mut n = match val {
        Value::Integer(n) if n > 1 => n.abs(),
        Value::Integer(1) => return Ok(Value::List(vec![])),
        _ => return Err("factors(n) expects a positive integer".to_string()),
    };
    let mut out = Vec::new();
    let mut d = 2i64;
    while d * d <= n {
        while n % d == 0 {
            out.push(Value::Integer(d));
            n /= d;
        }
        d += 1;
    }
    if n > 1 {
        out.push(Value::Integer(n));
    }
    Ok(Value::List(out))
}

fn call_totient(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("totient(n) expects exactly 1 argument".to_string());
    }
    let val = interpreter.eval_node(&args[0])?;
    let n = match val {
        Value::Integer(n) if n > 0 => n,
        _ => return Err("totient(n) expects a positive integer".to_string()),
    };
    let mut result = n;
    let mut x = n;
    let mut d = 2i64;
    while d * d <= x {
        if x % d == 0 {
            while x % d == 0 {
                x /= d;
            }
            result -= result / d;
        }
        d += 1;
    }
    if x > 1 {
        result -= result / x;
    }
    Ok(Value::Integer(result))
}

fn call_gamma(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("gamma(n) expects exactly 1 argument".to_string());
    }
    let val = interpreter.eval_node(&args[0])?;
    match val {
        Value::Integer(n) if n >= 1 => {
            if n <= 21 {
                let mut r = 1i64;
                for i in 2..n {
                    r = r.saturating_mul(i);
                }
                Ok(Value::Integer(r))
            } else {
                Ok(Value::Float(f64::gamma_approx(n as f64)))
            }
        }
        Value::Float(f) if f >= 1.0 => Ok(Value::Float(f64::gamma_approx(f))),
        _ => Err("gamma(n) expects n >= 1 (integer or float)".to_string()),
    }
}

trait GammaApprox {
    fn gamma_approx(self) -> f64;
}
impl GammaApprox for f64 {
    fn gamma_approx(self) -> f64 {
        if self <= 0.0 {
            return f64::NAN;
        }
        if self < 1.0 {
            return Self::gamma_approx(self + 1.0) / self;
        }
        let g = 7.0;
        let c: [f64; 9] = [
            0.99999999999980993,
            676.5203681218851,
            -1259.1392167224028,
            771.32342877765313,
            -176.61502916214059,
            12.507343278686905,
            -0.13857109526572012,
            9.9843695780195716e-6,
            1.5056327351493116e-7,
        ];
        let mut x = self;
        if x < 0.5 {
            return std::f64::consts::PI
                / ((std::f64::consts::PI * x).sin() * (1.0 - x).gamma_approx());
        }
        x -= 1.0;
        let mut t = c[0];
        for i in 1..9 {
            t += c[i] / (x + i as f64);
        }
        let t = t + g + 0.5;
        (x + g + 0.5).sqrt() * (x + g + 0.5).powf(x + 0.5) * (-(x + g + 0.5)).exp() * t
            * (2.0 * std::f64::consts::PI).sqrt()
    }
}

fn call_polyval(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("polyval(coeffs, x) expects exactly 2 arguments".to_string());
    }
    let coeffs_val = interpreter.eval_node(&args[0])?;
    let x_val = interpreter.eval_node(&args[1])?;
    let coeffs = match coeffs_val {
        Value::List(c) => c,
        _ => return Err("polyval(coeffs, x) expects list of coefficients (high to low power)".to_string()),
    };
    let x = match x_val {
        Value::Integer(i) => i as f64,
        Value::Float(f) => f,
        _ => return Err("polyval(coeffs, x) expects numeric x".to_string()),
    };
    let mut result = 0.0f64;
    for (i, c) in coeffs.iter().enumerate() {
        let co = match c {
            Value::Integer(n) => *n as f64,
            Value::Float(f) => *f,
            _ => return Err("polyval: coefficients must be numbers".to_string()),
        };
        let power = coeffs.len() - 1 - i;
        result += co * x.powi(power as i32);
    }
    Ok(if result.fract() == 0.0 && result.abs() <= i64::MAX as f64 {
        Value::Integer(result as i64)
    } else {
        Value::Float(result)
    })
}

fn mod_positive(a: i64, m: i64) -> i64 {
    let r = a % m;
    if r < 0 {
        r + m
    } else {
        r
    }
}

fn call_mod_add(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 3 {
        return Err("mod_add(a, b, m) expects exactly 3 arguments".to_string());
    }
    let (a, b, m) = three_ints(interpreter, args, "mod_add")?;
    if m <= 0 {
        return Err("mod_add: modulus m must be positive".to_string());
    }
    let a = mod_positive(a, m);
    let b = mod_positive(b, m);
    let r = ((a as i128) + (b as i128)) % (m as i128);
    Ok(Value::Integer(mod_positive(r as i64, m)))
}

fn call_mod_sub(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 3 {
        return Err("mod_sub(a, b, m) expects exactly 3 arguments".to_string());
    }
    let (a, b, m) = three_ints(interpreter, args, "mod_sub")?;
    if m <= 0 {
        return Err("mod_sub: modulus m must be positive".to_string());
    }
    let a = mod_positive(a, m);
    let b = mod_positive(b, m);
    let r = (a as i128 - b as i128) % (m as i128);
    Ok(Value::Integer(mod_positive(r as i64, m)))
}

fn call_mod_mul(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 3 {
        return Err("mod_mul(a, b, m) expects exactly 3 arguments".to_string());
    }
    let (a, b, m) = three_ints(interpreter, args, "mod_mul")?;
    if m <= 0 {
        return Err("mod_mul: modulus m must be positive".to_string());
    }
    let a = mod_positive(a, m);
    let b = mod_positive(b, m);
    let r = ((a as i128) * (b as i128)) % (m as i128);
    Ok(Value::Integer(mod_positive(r as i64, m)))
}

fn call_mod_pow(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 3 {
        return Err("mod_pow(base, exp, m) expects exactly 3 arguments".to_string());
    }
    let (base, exp, m) = three_ints(interpreter, args, "mod_pow")?;
    if m <= 0 {
        return Err("mod_pow: modulus m must be positive".to_string());
    }
    let base = mod_positive(base, m);
    let mut exp = exp;
    if exp < 0 {
        let inv = call_mod_inv(interpreter, &[args[0].clone(), args[2].clone()])?;
        let inv_i = match inv {
            Value::Integer(i) => i,
            _ => return Err("mod_pow: base not invertible mod m".to_string()),
        };
        exp = exp.saturating_neg();
        let mut result = 1i64;
        let mut b = inv_i;
        let mut e = exp;
        while e > 0 {
            if e % 2 == 1 {
                result = mod_positive((result as i128 * b as i128) as i64, m);
            }
            b = mod_positive((b as i128 * b as i128) as i64, m);
            e /= 2;
        }
        return Ok(Value::Integer(result));
    }
    let mut result = 1i64;
    let mut b = base;
    let mut e = exp;
    while e > 0 {
        if e % 2 == 1 {
            result = mod_positive((result as i128 * b as i128) as i64, m);
        }
        b = mod_positive((b as i128 * b as i128) as i64, m);
        e /= 2;
    }
    Ok(Value::Integer(result))
}

fn call_mod_inv(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("mod_inv(a, m) expects exactly 2 arguments".to_string());
    }
    let a_val = interpreter.eval_node(&args[0])?;
    let m_val = interpreter.eval_node(&args[1])?;
    let (a, m) = match (a_val, m_val) {
        (Value::Integer(a), Value::Integer(m)) => (a, m),
        _ => return Err("mod_inv(a, m) expects two integers".to_string()),
    };
    if m <= 0 {
        return Err("mod_inv: modulus m must be positive".to_string());
    }
    let (g, x, _) = egcd_inner(a, m);
    if g != 1 {
        return Err("mod_inv: a and m must be coprime".to_string());
    }
    Ok(Value::Integer(mod_positive(x, m)))
}

fn three_ints(
    interpreter: &mut Interpreter,
    args: &[AstNode],
    name: &str,
) -> Result<(i64, i64, i64), String> {
    let a = interpreter.eval_node(&args[0])?;
    let b = interpreter.eval_node(&args[1])?;
    let c = interpreter.eval_node(&args[2])?;
    match (a, b, c) {
        (Value::Integer(a), Value::Integer(b), Value::Integer(c)) => Ok((a, b, c)),
        _ => Err(format!("{}(a, b, m) expects integers", name)),
    }
}

fn egcd_inner(a: i64, b: i64) -> (i64, i64, i64) {
    let (mut a, mut b) = (a, b);
    let (mut x0, mut x1) = (1i64, 0i64);
    let (mut y0, mut y1) = (0i64, 1i64);
    while b != 0 {
        let q = a / b;
        (a, b) = (b, a - q * b);
        (x0, x1) = (x1, x0 - q * x1);
        (y0, y1) = (y1, y0 - q * y1);
    }
    (a, x0, y0)
}
