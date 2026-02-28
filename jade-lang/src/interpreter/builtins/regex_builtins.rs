//! Regex builtins: regex(pattern), regex(pattern, flags), and pre-built regex helpers.
//! Requires the "regex" feature.

#![cfg(feature = "regex")]

use crate::interpreter::{Interpreter, Value};
use crate::interpreter::value::{CompiledRegex, RegexMatch};
use crate::parser::AstNode;
use std::collections::HashMap;

fn pattern_with_flags(pattern: &str, flags: &str) -> Result<String, String> {
    let mut out = pattern.to_string();
    if flags.contains('i') {
        out = format!("(?i){}", out);
    }
    if flags.contains('m') {
        out = format!("(?m){}", out);
    }
    if flags.contains('s') {
        out = format!("(?s){}", out);
    }
    Ok(out)
}

fn compile_regex(pattern: &str, flags: Option<&str>) -> Result<Value, String> {
    let effective = match flags {
        Some(f) => pattern_with_flags(pattern, f)?,
        None => pattern.to_string(),
    };
    let re = regex::Regex::new(&effective).map_err(|e| format!("Invalid regex: {}", e))?;
    Ok(Value::Regex(Box::new(CompiledRegex {
        pattern: effective,
        inner: re,
    })))
}

pub(super) fn try_call(
    interpreter: &mut Interpreter,
    name: &str,
    args: &[AstNode],
) -> Result<Option<Value>, String> {
    let v = match name {
        "regex" => Some(call_regex(interpreter, args)?),
        "regex_email" => Some(call_regex_email(interpreter, args)?),
        "regex_url" => Some(call_regex_url(interpreter, args)?),
        "regex_ipv4" => Some(call_regex_ipv4(interpreter, args)?),
        "regex_uuid" => Some(call_regex_uuid(interpreter, args)?),
        "regex_hex_color" => Some(call_regex_hex_color(interpreter, args)?),
        "regex_semver" => Some(call_regex_semver(interpreter, args)?),
        "regex_replace" => Some(call_regex_replace(interpreter, args)?),
        "regex_split" => Some(call_regex_split(interpreter, args)?),
        _ => None,
    };
    Ok(v)
}

fn call_regex_replace(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 3 {
        return Err("regex_replace(string, regex, replacement) expects 3 arguments".to_string());
    }
    let s = get_string_arg(interpreter, args, 0)?;
    let repl = get_string_arg(interpreter, args, 2)?;
    let re_val = interpreter.eval_node(&args[1])?;
    let re = match re_val {
        Value::Regex(r) => r,
        _ => return Err("regex_replace second argument must be a regex".to_string()),
    };
    Ok(Value::String(regex_replace(&s, re.as_ref(), &repl)))
}

fn call_regex_split(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("regex_split(string, regex) expects 2 arguments".to_string());
    }
    let s = get_string_arg(interpreter, args, 0)?;
    let re_val = interpreter.eval_node(&args[1])?;
    let re = match re_val {
        Value::Regex(r) => r,
        _ => return Err("regex_split second argument must be a regex".to_string()),
    };
    let parts = regex_split(&s, re.as_ref());
    Ok(Value::List(parts.into_iter().map(Value::String).collect()))
}

fn get_string_arg(interpreter: &mut Interpreter, args: &[AstNode], idx: usize) -> Result<String, String> {
    match interpreter.eval_node(&args[idx])? {
        Value::String(s) => Ok(s),
        _ => Err("expected string argument".to_string()),
    }
}

fn call_regex(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.is_empty() || args.len() > 2 {
        return Err("regex(pattern) or regex(pattern, flags) expects 1 or 2 arguments".to_string());
    }
    let pattern = get_string_arg(interpreter, args, 0)?;
    let flags_str = if args.len() == 2 {
        Some(get_string_arg(interpreter, args, 1)?)
    } else {
        None
    };
    compile_regex(&pattern, flags_str.as_deref())
}

fn call_regex_email(_interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if !args.is_empty() {
        return Err("regex_email() expects no arguments".to_string());
    }
    let pat = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$";
    compile_regex(pat, None)
}

