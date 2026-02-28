//! Algorithm-helper builtins: swap, rotate_left/right, count_if, find_index, binary_search,
//! sliding_window, partition_range, two_pointers_sum, flood_fill,
//! lower_bound, upper_bound, prefix_sum, kadane, reverse_range, merge_sorted.

use crate::interpreter::{Interpreter, Value};
use crate::parser::AstNode;
use std::collections::VecDeque;

pub(super) fn try_call(
    interpreter: &mut Interpreter,
    name: &str,
    args: &[AstNode],
) -> Result<Option<Value>, String> {
    let v = match name {
        "swap" => Some(call_swap(interpreter, args)?),
        "rotate_left" => Some(call_rotate_left(interpreter, args)?),
        "rotate_right" => Some(call_rotate_right(interpreter, args)?),
        "count_if" => Some(call_count_if(interpreter, args)?),
        "find_index" => Some(call_find_index(interpreter, args)?),
        "binary_search" => Some(call_binary_search(interpreter, args)?),
        "sliding_window" => Some(call_sliding_window(interpreter, args)?),
        "partition_range" => Some(call_partition_range(interpreter, args)?),
        "two_pointers_sum" => Some(call_two_pointers_sum(interpreter, args)?),
        "flood_fill" => Some(call_flood_fill(interpreter, args)?),
        "lower_bound" => Some(call_lower_bound(interpreter, args)?),
        "upper_bound" => Some(call_upper_bound(interpreter, args)?),
        "prefix_sum" => Some(call_prefix_sum(interpreter, args)?),
        "kadane" => Some(call_kadane(interpreter, args)?),
        "reverse_range" => Some(call_reverse_range(interpreter, args)?),
        "merge_sorted" => Some(call_merge_sorted(interpreter, args)?),
        "gcd_list" => Some(call_gcd_list(interpreter, args)?),
        "lcm_list" => Some(call_lcm_list(interpreter, args)?),
        _ => None,
    };
    Ok(v)
}

fn call_swap(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 3 {
        return Err("swap() expects exactly 3 arguments: swap(list, i, j)".to_string());
    }
    let list_val = interpreter.eval_node(&args[0])?;
    let i_val = interpreter.eval_node(&args[1])?;
    let j_val = interpreter.eval_node(&args[2])?;
    match (list_val, i_val, j_val) {
        (Value::List(mut list), Value::Integer(i), Value::Integer(j)) => {
            let len = list.len() as i64;
            if i < 0 || i >= len || j < 0 || j >= len {
                return Err("swap() indices out of bounds".to_string());
            }
            list.swap(i as usize, j as usize);
            Ok(Value::List(list))
        }
        _ => Err("swap() expects (list, int, int)".to_string()),
    }
}

fn call_rotate_left(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("rotate_left() expects exactly 2 arguments: rotate_left(list, k)".to_string());
    }
    let list_val = interpreter.eval_node(&args[0])?;
    let k_val = interpreter.eval_node(&args[1])?;
    match (list_val, k_val) {
        (Value::List(list), Value::Integer(k)) => {
            if list.is_empty() {
                return Ok(Value::List(list));
            }
            let len = list.len();
            let k = ((k % len as i64) + len as i64) as usize % len;
            let mut result = list[k..].to_vec();
            result.extend_from_slice(&list[..k]);
            Ok(Value::List(result))
        }
        _ => Err("rotate_left() expects (list, int)".to_string()),
    }
}

fn call_rotate_right(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("rotate_right() expects exactly 2 arguments: rotate_right(list, k)".to_string());
    }
    let list_val = interpreter.eval_node(&args[0])?;
    let k_val = interpreter.eval_node(&args[1])?;
    match (list_val, k_val) {
        (Value::List(list), Value::Integer(k)) => {
            if list.is_empty() {
                return Ok(Value::List(list));
            }
            let len = list.len();
            let k = ((k % len as i64) + len as i64) as usize % len;
            let split_point = len - k;
            let mut result = list[split_point..].to_vec();
            result.extend_from_slice(&list[..split_point]);
            Ok(Value::List(result))
        }
        _ => Err("rotate_right() expects (list, int)".to_string()),
    }
}

fn call_count_if(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("count_if() expects exactly 2 arguments: count_if(list, predicate)".to_string());
    }
    let list_val = interpreter.eval_node(&args[0])?;
    let pred_val = interpreter.eval_node(&args[1])?;
    match list_val {
        Value::List(list) => {
            let mut count = 0;
            for item in list {
                let result = interpreter.call_value_with_args(pred_val.clone(), &[item], None)?;
                match result {
                    Value::Boolean(true) => count += 1,
                    Value::Boolean(false) => {}
                    _ => return Err("count_if() predicate must return boolean".to_string()),
                }
            }
            Ok(Value::Integer(count))
        }
        _ => Err("count_if() expects a list as first argument".to_string()),
    }
}

