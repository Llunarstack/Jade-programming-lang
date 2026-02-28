//! Variable/value inspection builtins: len, size, type_of, is_empty.

use crate::interpreter::{Interpreter, Value};
use crate::parser::AstNode;

pub(super) fn try_call(
    interpreter: &mut Interpreter,
    name: &str,
    args: &[AstNode],
) -> Result<Option<Value>, String> {
    let v = match name {
        "len" => Some(call_len(interpreter, args)?),
        "size" => Some(call_size(interpreter, args)?),
        "type_of" => Some(call_type_of(interpreter, args)?),
        "is_empty" => Some(call_is_empty(interpreter, args)?),
        _ => None,
    };
    Ok(v)
}

fn call_len(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("len() expects exactly 1 argument".to_string());
    }
    let val = interpreter.eval_node(&args[0])?;
    match val {
        Value::String(s) => Ok(Value::Integer(s.len() as i64)),
        Value::List(list) => Ok(Value::Integer(list.len() as i64)),
        Value::Tuple(tuple) => Ok(Value::Integer(tuple.len() as i64)),
        _ => Err("len() can only be called on strings, lists, and tuples".to_string()),
    }
}

fn call_size(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("size() expects exactly 1 argument".to_string());
    }
    let val = interpreter.eval_node(&args[0])?;
    match val {
        Value::Set(set) => Ok(Value::Integer(set.len() as i64)),
        Value::Counter(counter) => Ok(Value::Integer(counter.len() as i64)),
        Value::Deque(deque) => Ok(Value::Integer(deque.len() as i64)),
        Value::PriorityQ(pq) => Ok(Value::Integer(pq.len() as i64)),
        Value::Graph(graph) => Ok(Value::Integer(graph.len() as i64)),
        Value::Tree { children, .. } => Ok(Value::Integer(children.len() as i64)),
        Value::List(list) => Ok(Value::Integer(list.len() as i64)),
        Value::Dict(dict) => Ok(Value::Integer(dict.len() as i64)),
        _ => Err("size() can only be called on collections".to_string()),
    }
}

fn call_type_of(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("type_of() expects exactly 1 argument".to_string());
    }
    let val = interpreter.eval_node(&args[0])?;
    let type_name = match val {
        Value::Integer(_) => "int",
        Value::Float(_) => "float",
        Value::String(_) => "str",
        Value::Boolean(_) => "bool",
        Value::Char(_) => "char",
        Value::List(_) => "list",
        Value::Dict(_) => "dict",
        Value::Set(_) => "set",
        Value::Counter(_) => "counter",
        Value::Deque(_) => "deque",
        Value::PriorityQ(_) => "priorityq",
        Value::Graph(_) => "graph",
        Value::Tree { .. } => "tree",
        Value::Function { .. } => "function",
        Value::Infinity(_) => "infinity",
        Value::Emoji(_) => "emoji",
        Value::Money(_, _) => "money",
        Value::Hex(_) => "hex",
        Value::Date(_) => "date",
        Value::Time(_) => "time",
        Value::DateTime(_) => "datetime",
        Value::Tuple(_) => "tuple",
        Value::Range(_, _, _) => "range",
        Value::Task(_) => "task",
        Value::Channel(_) => "channel",
        Value::Vector(_) => "vec",
        Value::Matrix(_) => "mat",
        Value::Grid(_) => "grid",
        Value::GridNeighbors(_) => "grid_neighbors",
        Value::GridNeighbors8(_) => "grid_neighbors8",
        Value::GridFindAll(_) => "grid_find_all",
        Value::GridRow(_) => "grid_row",
        Value::GridCol(_) => "grid_col",
        Value::MatrixRow(_) => "matrix_row",
        Value::MatrixCol(_) => "matrix_col",
        Value::MatrixDiagonal(_) => "matrix_diagonal",
        Value::MatrixFlat(_) => "matrix_flat",
        Value::MatrixRowSums(_) => "matrix_row_sums",
        Value::MatrixColSums(_) => "matrix_col_sums",
        Value::MatrixRowMeans(_) => "matrix_row_means",
        Value::MatrixColMeans(_) => "matrix_col_means",
        Value::Enum { .. } => "enum",
        Value::EnumVariant { .. } => "enum_variant",
        Value::Class { .. } => "class",
        Value::Instance { .. } => "instance",
        Value::Constructor(_) => "constructor",
        Value::OnceCached { .. } => "once",
        Value::MirrorDispatch { .. } => "mirror",
        Value::None => "none",
        Value::Module { .. } => "module",
        Value::Trait { .. } => "trait",
        Value::Future { .. } => "future",
        Value::Interval(_, _) => "interval",
        Value::Queue(_) => "queue",
        Value::Ring { .. } => "ring",
        Value::Sorted(_) => "sorted",
        Value::Bag(_) => "bag",
        Value::Window { .. } => "window",
        Value::View { .. } => "view",
        Value::Prio(_) => "prio",
        Value::Diff(_) => "diff",
        Value::Span { .. } => "span",
        Value::MutSpan { .. } => "mut_span",
        Value::Chunk { .. } => "chunk",
        Value::Sparse { .. } => "sparse",
        Value::Encrypted { .. } => "encrypted",
        Value::Secret(_) => "secret",
        Value::UnionFind { .. } => "union_find",
        Value::Trie(_) => "trie",
        Value::Memoized { .. } => "memoized",
        #[cfg(feature = "regex")]
        Value::Regex(_) => "regex",
        Value::RegexMatch(_) => "match",
        Value::MatchGroup(_) => "match_group",
        Value::BoundMethod { .. } => "bound_method",
        Value::DateType => "date_type",
        Value::TimeType => "time_type",
        Value::DateTimeType => "datetime_type",
        Value::GraphType => "graph_type",
        Value::Duration { .. } => "duration",
        Value::Pool(_) => "pool",
        Value::PoolRef { .. } => "pool_ref",
        Value::SmallVec { .. } => "smallvec",
    };
    Ok(Value::String(type_name.to_string()))
}

fn call_is_empty(interpreter: &mut Interpreter, args: &[AstNode]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("is_empty() expects exactly 1 argument".to_string());
    }
    let val = interpreter.eval_node(&args[0])?;
    let empty = match val {
        Value::String(s) => s.is_empty(),
        Value::List(list) => list.is_empty(),
        Value::Dict(dict) => dict.is_empty(),
        Value::Set(set) => set.is_empty(),
        Value::Counter(counter) => counter.is_empty(),
        Value::Deque(deque) => deque.is_empty(),
        Value::PriorityQ(pq) => pq.is_empty(),
        Value::Graph(graph) => graph.is_empty(),
        Value::Tree { children, .. } => children.is_empty(),
        Value::Tuple(tuple) => tuple.is_empty(),
        Value::Vector(vec) => vec.is_empty(),
        Value::Matrix(mat) => mat.is_empty(),
        _ => return Err("is_empty() can only be called on collections".to_string()),
    };
    Ok(Value::Boolean(empty))
}
