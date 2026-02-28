//! Data structures & algorithms builtins: graph (bfs, dfs, add_node, add_edge, get_neighbors,
//! graph_nodes, graph_edges, topological_sort, dijkstra), deque, priority queue.

use crate::interpreter::{Interpreter, Value};
use crate::parser::AstNode;
use std::collections::{HashMap, VecDeque};

pub(super) fn try_call(
    interpreter: &mut Interpreter,
    name: &str,
    args: &[AstNode],
) -> Result<Option<Value>, String> {
    let v = match name {
        "bfs" => Some(call_bfs(interpreter, args)?),
        "dfs" => Some(call_dfs(interpreter, args)?),
        "add_node" => Some(call_add_node(interpreter, args)?),
        "add_edge" => Some(call_add_edge(interpreter, args)?),
        "get_neighbors" => Some(call_get_neighbors(interpreter, args)?),
        "graph_nodes" => Some(call_graph_nodes(interpreter, args)?),
        "graph_edges" => Some(call_graph_edges(interpreter, args)?),
        "topological_sort" => Some(call_topological_sort(interpreter, args)?),
        "dijkstra" => Some(call_dijkstra(interpreter, args)?),
        "push_front" => Some(call_push_front(interpreter, args)?),
        "push_back" => Some(call_push_back(interpreter, args)?),
        "pop_front" => Some(call_pop_front(interpreter, args)?),
        "pop_back" => Some(call_pop_back(interpreter, args)?),
        "peek_front" => Some(call_peek_front(interpreter, args)?),
        "peek_back" => Some(call_peek_back(interpreter, args)?),
        "pq_push" => Some(call_pq_push(interpreter, args)?),
        "pq_pop" => Some(call_pq_pop(interpreter, args)?),
        "pq_peek" => Some(call_pq_peek(interpreter, args)?),
        _ => None,
    };
    Ok(v)
}

fn call_bfs(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() < 2 || args.len() > 3 {
        return Err("bfs() expects 2-3 arguments: graph, start, [goal]".to_string());
    }
    let graph_val = interpreter.eval_node(&args[0])?;
    let start_val = interpreter.eval_node(&args[1])?;
    let goal_val = if args.len() == 3 {
        Some(interpreter.eval_node(&args[2])?)
    } else {
        None
    };
    match (graph_val, start_val) {
        (Value::Graph(graph), Value::String(start)) => {
            let goal = goal_val.and_then(|v| {
                if let Value::String(s) = v {
                    Some(s)
                } else {
                    None
                }
            });
            let path = interpreter.bfs_search(&graph, &start, goal.as_deref())?;
            Ok(Value::List(path.into_iter().map(Value::String).collect()))
        }
        _ => Err("bfs() expects graph and string start node".to_string()),
    }
}

fn call_dfs(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() < 2 || args.len() > 3 {
        return Err("dfs() expects 2-3 arguments: graph, start, [goal]".to_string());
    }
    let graph_val = interpreter.eval_node(&args[0])?;
    let start_val = interpreter.eval_node(&args[1])?;
    let goal_val = if args.len() == 3 {
        Some(interpreter.eval_node(&args[2])?)
    } else {
        None
    };
    match (graph_val, start_val) {
        (Value::Graph(graph), Value::String(start)) => {
            let goal = goal_val.and_then(|v| {
                if let Value::String(s) = v {
                    Some(s)
                } else {
                    None
                }
            });
            let path = interpreter.dfs_search(&graph, &start, goal.as_deref())?;
            Ok(Value::List(path.into_iter().map(Value::String).collect()))
        }
        _ => Err("dfs() expects graph and string start node".to_string()),
    }
}

fn call_add_node(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("add_node() expects exactly 2 arguments: add_node(graph, node_name)".to_string());
    }
    let graph_val = interpreter.eval_node(&args[0])?;
    let node_val = interpreter.eval_node(&args[1])?;
    let node_name = match node_val {
        Value::String(s) => s,
        _ => return Err("Node name must be a string".to_string()),
    };
    match graph_val {
        Value::Graph(mut graph) => {
            graph.entry(node_name).or_insert_with(Vec::new);
            Ok(Value::Graph(graph))
        }
        _ => Err("add_node() can only be called on graphs".to_string()),
    }
}

fn call_add_edge(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() < 3 || args.len() > 4 {
        return Err("add_edge() expects 3-4 arguments: add_edge(graph, from, to, [weight])".to_string());
    }
    let graph_val = interpreter.eval_node(&args[0])?;
    let from_val = interpreter.eval_node(&args[1])?;
    let to_val = interpreter.eval_node(&args[2])?;
    let weight_val = if args.len() == 4 {
        interpreter.eval_node(&args[3])?
    } else {
        Value::Float(1.0)
    };
    let from = match from_val {
        Value::String(s) => s,
        _ => return Err("From node must be a string".to_string()),
    };
    let to = match to_val {
        Value::String(s) => s,
        _ => return Err("To node must be a string".to_string()),
    };
    let weight = match weight_val {
        Value::Integer(i) => i as f64,
        Value::Float(f) => f,
        _ => return Err("Weight must be a number".to_string()),
    };
    match graph_val {
        Value::Graph(mut graph) => {
            if !graph.contains_key(&from) {
                graph.insert(from.clone(), Vec::new());
            }
            if !graph.contains_key(&to) {
                graph.insert(to.clone(), Vec::new());
            }
            if let Some(edges) = graph.get_mut(&from) {
                edges.push((to, weight));
            }
            Ok(Value::Graph(graph))
        }
        _ => Err("add_edge() can only be called on graphs".to_string()),
    }
}