fn call_find_index(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("find_index() expects exactly 2 arguments: find_index(list, predicate)".to_string());
    }
    let list_val = interpreter.eval_node(&args[0])?;
    let pred_val = interpreter.eval_node(&args[1])?;
    match list_val {
        Value::List(list) => {
            for (i, item) in list.iter().enumerate() {
                let result = interpreter.call_value_with_args(
                    pred_val.clone(),
                    std::slice::from_ref(item),
                    None,
                )?;
                match result {
                    Value::Boolean(true) => return Ok(Value::Integer(i as i64)),
                    Value::Boolean(false) => {}
                    _ => return Err("find_index() predicate must return boolean".to_string()),
                }
            }
            Ok(Value::Integer(-1))
        }
        _ => Err("find_index() expects a list as first argument".to_string()),
    }
}

fn call_binary_search(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("binary_search() expects exactly 2 arguments".to_string());
    }
    let list_val = interpreter.eval_node(&args[0])?;
    let target_val = interpreter.eval_node(&args[1])?;
    match list_val {
        Value::List(list) => {
            if let Value::Integer(target) = target_val {
                let mut left = 0;
                let mut right = list.len();
                while left < right {
                    let mid = left + (right - left) / 2;
                    match &list[mid] {
                        Value::Integer(mid_val) => {
                            if *mid_val == target {
                                return Ok(Value::Integer(mid as i64));
                            } else if *mid_val < target {
                                left = mid + 1;
                            } else {
                                right = mid;
                            }
                        }
                        _ => {
                            return Err("binary_search() requires a sorted list of integers".to_string());
                        }
                    }
                }
                Ok(Value::Integer(-1))
            } else {
                Err("binary_search() target must be an integer".to_string())
            }
        }
        _ => Err("binary_search() can only be called on lists".to_string()),
    }
}

fn call_sliding_window(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("sliding_window() expects exactly 2 arguments".to_string());
    }
    let list_val = interpreter.eval_node(&args[0])?;
    let size_val = interpreter.eval_node(&args[1])?;
    match (list_val, size_val) {
        (Value::List(list), Value::Integer(size)) => {
            if size <= 0 {
                return Err("sliding_window() size must be positive".to_string());
            }
            let size = size as usize;
            if list.len() < size {
                return Ok(Value::List(Vec::new()));
            }
            let mut result = Vec::new();
            for i in 0..=(list.len() - size) {
                let window: Vec<Value> = list[i..i + size].to_vec();
                result.push(Value::List(window));
            }
            Ok(Value::List(result))
        }
        _ => Err("sliding_window() expects a list and an integer".to_string()),
    }
}

/// Compare two values for <= (for partition). Supports int, float, string.
fn value_leq(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Integer(x), Value::Integer(y)) => x <= y,
        (Value::Float(x), Value::Float(y)) => x <= y,
        (Value::Integer(x), Value::Float(y)) => (*x as f64) <= *y,
        (Value::Float(x), Value::Integer(y)) => *x <= (*y as f64),
        (Value::String(x), Value::String(y)) => x <= y,
        _ => false,
    }
}

fn call_partition_range(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 3 {
        return Err("partition_range(list, low, high) expects exactly 3 arguments".to_string());
    }
    let list_val = interpreter.eval_node(&args[0])?;
    let low_val = interpreter.eval_node(&args[1])?;
    let high_val = interpreter.eval_node(&args[2])?;
    let (mut list, low, high) = match (list_val, low_val, high_val) {
        (Value::List(l), Value::Integer(lo), Value::Integer(hi)) => (l, lo as usize, hi as usize),
        _ => return Err("partition_range expects (list, int, int)".to_string()),
    };
    if list.is_empty() || low >= list.len() || high >= list.len() || low > high {
        return Err("partition_range: invalid low/high indices".to_string());
    }
    let pivot = list[high].clone();
    let mut i = low;
    for j in low..high {
        if value_leq(&list[j], &pivot) {
            list.swap(i, j);
            i += 1;
        }
    }
    list.swap(i, high);
    Ok(Value::Tuple(vec![
        Value::List(list),
        Value::Integer(i as i64),
    ]))
}

