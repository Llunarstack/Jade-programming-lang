//! Trie (prefix tree) builtins: trie_new, trie_insert, trie_contains, trie_prefix_search.

use crate::interpreter::{Interpreter, Value, TrieNode};
use crate::parser::AstNode;

pub(super) fn try_call(
    interpreter: &mut Interpreter,
    name: &str,
    args: &[AstNode],
) -> Result<Option<Value>, String> {
    let v = match name {
        "trie_new" => Some(call_trie_new(interpreter, args)?),
        "trie_insert" => Some(call_trie_insert(interpreter, args)?),
        "trie_contains" => Some(call_trie_contains(interpreter, args)?),
        "trie_prefix_search" => Some(call_trie_prefix_search(interpreter, args)?),
        _ => None,
    };
    Ok(v)
}

fn call_trie_new(_interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if !args.is_empty() {
        return Err("trie_new() expects no arguments".to_string());
    }
    Ok(Value::Trie(Box::new(TrieNode::default())))
}

fn trie_insert_node(node: &mut TrieNode, s: &str) {
    let mut current = node;
    for ch in s.chars() {
        current = current.children.entry(ch).or_insert_with(|| Box::new(TrieNode::default()));
    }
    current.is_end = true;
}

fn call_trie_insert(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("trie_insert(trie, s) expects exactly 2 arguments".to_string());
    }
    let trie_val = interpreter.eval_node(&args[0])?;
    let s_val = interpreter.eval_node(&args[1])?;
    let (mut node, s) = match (trie_val, s_val) {
        (Value::Trie(node), Value::String(s)) => (*node, s),
        _ => return Err("trie_insert(trie, s) expects Trie and string".to_string()),
    };
    trie_insert_node(&mut node, &s);
    Ok(Value::Trie(Box::new(node)))
}

fn trie_contains_node(node: &TrieNode, s: &str) -> bool {
    let mut current = node;
    for ch in s.chars() {
        match current.children.get(&ch) {
            Some(child) => current = child,
            None => return false,
        }
    }
    current.is_end
}

fn call_trie_contains(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("trie_contains(trie, s) expects exactly 2 arguments".to_string());
    }
    let trie_val = interpreter.eval_node(&args[0])?;
    let s_val = interpreter.eval_node(&args[1])?;
    let (node, s) = match (trie_val, s_val) {
        (Value::Trie(node), Value::String(s)) => (node, s),
        _ => return Err("trie_contains(trie, s) expects Trie and string".to_string()),
    };
    Ok(Value::Boolean(trie_contains_node(&node, &s)))
}

fn trie_prefix_collect(node: &TrieNode, prefix: &str, acc: &mut Vec<String>) {
    if node.is_end {
        acc.push(prefix.to_string());
    }
    for (ch, child) in &node.children {
        let mut p = prefix.to_string();
        p.push(*ch);
        trie_prefix_collect(child, &p, acc);
    }
}

fn call_trie_prefix_search(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("trie_prefix_search(trie, prefix) expects exactly 2 arguments".to_string());
    }
    let trie_val = interpreter.eval_node(&args[0])?;
    let prefix_val = interpreter.eval_node(&args[1])?;
    let (node, prefix) = match (trie_val, prefix_val) {
        (Value::Trie(node), Value::String(s)) => (node, s),
        _ => return Err("trie_prefix_search(trie, prefix) expects Trie and string".to_string()),
    };
    let mut current = node.as_ref();
    for ch in prefix.chars() {
        match current.children.get(&ch) {
            Some(child) => current = child,
            None => return Ok(Value::List(Vec::new())),
        }
    }
    let mut acc = Vec::new();
    trie_prefix_collect(current, &prefix, &mut acc);
    Ok(Value::List(
        acc.into_iter().map(Value::String).collect(),
    ))
}