fn call_get_neighbors(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("get_neighbors() expects exactly 2 arguments: get_neighbors(graph, node)".to_string());
    }
    let graph_val = interpreter.eval_node(&args[0])?;
    let node_val = interpreter.eval_node(&args[1])?;
    let node = match node_val {
        Value::String(s) => s,
        _ => return Err("Node must be a string".to_string()),
    };
    match graph_val {
        Value::Graph(graph) => {
            if let Some(neighbors) = graph.get(&node) {
                let list: Vec<Value> = neighbors
                    .iter()
                    .map(|(n, w)| Value::Tuple(vec![Value::String(n.clone()), Value::Float(*w)]))
                    .collect();
                Ok(Value::List(list))
            } else {
                Err(format!("Node '{}' not found in graph", node))
            }
        }
        _ => Err("get_neighbors() can only be called on graphs".to_string()),
    }
}

fn call_push_front(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("push_front() expects exactly 2 arguments: push_front(deque, item)".to_string());
    }
    let deque_val = interpreter.eval_node(&args[0])?;
    let item_val = interpreter.eval_node(&args[1])?;
    match deque_val {
        Value::Deque(mut deque) => {
            deque.insert(0, item_val);
            Ok(Value::Deque(deque))
        }
        _ => Err("push_front() can only be called on deques".to_string()),
    }
}

fn call_push_back(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("push_back() expects exactly 2 arguments: push_back(deque, item)".to_string());
    }
    let deque_val = interpreter.eval_node(&args[0])?;
    let item_val = interpreter.eval_node(&args[1])?;
    match deque_val {
        Value::Deque(mut deque) => {
            deque.push(item_val);
            Ok(Value::Deque(deque))
        }
        _ => Err("push_back() can only be called on deques".to_string()),
    }
}

fn call_pop_front(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("pop_front() expects exactly 1 argument: pop_front(deque)".to_string());
    }
    let deque_val = interpreter.eval_node(&args[0])?;
    match deque_val {
        Value::Deque(mut deque) => {
            if !deque.is_empty() {
                Ok(deque.remove(0))
            } else {
                Err("Cannot pop_front from empty deque".to_string())
            }
        }
        _ => Err("pop_front() can only be called on deques".to_string()),
    }
}

fn call_pop_back(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("pop_back() expects exactly 1 argument: pop_back(deque)".to_string());
    }
    let deque_val = interpreter.eval_node(&args[0])?;
    match deque_val {
        Value::Deque(mut deque) => {
            if let Some(item) = deque.pop() {
                Ok(item)
            } else {
                Err("Cannot pop_back from empty deque".to_string())
            }
        }
        _ => Err("pop_back() can only be called on deques".to_string()),
    }
}

fn call_peek_front(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("peek_front() expects exactly 1 argument: peek_front(deque)".to_string());
    }
    let deque_val = interpreter.eval_node(&args[0])?;
    match deque_val {
        Value::Deque(deque) => {
            if !deque.is_empty() {
                Ok(deque[0].clone())
            } else {
                Err("Cannot peek_front on empty deque".to_string())
            }
        }
        _ => Err("peek_front() can only be called on deques".to_string()),
    }
}

fn call_peek_back(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("peek_back() expects exactly 1 argument: peek_back(deque)".to_string());
    }
    let deque_val = interpreter.eval_node(&args[0])?;
    match deque_val {
        Value::Deque(ref deque) => {
            if !deque.is_empty() {
                Ok(deque[deque.len() - 1].clone())
            } else {
                Err("Cannot peek_back on empty deque".to_string())
            }
        }
        _ => Err("peek_back() can only be called on deques".to_string()),
    }
}

fn call_graph_nodes(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("graph_nodes(graph) expects exactly 1 argument".to_string());
    }
    let graph_val = interpreter.eval_node(&args[0])?;
    match graph_val {
        Value::Graph(graph) => {
            let mut nodes: std::collections::HashSet<String> = graph.keys().cloned().collect();
            for edges in graph.values() {
                for (to, _) in edges {
                    nodes.insert(to.clone());
                }
            }
            let mut list: Vec<String> = nodes.into_iter().collect();
            list.sort();
            Ok(Value::List(
                list.into_iter().map(Value::String).collect(),
            ))
        }
        _ => Err("graph_nodes() can only be called on graphs".to_string()),
    }
}