fn call_two_pointers_sum(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("two_pointers_sum(arr, target) expects exactly 2 arguments".to_string());
    }
    let list_val = interpreter.eval_node(&args[0])?;
    let target_val = interpreter.eval_node(&args[1])?;
    let (list, target) = match (list_val, target_val) {
        (Value::List(l), Value::Integer(t)) => (l, t),
        _ => return Err("two_pointers_sum expects (list of ints, int target)".to_string()),
    };
    let mut left = 0i64;
    let mut right = list.len() as i64 - 1;
    while left < right {
        let sum = match (&list[left as usize], &list[right as usize]) {
            (Value::Integer(a), Value::Integer(b)) => a + b,
            _ => return Err("two_pointers_sum requires list of integers".to_string()),
        };
        match sum.cmp(&target) {
            std::cmp::Ordering::Equal => {
                return Ok(Value::Tuple(vec![
                    Value::Integer(left),
                    Value::Integer(right),
                ]))
            }
            std::cmp::Ordering::Less => left += 1,
            std::cmp::Ordering::Greater => right -= 1,
        }
    }
    Ok(Value::Tuple(vec![Value::Integer(-1), Value::Integer(-1)]))
}

fn call_flood_fill(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 4 {
        return Err("flood_fill(grid, start_row, start_col, predicate) expects 4 arguments".to_string());
    }
    let grid_val = interpreter.eval_node(&args[0])?;
    let row_val = interpreter.eval_node(&args[1])?;
    let col_val = interpreter.eval_node(&args[2])?;
    let pred_val = interpreter.eval_node(&args[3])?;
    let (grid, start_r, start_c) = match (grid_val, row_val, col_val) {
        (Value::Grid(g), Value::Integer(r), Value::Integer(c)) => (g, r as usize, c as usize),
        _ => return Err("flood_fill expects (grid, int, int)".to_string()),
    };
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };
    if start_r >= rows || start_c >= cols {
        return Err("flood_fill: start position out of bounds".to_string());
    }
    let mut visited = std::collections::HashSet::new();
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back((start_r, start_c));
    visited.insert((start_r, start_c));
    let mut result = vec![Value::Tuple(vec![
        Value::Integer(start_r as i64),
        Value::Integer(start_c as i64),
    ])];
    let neighbors: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    while let Some((r, c)) = queue.pop_front() {
        for (dr, dc) in neighbors.iter() {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr < 0 || nc < 0 {
                continue;
            }
            let nr = nr as usize;
            let nc = nc as usize;
            if nr >= rows || nc >= cols || visited.contains(&(nr, nc)) {
                continue;
            }
            let cell = &grid[nr][nc];
            let ok = interpreter.call_value_with_args(pred_val.clone(), &[cell.clone()], None)?;
            match ok {
                Value::Boolean(true) => {
                    visited.insert((nr, nc));
                    queue.push_back((nr, nc));
                    result.push(Value::Tuple(vec![
                        Value::Integer(nr as i64),
                        Value::Integer(nc as i64),
                    ]));
                }
                _ => {}
            }
        }
    }
    Ok(Value::List(result))
}

/// Compare two values for ordering. Returns (a <= b, a < b) style for bounds.
fn value_cmp(a: &Value, b: &Value) -> Option<std::cmp::Ordering> {
    match (a, b) {
        (Value::Integer(x), Value::Integer(y)) => Some(x.cmp(y)),
        (Value::Float(x), Value::Float(y)) => Some(x.partial_cmp(y)?),
        (Value::Integer(x), Value::Float(y)) => (*x as f64).partial_cmp(y),
        (Value::Float(x), Value::Integer(y)) => x.partial_cmp(&(*y as f64)),
        (Value::String(x), Value::String(y)) => Some(x.cmp(y)),
        _ => None,
    }
}

fn call_lower_bound(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("lower_bound(list, value) expects exactly 2 arguments".to_string());
    }
    let list_val = interpreter.eval_node(&args[0])?;
    let target = interpreter.eval_node(&args[1])?;
    let list = match &list_val {
        Value::List(l) => l,
        _ => return Err("lower_bound() expects a list".to_string()),
    };
    let mut left = 0;
    let mut right = list.len();
    while left < right {
        let mid = left + (right - left) / 2;
        match value_cmp(&list[mid], &target) {
            Some(std::cmp::Ordering::Less) => left = mid + 1,
            _ => right = mid,
        }
    }
    Ok(Value::Integer(left as i64))
}

fn call_upper_bound(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("upper_bound(list, value) expects exactly 2 arguments".to_string());
    }
    let list_val = interpreter.eval_node(&args[0])?;
    let target = interpreter.eval_node(&args[1])?;
    let list = match &list_val {
        Value::List(l) => l,
        _ => return Err("upper_bound() expects a list".to_string()),
    };
    let mut left = 0;
    let mut right = list.len();
    while left < right {
        let mid = left + (right - left) / 2;
        match value_cmp(&target, &list[mid]) {
            Some(std::cmp::Ordering::Less) => right = mid,
            _ => left = mid + 1,
        }
    }
    Ok(Value::Integer(left as i64))
}