fn call_regex_url(_interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if !args.is_empty() {
        return Err("regex_url() expects no arguments".to_string());
    }
    let pat = r"https?://[^\s]+";
    compile_regex(pat, None)
}

fn call_regex_ipv4(_interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if !args.is_empty() {
        return Err("regex_ipv4() expects no arguments".to_string());
    }
    let pat = r"^(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$";
    compile_regex(pat, None)
}

fn call_regex_uuid(_interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if !args.is_empty() {
        return Err("regex_uuid() expects no arguments".to_string());
    }
    let pat = r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$";
    compile_regex(pat, None)
}

fn call_regex_hex_color(_interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if !args.is_empty() {
        return Err("regex_hex_color() expects no arguments".to_string());
    }
    let pat = r"^#([0-9A-Fa-f]{3}|[0-9A-Fa-f]{6}|[0-9A-Fa-f]{8})$";
    compile_regex(pat, None)
}

fn call_regex_semver(_interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if !args.is_empty() {
        return Err("regex_semver() expects no arguments".to_string());
    }
    let pat = r"^\d+\.\d+\.\d+(-[0-9A-Za-z.-]+)?(\+[0-9A-Za-z.-]+)?$";
    compile_regex(pat, None)
}

/// Run regex operations from call.rs (string contains/matches/find/find_all/replace/split).
/// These are called with (string_val, regex_val) or (string_val, regex_val, replacement).
#[cfg(feature = "regex")]
pub fn regex_contains(s: &str, re: &crate::interpreter::value::CompiledRegex) -> bool {
    re.inner.is_match(s)
}

#[cfg(feature = "regex")]
pub fn regex_matches(s: &str, re: &crate::interpreter::value::CompiledRegex) -> bool {
    re.inner.find(s).map(|m| m.start() == 0 && m.end() == s.len()).unwrap_or(false)
}

#[cfg(feature = "regex")]
pub fn regex_find(s: &str, re: &crate::interpreter::value::CompiledRegex) -> Option<Value> {
    let cap = re.inner.captures(s)?;
    let mat = cap.get(0)?;
    let groups: Vec<String> = cap.iter().skip(1).map(|m| m.map(|x| x.as_str().to_string()).unwrap_or_default()).collect();
    let named: HashMap<String, String> = re.inner.capture_names()
        .flatten()
        .filter_map(|name| {
            cap.name(name).map(|m| (name.to_string(), m.as_str().to_string()))
        })
        .collect();
    Some(Value::RegexMatch(Box::new(RegexMatch {
        text: mat.as_str().to_string(),
        start: mat.start(),
        end: mat.end(),
        groups,
        named,
    })))
}

#[cfg(feature = "regex")]
pub fn regex_find_all(s: &str, re: &crate::interpreter::value::CompiledRegex) -> Vec<Value> {
    let mut out = Vec::new();
    for cap in re.inner.captures_iter(s) {
        let mat = cap.get(0).unwrap();
        let groups: Vec<String> = cap.iter().skip(1).map(|m| m.map(|x| x.as_str().to_string()).unwrap_or_default()).collect();
        let named: HashMap<String, String> = re.inner.capture_names()
            .flatten()
            .filter_map(|name| cap.name(name).map(|m| (name.to_string(), m.as_str().to_string())))
            .collect();
        out.push(Value::RegexMatch(Box::new(RegexMatch {
            text: mat.as_str().to_string(),
            start: mat.start(),
            end: mat.end(),
            groups,
            named,
        })));
    }
    out
}

#[cfg(feature = "regex")]
pub fn regex_replace(s: &str, re: &crate::interpreter::value::CompiledRegex, replacement: &str) -> String {
    re.inner.replace_all(s, replacement).to_string()
}

#[cfg(feature = "regex")]
pub fn regex_split(s: &str, re: &crate::interpreter::value::CompiledRegex) -> Vec<String> {
    re.inner.split(s).map(|s| s.to_string()).collect()
}
