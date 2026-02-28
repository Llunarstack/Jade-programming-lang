//! Union-Find (Disjoint Set Union) builtins:
//!   uf_new, uf_find, uf_union, uf_connected

use crate::interpreter::{Interpreter, Value};
use crate::parser::AstNode;

pub(super) fn try_call(
    interpreter: &mut Interpreter,
    name: &str,
    args: &[AstNode],
) -> Result<Option<Value>, String> {
    let result = match name {
        "uf_new"      => Some(call_uf_new(interpreter, args)?),
        "uf_find"     => Some(call_uf_find(interpreter, args)?),
        "uf_union"    => Some(call_uf_union(interpreter, args)?),
        "uf_connected"=> Some(call_uf_connected(interpreter, args)?),
        _             => None,
    };
    Ok(result)
}

// ----------------------------------------------------------------------------

fn call_uf_new(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("uf_new expects exactly 1 argument: size".to_string());
    }

    let size_val = interpreter.eval_node(&args[0])?;
    let size = match size_val {
        Value::Integer(n) if n >= 0 => n as usize,
        _ => return Err("uf_new expects a non-negative integer size".to_string()),
    };

    let parent: Vec<usize> = (0..size).collect();
    let rank = vec![0; size];

    Ok(Value::UnionFind { parent, rank })
}

// Path compression (mutating version)
fn find_in_place(parent: &mut [usize], mut x: usize) -> usize {
    while parent[x] != x {
        // One-pass path halving + compression
        let next = parent[x];
        parent[x] = parent[next];
        x = next;
    }
    x
}

// Non-mutating find (used by uf_connected)
fn find(parent: &[usize], mut x: usize) -> usize {
    while parent[x] != x {
        x = parent[x];
    }
    x
}

fn call_uf_find(
    interpreter: &mut Interpreter,
    args: &[AstNode],
) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("uf_find(uf, x) expects exactly 2 arguments".to_string());
    }

    let uf_val  = interpreter.eval_node(&args[0])?;
    let idx_val = interpreter.eval_node(&args[1])?;

    let (mut parent, rank, x) = match (uf_val, idx_val) {
        (
            Value::UnionFind { parent, rank },
            Value::Integer(i),
        ) if i >= 0 => (parent, rank, i as usize),
        _ => return Err("uf_find expects (UnionFind, non-negative integer)".to_string()),
    };

    if x >= parent.len() {
        return Err(format!("uf_find: index out of bounds ({} >= {})", x, parent.len()));
    }

    let root = find_in_place(&mut parent, x);

    Ok(Value::Tuple(vec![
        Value::UnionFind { parent, rank },
        Value::Integer(root as i64),
    ]))
}

fn call_uf_union(
    interpreter: &mut Interpreter,
    args: &[AstNode],
) -> Result<Value, String> {
    if args.len() != 3 {
        return Err("uf_union(uf, a, b) expects exactly 3 arguments".to_string());
    }

    let uf_val = interpreter.eval_node(&args[0])?;
    let a_val  = interpreter.eval_node(&args[1])?;
    let b_val  = interpreter.eval_node(&args[2])?;

    let (mut parent, mut rank) = match uf_val {
        Value::UnionFind { parent, rank } => (parent, rank),
        _ => return Err("uf_union expects UnionFind as first argument".to_string()),
    };

    let a = match a_val {
        Value::Integer(n) if n >= 0 => n as usize,
        _ => return Err("uf_union: a must be non-negative integer".to_string()),
    };
    let b = match b_val {
        Value::Integer(n) if n >= 0 => n as usize,
        _ => return Err("uf_union: b must be non-negative integer".to_string()),
    };

    if a >= parent.len() || b >= parent.len() {
        return Err(format!(
            "uf_union: index out of bounds (size = {}, a = {}, b = {})",
            parent.len(), a, b
        ));
    }

    let ra = find_in_place(&mut parent, a);
    let rb = find_in_place(&mut parent, b);

    if ra == rb {
        return Ok(Value::UnionFind { parent, rank });
    }

    // Union by rank
    if rank[ra] < rank[rb] {
        parent[ra] = rb;
    } else if rank[ra] > rank[rb] {
        parent[rb] = ra;
    } else {
        parent[rb] = ra;
        rank[ra] += 1;
    }

    Ok(Value::UnionFind { parent, rank })
}

fn call_uf_connected(
    interpreter: &mut Interpreter,
    args: &[AstNode],
) -> Result<Value, String> {
    if args.len() != 3 {
        return Err("uf_connected(uf, a, b) expects exactly 3 arguments".to_string());
    }

    let uf_val = interpreter.eval_node(&args[0])?;
    let a_val  = interpreter.eval_node(&args[1])?;
    let b_val  = interpreter.eval_node(&args[2])?;

    let (parent, a, b) = match (uf_val, a_val, b_val) {
        (
            Value::UnionFind { parent, .. },
            Value::Integer(ai),
            Value::Integer(bi),
        ) if ai >= 0 && bi >= 0 => (parent, ai as usize, bi as usize),
        _ => return Err("uf_connected expects (UnionFind, non-neg int, non-neg int)".to_string()),
    };

    if a >= parent.len() || b >= parent.len() {
        return Err(format!(
            "uf_connected: index out of bounds (size = {}, a = {}, b = {})",
            parent.len(), a, b
        ));
    }

    let ra = find(&parent, a);
    let rb = find(&parent, b);

    Ok(Value::Boolean(ra == rb))
}