fn call_prefix_sum(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("prefix_sum(list) expects exactly 1 argument".to_string());
    }
    let list_val = interpreter.eval_node(&args[0])?;
    let list = match &list_val {
        Value::List(l) => l,
        _ => return Err("prefix_sum() expects a list of numbers".to_string()),
    };
    let mut out = Vec::with_capacity(list.len());
    let mut sum = 0i64;
    for v in list {
        match v {
            Value::Integer(x) => {
                sum = sum.saturating_add(*x);
                out.push(Value::Integer(sum));
            }
            _ => return Err("prefix_sum() requires list of integers".to_string()),
        }
    }
    Ok(Value::List(out))
}

fn call_kadane(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("kadane(list) expects exactly 1 argument".to_string());
    }
    let list_val = interpreter.eval_node(&args[0])?;
    let list = match &list_val {
        Value::List(l) => l,
        _ => return Err("kadane() expects a list of numbers".to_string()),
    };
    let mut best = i64::MIN;
    let mut cur = 0i64;
    for v in list {
        let x = match v {
            Value::Integer(n) => *n,
            _ => return Err("kadane() requires list of integers".to_string()),
        };
        cur = cur.saturating_add(x).max(x);
        best = best.max(cur);
    }
    Ok(Value::Integer(if list.is_empty() { 0 } else { best }))
}

fn call_reverse_range(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 3 {
        return Err("reverse_range(list, start, end) expects exactly 3 arguments".to_string());
    }
    let list_val = interpreter.eval_node(&args[0])?;
    let start_val = interpreter.eval_node(&args[1])?;
    let end_val = interpreter.eval_node(&args[2])?;
    let (mut list, start, end) = match (list_val, start_val, end_val) {
        (Value::List(l), Value::Integer(s), Value::Integer(e)) => (l, s as usize, e as usize),
        _ => return Err("reverse_range expects (list, int, int)".to_string()),
    };
    let len = list.len();
    if start > end || end > len {
        return Err("reverse_range: invalid start/end".to_string());
    }
    list[start..=end].reverse();
    Ok(Value::List(list))
}

fn call_merge_sorted(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("merge_sorted(a, b) expects exactly 2 arguments".to_string());
    }
    let a_val = interpreter.eval_node(&args[0])?;
    let b_val = interpreter.eval_node(&args[1])?;
    let (a, b) = match (&a_val, &b_val) {
        (Value::List(x), Value::List(y)) => (x.clone(), y.clone()),
        _ => return Err("merge_sorted() expects two lists".to_string()),
    };
    let mut out = Vec::with_capacity(a.len() + b.len());
    let mut i = 0;
    let mut j = 0;
    while i < a.len() && j < b.len() {
        match value_cmp(&a[i], &b[j]) {
            Some(std::cmp::Ordering::Less) | Some(std::cmp::Ordering::Equal) => {
                out.push(a[i].clone());
                i += 1;
            }
            _ => {
                out.push(b[j].clone());
                j += 1;
            }
        }
    }
    out.extend(a[i..].iter().cloned());
    out.extend(b[j..].iter().cloned());
    Ok(Value::List(out))
}

fn gcd(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm_two(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 {
        return 0;
    }
    let g = gcd(a, b);
    (a.abs() / g).saturating_mul(b.abs())
}

fn call_gcd_list(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("gcd_list(list) expects exactly 1 argument".to_string());
    }
    let list_val = interpreter.eval_node(&args[0])?;
    let list = match &list_val {
        Value::List(l) => l,
        _ => return Err("gcd_list() expects a list of integers".to_string()),
    };
    let mut acc = 0i64;
    for v in list {
        let n = match v {
            Value::Integer(x) => *x,
            _ => return Err("gcd_list() requires list of integers".to_string()),
        };
        acc = gcd(acc, n);
    }
    Ok(Value::Integer(acc))
}

fn call_lcm_list(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("lcm_list(list) expects exactly 1 argument".to_string());
    }
    let list_val = interpreter.eval_node(&args[0])?;
    let list = match &list_val {
        Value::List(l) => l,
        _ => return Err("lcm_list() expects a list of integers".to_string()),
    };
    let mut acc = 1i64;
    for v in list {
        let n = match v {
            Value::Integer(x) => *x,
            _ => return Err("lcm_list() requires list of integers".to_string()),
        };
        acc = lcm_two(acc, n);
    }
    Ok(Value::Integer(acc))
}