fn call_graph_edges(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("graph_edges(graph) expects exactly 1 argument".to_string());
    }
    let graph_val = interpreter.eval_node(&args[0])?;
    match graph_val {
        Value::Graph(graph) => {
            let mut out = Vec::new();
            for (from, edges) in graph {
                for (to, w) in edges {
                    out.push(Value::Tuple(vec![
                        Value::String(from.clone()),
                        Value::String(to),
                        Value::Float(w),
                    ]));
                }
            }
            Ok(Value::List(out))
        }
        _ => Err("graph_edges() can only be called on graphs".to_string()),
    }
}

fn call_topological_sort(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("topological_sort(graph) expects exactly 1 argument".to_string());
    }
    let graph_val = interpreter.eval_node(&args[0])?;
    let graph = match &graph_val {
        Value::Graph(g) => g,
        _ => return Err("topological_sort() can only be called on graphs".to_string()),
    };
    let mut nodes: std::collections::HashSet<&String> = graph.keys().collect();
    for edges in graph.values() {
        for (to, _) in edges {
            nodes.insert(to);
        }
    }
    let mut in_degree: HashMap<String, i32> = HashMap::new();
    for n in &nodes {
        in_degree.insert((*n).clone(), 0);
    }
    for edges in graph.values() {
        for (to, _) in edges {
            if let Some(d) = in_degree.get_mut(to) {
                *d += 1;
            }
            // nodes only as targets are already in in_degree from the init loop
        }
    }
    let mut q: VecDeque<String> = in_degree
        .iter()
        .filter(|(_, &d)| d == 0)
        .map(|(k, _)| k.clone())
        .collect();
    let mut order = Vec::new();
    while let Some(u) = q.pop_front() {
        order.push(Value::String(u.clone()));
        if let Some(edges) = graph.get(&u) {
            for (v, _) in edges {
                if let Some(d) = in_degree.get_mut(v) {
                    *d -= 1;
                    if *d == 0 {
                        q.push_back(v.clone());
                    }
                }
            }
        }
    }
    if order.len() != nodes.len() {
        return Err("topological_sort: graph has a cycle".to_string());
    }
    Ok(Value::List(order))
}

fn call_dijkstra(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("dijkstra(graph, start) expects exactly 2 arguments".to_string());
    }
    let graph_val = interpreter.eval_node(&args[0])?;
    let start_val = interpreter.eval_node(&args[1])?;
    let (graph, start) = match (&graph_val, &start_val) {
        (Value::Graph(g), Value::String(s)) => (g, s.clone()),
        _ => return Err("dijkstra() expects (graph, string start node)".to_string()),
    };
    let mut dist: HashMap<String, f64> = HashMap::new();
    dist.insert(start.clone(), 0.0);
    let mut pq: Vec<(f64, String)> = vec![(0.0, start)];
    while !pq.is_empty() {
        pq.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        let (d, u) = pq.remove(0);
        if d > *dist.get(&u).unwrap_or(&f64::INFINITY) {
            continue;
        }
        if let Some(edges) = graph.get(&u) {
            for (v, w) in edges {
                let new_d = d + w;
                if new_d < *dist.get(v).unwrap_or(&f64::INFINITY) {
                    dist.insert(v.clone(), new_d);
                    pq.push((new_d, v.clone()));
                }
            }
        }
    }
    let out: HashMap<String, Value> = dist
        .into_iter()
        .map(|(k, v)| (k, Value::Float(v)))
        .collect();
    Ok(Value::Dict(out))
}

fn call_pq_push(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 3 {
        return Err("pq_push() expects exactly 3 arguments: pq_push(pq, priority, item)".to_string());
    }
    let pq_val = interpreter.eval_node(&args[0])?;
    let priority_val = interpreter.eval_node(&args[1])?;
    let item_val = interpreter.eval_node(&args[2])?;
    let priority = match priority_val {
        Value::Integer(p) => p,
        _ => return Err("Priority must be an integer".to_string()),
    };
    match pq_val {
        Value::PriorityQ(mut pq) => {
            pq.push((priority, item_val));
            pq.sort_by_key(|(p, _)| *p);
            Ok(Value::PriorityQ(pq))
        }
        _ => Err("pq_push() can only be called on priority queues".to_string()),
    }
}

fn call_pq_pop(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("pq_pop() expects exactly 1 argument: pq_pop(pq)".to_string());
    }
    let pq_val = interpreter.eval_node(&args[0])?;
    match pq_val {
        Value::PriorityQ(mut pq) => {
            if !pq.is_empty() {
                Ok(pq.remove(0).1)
            } else {
                Err("Cannot pop from empty priority queue".to_string())
            }
        }
        _ => Err("pq_pop() can only be called on priority queues".to_string()),
    }
}

fn call_pq_peek(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("pq_peek() expects exactly 1 argument: pq_peek(pq)".to_string());
    }
    let pq_val = interpreter.eval_node(&args[0])?;
    match pq_val {
        Value::PriorityQ(pq) => {
            if !pq.is_empty() {
                Ok(pq[0].1.clone())
            } else {
                Err("Cannot peek on empty priority queue".to_string())
            }
        }
        _ => Err("pq_peek() can only be called on priority queues".to_string()),
    }
}
