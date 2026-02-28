use super::*;
use crate::parser::AstNode;
use std::collections::HashSet;

impl Interpreter {
    pub(super) fn eval_node(&mut self, node: &AstNode) -> Result<Value, String> {
        match node {
            AstNode::Integer(..)
            | AstNode::Float(..)
            | AstNode::String(..)
            | AstNode::StringInterpolation { .. }
            | AstNode::Boolean(..)
            | AstNode::Char(..)
            | AstNode::Infinity(..)
            | AstNode::Emoji(..)
            | AstNode::Money(..)
            | AstNode::Hex(..)
            | AstNode::Date(..)
            | AstNode::Time(..)
            | AstNode::DateTime(..)
            | AstNode::Tuple(..)
            | AstNode::List(..)
            | AstNode::Vector(..)
            | AstNode::Matrix(..)
            | AstNode::Dict(..)
            | AstNode::Identifier(..) => return super::eval_primitive::eval_primitive(self, node),

            AstNode::VarDeclaration {
                var_type,
                name,
                value,
                immutable: _,
                is_static,
                type_modifier,
            } => {
                let mut val = self.eval_node(value)?;

                // Handle type modifiers (enc, secret, etc.)
                if let Some(modifier) = type_modifier {
                    match modifier.as_str() {
                        "enc" => {
                            {
                                // Auto-encrypt the value
                                let key_id = format!("auto-{}", name);
                                use sha2::{Digest, Sha256};
                                let mut hasher = Sha256::new();
                                hasher.update(key_id.as_bytes());
                                let key = hasher.finalize().to_vec();

                                let json_str = match &val {
                                    Value::Integer(i) => i.to_string(),
                                    Value::Float(f) => f.to_string(),
                                    Value::String(s) => format!("\"{}\"", s),
                                    Value::Boolean(b) => b.to_string(),
                                    _ => serde_json::to_string(&format!("{:?}", val))
                                        .unwrap_or_else(|_| "null".to_string()),
                                };

                                let nonce = super::crypto::generate_nonce(12);
                                let ciphertext = super::crypto::enigma_encrypt(
                                    json_str.as_bytes(),
                                    &key,
                                    &nonce,
                                    b"",
                                )?;

                                val = Value::Encrypted {
                                    ciphertext,
                                    key_id,
                                    nonce,
                                };
                            }
                        }
                        "secret" => {
                            // Wrap in Secret type
                            let secret_str = match val {
                                Value::String(s) => s,
                                Value::Integer(i) => i.to_string(),
                                _ => format!("{:?}", val),
                            };
                            val = Value::Secret(secret_str);
                        }
                        _ => {}
                    }
                }

                // Convert the value based on the declared type
                let converted_val = match var_type.as_str() {
                    "vec" | "vector" => {
                        match val {
                            Value::Vector(_) => val, // Already a vector, no conversion needed
                            Value::List(list) => {
                                // Convert list to vector
                                let mut vector = Vec::new();
                                for item in list {
                                    match item {
                                        Value::Integer(i) => vector.push(i as f64),
                                        Value::Float(f) => vector.push(f),
                                        _ => {
                                            return Err(
                                                "Vector elements must be numeric".to_string()
                                            )
                                        }
                                    }
                                }
                                Value::Vector(vector)
                            }
                            _ => {
                                return Err(
                                    "Vector must be initialized with a list or vector".to_string()
                                )
                            }
                        }
                    }
                    "mat" | "matrix" => {
                        match val {
                            Value::Matrix(_) => val, // Already a matrix, no conversion needed
                            Value::List(list) => {
                                // Convert list of lists to matrix
                                let mut matrix = Vec::new();
                                for row_val in list {
                                    match row_val {
                                        Value::List(row) => {
                                            let mut matrix_row = Vec::new();
                                            for item in row {
                                                match item {
                                                    Value::Integer(i) => matrix_row.push(i as f64),
                                                    Value::Float(f) => matrix_row.push(f),
                                                    _ => {
                                                        return Err(
                                                            "Matrix elements must be numeric"
                                                                .to_string(),
                                                        )
                                                    }
                                                }
                                            }
                                            matrix.push(matrix_row);
                                        }
                                        _ => return Err("Matrix rows must be lists".to_string()),
                                    }
                                }
                                Value::Matrix(matrix)
                            }
                            _ => {
                                return Err(
                                    "Matrix must be initialized with a list of lists or matrix"
                                        .to_string(),
                                )
                            }
                        }
                    }
                    "set" => {
                        match val {
                            Value::Set(_) => val, // Already a set
                            Value::List(list) => {
                                // Convert list to set
                                let mut set = HashSet::new();
                                for item in list {
                                    let key = match item {
                                        Value::String(s) => s,
                                        Value::Integer(i) => i.to_string(),
                                        Value::Float(f) => f.to_string(),
                                        Value::Boolean(b) => b.to_string(),
                                        _ => {
                                            return Err(
                                                "Set elements must be convertible to strings"
                                                    .to_string(),
                                            )
                                        }
                                    };
                                    set.insert(key);
                                }
                                Value::Set(set)
                            }
                            _ => return Err("Set must be initialized with a list".to_string()),
                        }
                    }
                    "counter" => {
                        match val {
                            Value::Counter(_) => val, // Already a counter
                            Value::String(s) => {
                                // Count characters in string
                                let mut counter = HashMap::new();
                                for ch in s.chars() {
                                    let key = ch.to_string();
                                    *counter.entry(key).or_insert(0) += 1;
                                }
                                Value::Counter(counter)
                            }
                            Value::List(list) => {
                                // Count elements in list
                                let mut counter = HashMap::new();
                                for item in list {
                                    let key =
                                        match item {
                                            Value::String(s) => s,
                                            Value::Integer(i) => i.to_string(),
                                            Value::Float(f) => f.to_string(),
                                            Value::Boolean(b) => b.to_string(),
                                            _ => return Err(
                                                "Counter elements must be convertible to strings"
                                                    .to_string(),
                                            ),
                                        };
                                    *counter.entry(key).or_insert(0) += 1;
                                }
                                Value::Counter(counter)
                            }
                            _ => {
                                return Err(
                                    "Counter must be initialized with a string or list".to_string()
                                )
                            }
                        }
                    }
                    "deque" => match val {
                        Value::Deque(_) => val,
                        Value::List(list) => Value::Deque(list),
                        _ => return Err("Deque must be initialized with a list".to_string()),
                    },
                    "priorityq" => {
                        match val {
                            Value::PriorityQ(_) => val,
                            Value::List(list) => {
                                // Convert list of (priority, value) tuples to priority queue
                                let mut pq = Vec::new();
                                for item in list {
                                    match item {
                                        Value::Tuple(tuple) if tuple.len() == 2 => {
                                            let priority = match &tuple[0] {
                                                Value::Integer(i) => *i,
                                                Value::Float(f) => *f as i64,
                                                _ => return Err("Priority must be numeric".to_string()),
                                            };
                                            pq.push((priority, tuple[1].clone()));
                                        }
                                        _ => return Err("Priority queue elements must be (priority, value) tuples".to_string()),
                                    }
                                }
                                Value::PriorityQ(pq)
                            }
                            _ => {
                                return Err(
                                    "Priority queue must be initialized with a list of tuples"
                                        .to_string(),
                                )
                            }
                        }
                    }
                    "graph" => {
                        match val {
                            Value::Graph(_) => val,
                            Value::Dict(dict) => {
                                // Convert dict to graph: {node: [(neighbor, weight), ...]}
                                let mut graph = HashMap::new();
                                for (node, edges_val) in dict {
                                    match edges_val {
                                        Value::List(edges) => {
                                            let mut edge_list = Vec::new();
                                            for edge in edges {
                                                match edge {
                                                    Value::Tuple(tuple) if tuple.len() == 2 => {
                                                        let neighbor = match &tuple[0] {
                                                            Value::String(s) => s.clone(),
                                                            _ => return Err("Graph neighbor must be a string".to_string()),
                                                        };
                                                        let weight = match &tuple[1] {
                                                            Value::Float(f) => *f,
                                                            Value::Integer(i) => *i as f64,
                                                            _ => return Err("Graph weight must be numeric".to_string()),
                                                        };
                                                        edge_list.push((neighbor, weight));
                                                    }
                                                    _ => return Err("Graph edges must be (neighbor, weight) tuples".to_string()),
                                                }
                                            }
                                            graph.insert(node, edge_list);
                                        }
                                        _ => {
                                            return Err(
                                                "Graph node edges must be a list".to_string()
                                            )
                                        }
                                    }
                                }
                                Value::Graph(graph)
                            }
                            _ => {
                                return Err(
                                    "Graph must be initialized with a dictionary".to_string()
                                )
                            }
                        }
                    }
                    "grid" => match val {
                        Value::Grid(_) => val,
                        Value::Matrix(rows) => {
                            let grid: Vec<Vec<Value>> = rows
                                .into_iter()
                                .map(|row| row.into_iter().map(Value::Float).collect())
                                .collect();
                            Value::Grid(grid)
                        }
                        Value::List(rows) => {
                            let mut grid = Vec::new();
                            for row in rows {
                                match row {
                                    Value::List(cells) => grid.push(cells),
                                    _ => {
                                        return Err("Grid must be a list of lists (2D)".to_string())
                                    }
                                }
                            }
                            Value::Grid(grid)
                        }
                        _ => {
                            return Err(
                                "Grid must be initialized with a list of lists or matrix literal"
                                    .to_string(),
                            )
                        }
                    },
                    "tree" => {
                        match val {
                            Value::Tree { .. } => val,
                            Value::Dict(dict) => {
                                // Convert dict to tree: {value: ..., children: [...]}
                                let value = dict
                                    .get("value")
                                    .ok_or_else(|| "Tree must have 'value' field".to_string())?
                                    .clone();
                                let children = match dict.get("children") {
                                    Some(Value::List(children_list)) => children_list.clone(),
                                    Some(_) => {
                                        return Err("Tree 'children' must be a list".to_string())
                                    }
                                    None => Vec::new(),
                                };
                                Value::Tree {
                                    value: Box::new(value),
                                    children,
                                }
                            }
                            _ => {
                                return Err("Tree must be initialized with a dictionary".to_string())
                            }
                        }
                    }
                    // Advanced array types
                    "span" => {
                        match val {
                            Value::Span { .. } => val,
                            Value::List(list) => {
                                // Create a span of the entire list
                                let len = list.len();
                                Value::Span {
                                    source: Box::new(Value::List(list)),
                                    start: 0,
                                    end: len,
                                }
                            }
                            _ => return Err("Span must be initialized with a list".to_string()),
                        }
                    }
                    "mut_span" => match val {
                        Value::MutSpan { .. } => val,
                        Value::List(list) => {
                            let len = list.len();
                            Value::MutSpan {
                                source: Box::new(Value::List(list)),
                                start: 0,
                                end: len,
                            }
                        }
                        _ => return Err("MutSpan must be initialized with a list".to_string()),
                    },
                    "chunk" => match val {
                        Value::Chunk { .. } => val,
                        Value::List(list) => Value::Chunk {
                            source: Box::new(Value::List(list)),
                            chunk_size: 1,
                            current_index: 0,
                        },
                        _ => return Err("Chunk must be initialized with a list".to_string()),
                    },
                    "sparse" => {
                        match val {
                            Value::Sparse { .. } => val,
                            Value::List(list) => {
                                // Convert list to sparse array
                                let mut data = HashMap::new();
                                for (i, item) in list.iter().enumerate() {
                                    data.insert(i, item.clone());
                                }
                                Value::Sparse {
                                    data,
                                    default: Box::new(Value::Integer(0)),
                                    size: list.len(),
                                }
                            }
                            Value::Integer(size) => {
                                // Create empty sparse array of given size
                                Value::Sparse {
                                    data: HashMap::new(),
                                    default: Box::new(Value::Integer(0)),
                                    size: size as usize,
                                }
                            }
                            _ => {
                                return Err(
                                    "Sparse must be initialized with a list or size".to_string()
                                )
                            }
                        }
                    }
                    "ring" => {
                        match val {
                            Value::Ring { .. } => val,
                            Value::Integer(capacity) => {
                                // Create empty ring buffer with given capacity
                                Value::Ring {
                                    buffer: vec![Value::None; capacity as usize],
                                    capacity: capacity as usize,
                                    head: 0,
                                    size: 0,
                                }
                            }
                            Value::List(list) => {
                                // Create ring from list
                                let capacity = list.len();
                                Value::Ring {
                                    buffer: list,
                                    capacity,
                                    head: 0,
                                    size: capacity,
                                }
                            }
                            _ => {
                                return Err("Ring must be initialized with capacity (int) or list"
                                    .to_string())
                            }
                        }
                    }
                    _ => val, // No conversion needed for other types
                };

                if *is_static {
                    self.statics.insert(name.clone(), converted_val.clone());
                } else {
                    self.set_variable(name.clone(), converted_val.clone());
                }
                Ok(converted_val)
            }

            AstNode::EnumDeclaration {
                name,
                backing_type: _,
                variants,
            } => {
                let mut variant_map = HashMap::new();
                let mut next_int_val = 1;

                for (var_name, val_expr) in variants {
                    let val = if let Some(expr) = val_expr {
                        self.eval_node(expr)?
                    } else {
                        let v = Value::Integer(next_int_val);
                        next_int_val += 1;
                        v
                    };
                    variant_map.insert(var_name.clone(), val);
                }

                let enum_val = Value::Enum {
                    name: name.clone(),
                    variants: variant_map,
                };

                self.set_variable(name.clone(), enum_val.clone());
                Ok(enum_val)
            }

            AstNode::ClassDeclaration {
                name,
                class_type,
                parent,
                traits: _,
                fields,
                methods,
                static_fields,
                static_methods,
            } => {
                let mut class_fields = HashMap::new();
                let mut class_methods = HashMap::new();
                let mut class_static_fields = HashMap::new();
                let mut class_static_methods = HashMap::new();

                for field in fields {
                    let default_val = if let Some(ref expr) = field.default_value {
                        self.eval_node(expr)?
                    } else {
                        Value::None
                    };
                    class_fields.insert(field.name.clone(), default_val);
                }

                for field in static_fields {
                    let default_val = if let Some(ref expr) = field.default_value {
                        self.eval_node(expr)?
                    } else {
                        Value::None
                    };
                    class_static_fields.insert(field.name.clone(), default_val);
                }

                for method in methods {
                    if let AstNode::FunctionDeclaration {
                        name: method_name,
                        params,
                        body,
                        ..
                    } = method
                    {
                        let param_names: Vec<String> =
                            params.iter().map(|(_, n)| n.clone()).collect();
                        let func = Value::Function {
                            name: method_name.clone(),
                            params: param_names,
                            body: body.clone(),
                        };
                        class_methods.insert(method_name.clone(), func);
                    }
                }

                for method in static_methods {
                    if let AstNode::FunctionDeclaration {
                        name: method_name,
                        params,
                        body,
                        ..
                    } = method
                    {
                        let param_names: Vec<String> =
                            params.iter().map(|(_, n)| n.clone()).collect();
                        let func = Value::Function {
                            name: method_name.clone(),
                            params: param_names,
                            body: body.clone(),
                        };
                        class_static_methods.insert(method_name.clone(), func);
                    }
                }

                let class_val = Value::Class {
                    name: name.clone(),
                    class_type: class_type.clone(),
                    parent: parent.clone(),
                    fields: class_fields,
                    methods: class_methods,
                    static_fields: class_static_fields,
                    static_methods: class_static_methods,
                };

                self.set_variable(name.clone(), class_val.clone());
                Ok(class_val)
            }

            AstNode::FunctionDeclaration {
                name,
                params,
                body,
                decorators,
                ..
            } => {
                let param_names: Vec<String> =
                    params.iter().map(|(_, name)| name.clone()).collect();
                let mut func = Value::Function {
                    name: name.clone(),
                    params: param_names,
                    body: body.clone(),
                };

                // Apply decorators (bottom-to-top order)
                for decorator in decorators.iter().rev() {
                    func = self.apply_decorator(&decorator.name, &decorator.args, func)?;
                }

                self.set_variable(name.clone(), func.clone());
                Ok(func)
            }

            AstNode::FunctionCall { name, args } => self.call_function(name, args),

            AstNode::Call { callee, args } => {
                let (callee_val, this_opt) =
                    if let AstNode::DotAccess { object, field } = callee.as_ref() {
                        let receiver = self.eval_node(object)?;
                        let val = self.get_property(&receiver, field)?;
                        let this_opt = match &receiver {
                            Value::Instance { .. } => Some(receiver),
                            _ => None,
                        };
                        (val, this_opt)
                    } else {
                        (self.eval_node(callee)?, None)
                    };
                self.call_value(callee_val, args, this_opt)
            }

            AstNode::BroadcastCall { callee, args } => {
                let callee_val = self.eval_node(callee)?;
                let evaled_args: Vec<Value> = args
                    .iter()
                    .map(|a| self.eval_node(a))
                    .collect::<Result<Vec<_>, _>>()?;
                let len = evaled_args.iter().fold(1usize, |acc, v| {
                    if let Value::List(l) = v {
                        acc.max(l.len())
                    } else {
                        acc
                    }
                });
                let mut results = Vec::with_capacity(len);
                for i in 0..len {
                    let call_args: Vec<Value> = evaled_args
                        .iter()
                        .map(|v| match v {
                            Value::List(l) => l.get(i).cloned().unwrap_or(Value::None),
                            _ => v.clone(),
                        })
                        .collect();
                    let result = self.call_value_with_args(callee_val.clone(), &call_args, None)?;
                    results.push(result);
                }
                if results.len() == 1 && evaled_args.iter().all(|v| !matches!(v, Value::List(_))) {
                    Ok(results.into_iter().next().unwrap_or(Value::None))
                } else {
                    Ok(Value::List(results))
                }
            }

            AstNode::Binary {
                left,
                operator,
                right,
            } => {
                // Short-circuit evaluation for And/Or
                match operator {
                    BinaryOp::And => {
                        let left_val = self.eval_node(left)?;
                        match left_val {
                            Value::Boolean(false) => Ok(Value::Boolean(false)),
                            Value::Boolean(true) => {
                                let right_val = self.eval_node(right)?;
                                match right_val {
                                    Value::Boolean(b) => Ok(Value::Boolean(b)),
                                    _ => {
                                        Err(
                                            "And operator requires boolean operands".to_string()
                                        )
                                    }
                                }
                            }
                            _ => Err("And operator requires boolean operands".to_string()),
                        }
                    }
                    BinaryOp::Or => {
                        let left_val = self.eval_node(left)?;
                        match left_val {
                            Value::Boolean(true) => Ok(Value::Boolean(true)),
                            Value::Boolean(false) => {
                                let right_val = self.eval_node(right)?;
                                match right_val {
                                    Value::Boolean(b) => Ok(Value::Boolean(b)),
                                    _ => {
                                        Err(
                                            "Or operator requires boolean operands".to_string()
                                        )
                                    }
                                }
                            }
                            _ => Err("Or operator requires boolean operands".to_string()),
                        }
                    }
                    _ => {
                        // Normal evaluation for other operators
                        let left_val = self.eval_node(left)?;
                        let right_val = self.eval_node(right)?;
                        self.eval_binary_op(&left_val, operator, &right_val)
                    }
                }
            }

            AstNode::Unary { operator, operand } => {
                let val = self.eval_node(operand)?;
                self.eval_unary_op(operator, &val)
            }

            AstNode::Pipeline { left, right } => {
                let left_val = self.eval_node(left)?;

                // Set the pipeline value as '_' for the right side
                self.set_variable("_".to_string(), left_val.clone());

                // Also set it as a special pipeline variable
                self.set_variable("__pipeline__".to_string(), left_val);

                let result = self.eval_node(right)?;

                // Clean up pipeline variable
                self.set_variable("__pipeline__".to_string(), Value::None);

                Ok(result)
            }

            AstNode::DotAccess { object, field } => {
                let obj_val = self.eval_node(object)?;
                match obj_val {
                    Value::Enum {
                        name: enum_name,
                        variants,
                    } => {
                        if let Some(val) = variants.get(field) {
                            Ok(Value::EnumVariant {
                                enum_name: enum_name.clone(),
                                variant_name: field.clone(),
                                value: Box::new(val.clone()),
                            })
                        } else {
                            match field.as_str() {
                                "count" => Ok(Value::Integer(variants.len() as i64)),
                                "names" => {
                                    let names: Vec<Value> =
                                        variants.keys().map(|k| Value::String(k.clone())).collect();
                                    Ok(Value::List(names))
                                }
                                "values" => {
                                    let vals: Vec<Value> = variants.values().cloned().collect();
                                    Ok(Value::List(vals))
                                }
                                _ => Err(format!(
                                    "Unknown variant or method '{}' on enum '{}'",
                                    field, enum_name
                                )),
                            }
                        }
                    }
                    Value::EnumVariant {
                        enum_name: _,
                        variant_name,
                        value,
                    } => match field.as_str() {
                        "label" | "name" => Ok(Value::String(variant_name)),
                        "value" => Ok(*value),
                        _ => Err(format!("Unknown property '{}' on enum variant", field)),
                    },
                    Value::Class {
                        name: class_name,
                        class_type: _,
                        parent: _,
                        fields: _,
                        methods: _,
                        static_fields,
                        static_methods,
                    } => {
                        if field == "new" {
                            Ok(Value::Constructor(class_name.clone()))
                        } else if let Some(v) = static_fields.get(field) {
                            Ok(v.clone())
                        } else if let Some(v) = static_methods.get(field) {
                            Ok(v.clone())
                        } else {
                            Err(format!(
                                "Unknown static field or method '{}' on class '{}'",
                                field, class_name
                            ))
                        }
                    }
                    Value::Instance { class_name, fields } => {
                        if let Some(v) = fields.get(field) {
                            Ok(v.clone())
                        } else {
                            self.get_instance_method(class_name.as_str(), field)
                        }
                    }
                    Value::RegexMatch(m) => match field.as_str() {
                        "text" => Ok(Value::String(m.text.clone())),
                        "start" => Ok(Value::Integer(m.start as i64)),
                        "end" => Ok(Value::Integer(m.end as i64)),
                        "group" => Ok(Value::MatchGroup(m.clone())),
                        _ => m.named.get(field).cloned().map(Value::String).ok_or_else(|| {
                            format!("Unknown match field '{}' (use text, start, end, group(n), or named group)", field)
                        }),
                    },
                    Value::Dict(dict) => {
                        // Regular dictionary field access
                        if let Some(value) = dict.get(field) {
                            Ok(value.clone())
                        } else {
                            // Check if it's a dictionary method
                            match field.as_str() {
                                "items" => {
                                    let mut items = Vec::new();
                                    for (k, v) in dict.iter() {
                                        items.push(Value::Tuple(vec![
                                            Value::String(k.clone()),
                                            v.clone(),
                                        ]));
                                    }
                                    Ok(Value::List(items))
                                }
                                "keys" => {
                                    let keys: Vec<Value> =
                                        dict.keys().map(|k| Value::String(k.clone())).collect();
                                    Ok(Value::List(keys))
                                }
                                "values" => {
                                    let values: Vec<Value> = dict.values().cloned().collect();
                                    Ok(Value::List(values))
                                }
                                "has" => {
                                    Err(
                                        "dict.has() requires a key argument - use dict.has(key)"
                                            .to_string(),
                                    )
                                }
                                "get" => {
                                    Err(
                                        "dict.get() requires key and optional default arguments"
                                            .to_string(),
                                    )
                                }
                                "remove" => {
                                    Err("dict.remove() requires a key argument - use dict.remove(key)".to_string())
                                }
                                "merge" => {
                                    Err(
                                        "dict.merge() requires another dict argument".to_string()
                                    )
                                }
                                "update" => {
                                    Err(
                                        "dict.update() requires another dict argument".to_string()
                                    )
                                }
                                "clear" => {
                                    Err("dict.clear() cannot be called as a method - use clear(dict)".to_string())
                                }
                                "size" | "len" | "length" => Ok(Value::Integer(dict.len() as i64)),
                                _ => Err(format!("Dictionary field '{}' not found", field)),
                            }
                        }
                    }
                    Value::Date(s) => {
                        let parts: Vec<&str> = s.trim().split('-').collect();
                        if parts.len() != 3 {
                            return Err(format!("Invalid date format '{}'", s));
                        }
                        let y: i64 = parts[0].parse().map_err(|_| "Invalid year".to_string())?;
                        let m: i64 = parts[1].parse().map_err(|_| "Invalid month".to_string())?;
                        let d: i64 = parts[2].parse().map_err(|_| "Invalid day".to_string())?;
                        match field.as_str() {
                            "year" => Ok(Value::Integer(y)),
                            "month" => Ok(Value::Integer(m)),
                            "day" => Ok(Value::Integer(d)),
                            _ => Err(format!("Unknown date property '{}'", field)),
                        }
                    }
                    Value::Time(s) => {
                        let s = s.trim();
                        let time_part = s.split('.').next().unwrap_or(s);
                        let parts: Vec<&str> = time_part.split(':').collect();
                        if parts.len() < 3 {
                            return Err(format!("Invalid time format '{}'", s));
                        }
                        let h: i64 = parts[0].parse().map_err(|_| "Invalid hour".to_string())?;
                        let min: i64 = parts[1].parse().map_err(|_| "Invalid minute".to_string())?;
                        let sec: i64 = parts[2].parse().map_err(|_| "Invalid second".to_string())?;
                        match field.as_str() {
                            "hour" => Ok(Value::Integer(h)),
                            "minute" => Ok(Value::Integer(min)),
                            "second" => Ok(Value::Integer(sec)),
                            _ => Err(format!("Unknown time property '{}'", field)),
                        }
                    }
                    Value::DateTime(s) => {
                        let s = s.trim();
                        let (date_part, time_part) = match s.find('T').or_else(|| s.find(' ')) {
                            Some(pos) => {
                                let (d, t) = s.split_at(pos);
                                (d.trim(), t.trim().trim_start_matches('T').trim_start_matches(' '))
                            }
                            None => (s, "00:00:00"),
                        };
                        let d_parts: Vec<&str> = date_part.split('-').collect();
                        let t_parts: Vec<&str> = time_part.split(':').collect();
                        if d_parts.len() != 3 {
                            return Err(format!("Invalid datetime date part '{}'", s));
                        }
                        let y: i64 = d_parts[0].parse().map_err(|_| "Invalid year".to_string())?;
                        let m: i64 = d_parts[1].parse().map_err(|_| "Invalid month".to_string())?;
                        let day: i64 = d_parts[2].parse().map_err(|_| "Invalid day".to_string())?;
                        let h: i64 = t_parts.get(0).and_then(|x| x.parse().ok()).unwrap_or(0);
                        let min: i64 = t_parts.get(1).and_then(|x| x.parse().ok()).unwrap_or(0);
                        let sec: i64 = t_parts.get(2).and_then(|x| x.parse().ok()).unwrap_or(0);
                        match field.as_str() {
                            "year" => Ok(Value::Integer(y)),
                            "month" => Ok(Value::Integer(m)),
                            "day" => Ok(Value::Integer(day)),
                            "hour" => Ok(Value::Integer(h)),
                            "minute" => Ok(Value::Integer(min)),
                            "second" => Ok(Value::Integer(sec)),
                            "date" => Ok(Value::Date(format!("{:04}-{:02}-{:02}", y, m, day))),
                            "time" => Ok(Value::Time(format!("{:02}:{:02}:{:02}", h, min, sec))),
                            _ => Err(format!("Unknown datetime property '{}'", field)),
                        }
                    }
                    Value::Duration { total_seconds } => match field.as_str() {
                        "days" => Ok(Value::Integer(total_seconds / 86400)),
                        "hours" => Ok(Value::Integer(total_seconds / 3600)),
                        "minutes" => Ok(Value::Integer(total_seconds / 60)),
                        "seconds" => Ok(Value::Integer(total_seconds)),
                        "is_zero" => Ok(Value::Boolean(total_seconds == 0)),
                        _ => Err(format!("Unknown interval property '{}'", field)),
                    },
                    Value::List(list) => {
                        // List methods
                        match field.as_str() {
                            "len" | "length" | "size" => Ok(Value::Integer(list.len() as i64)),
                            "first" => list
                                .first()
                                .cloned()
                                .ok_or_else(|| "List is empty".to_string()),
                            "last" => list
                                .last()
                                .cloned()
                                .ok_or_else(|| "List is empty".to_string()),
                            "sum" => {
                                let mut sum = 0.0;
                                for item in list.iter() {
                                    match item {
                                        Value::Integer(i) => sum += *i as f64,
                                        Value::Float(f) => sum += f,
                                        _ => {
                                            return Err("sum() requires numeric values".to_string())
                                        }
                                    }
                                }
                                if list.iter().all(|v| matches!(v, Value::Integer(_))) {
                                    Ok(Value::Integer(sum as i64))
                                } else {
                                    Ok(Value::Float(sum))
                                }
                            }
                            "product" => {
                                let mut product = 1.0;
                                for item in list.iter() {
                                    match item {
                                        Value::Integer(i) => product *= *i as f64,
                                        Value::Float(f) => product *= f,
                                        _ => {
                                            return Err(
                                                "product() requires numeric values".to_string()
                                            )
                                        }
                                    }
                                }
                                if list.iter().all(|v| matches!(v, Value::Integer(_))) {
                                    Ok(Value::Integer(product as i64))
                                } else {
                                    Ok(Value::Float(product))
                                }
                            }
                            "min" => {
                                if list.is_empty() {
                                    return Err("min() requires non-empty list".to_string());
                                }
                                let mut min_val = list[0].clone();
                                for item in list.iter().skip(1) {
                                    match (&min_val, item) {
                                        (Value::Integer(a), Value::Integer(b)) if b < a => {
                                            min_val = item.clone()
                                        }
                                        (Value::Float(a), Value::Float(b)) if b < a => {
                                            min_val = item.clone()
                                        }
                                        (Value::Integer(a), Value::Float(b))
                                            if (*b as i64) < *a =>
                                        {
                                            min_val = item.clone()
                                        }
                                        (Value::Float(a), Value::Integer(b))
                                            if (*b as f64) < *a =>
                                        {
                                            min_val = item.clone()
                                        }
                                        _ => {}
                                    }
                                }
                                Ok(min_val)
                            }
                            "max" => {
                                if list.is_empty() {
                                    return Err("max() requires non-empty list".to_string());
                                }
                                let mut max_val = list[0].clone();
                                for item in list.iter().skip(1) {
                                    match (&max_val, item) {
                                        (Value::Integer(a), Value::Integer(b)) if b > a => {
                                            max_val = item.clone()
                                        }
                                        (Value::Float(a), Value::Float(b)) if b > a => {
                                            max_val = item.clone()
                                        }
                                        (Value::Integer(a), Value::Float(b))
                                            if (*b as i64) > *a =>
                                        {
                                            max_val = item.clone()
                                        }
                                        (Value::Float(a), Value::Integer(b))
                                            if (*b as f64) > *a =>
                                        {
                                            max_val = item.clone()
                                        }
                                        _ => {}
                                    }
                                }
                                Ok(max_val)
                            }
                            "scan_max" => {
                                let mut result = Vec::new();
                                let mut running_max = None;
                                for item in list.iter() {
                                    let v = match item {
                                        Value::Integer(i) => *i as f64,
                                        Value::Float(f) => *f,
                                        _ => {
                                            return Err(
                                                "scan_max() requires numeric list".to_string()
                                            )
                                        }
                                    };
                                    running_max =
                                        Some(running_max.map(|m: f64| m.max(v)).unwrap_or(v));
                                    result.push(Value::Float(running_max.unwrap()));
                                }
                                if list.iter().all(|v| matches!(v, Value::Integer(_))) {
                                    Ok(Value::List(
                                        result
                                            .iter()
                                            .map(|v| {
                                                if let Value::Float(f) = v {
                                                    Value::Integer(*f as i64)
                                                } else {
                                                    v.clone()
                                                }
                                            })
                                            .collect(),
                                    ))
                                } else {
                                    Ok(Value::List(result))
                                }
                            }
                            "scan_sum" => {
                                let mut result = Vec::new();
                                let mut running = 0.0;
                                for item in list.iter() {
                                    match item {
                                        Value::Integer(i) => running += *i as f64,
                                        Value::Float(f) => running += f,
                                        _ => {
                                            return Err(
                                                "scan_sum() requires numeric list".to_string()
                                            )
                                        }
                                    }
                                    result.push(Value::Float(running));
                                }
                                if list.iter().all(|v| matches!(v, Value::Integer(_))) {
                                    Ok(Value::List(
                                        result
                                            .iter()
                                            .map(|v| {
                                                if let Value::Float(f) = v {
                                                    Value::Integer(*f as i64)
                                                } else {
                                                    v.clone()
                                                }
                                            })
                                            .collect(),
                                    ))
                                } else {
                                    Ok(Value::List(result))
                                }
                            }
                            "scan_right_max" => {
                                let mut result = Vec::new();
                                let mut running_max = None;
                                for item in list.iter().rev() {
                                    let v = match item {
                                        Value::Integer(i) => *i as f64,
                                        Value::Float(f) => *f,
                                        _ => {
                                            return Err("scan_right_max() requires numeric list"
                                                .to_string())
                                        }
                                    };
                                    running_max =
                                        Some(running_max.map(|m: f64| m.max(v)).unwrap_or(v));
                                    if let Some(max_val) = running_max {
                                        result.push(Value::Float(max_val));
                                    }
                                }
                                result.reverse();
                                if list.iter().all(|v| matches!(v, Value::Integer(_))) {
                                    Ok(Value::List(
                                        result
                                            .iter()
                                            .map(|v| {
                                                if let Value::Float(f) = v {
                                                    Value::Integer(*f as i64)
                                                } else {
                                                    v.clone()
                                                }
                                            })
                                            .collect(),
                                    ))
                                } else {
                                    Ok(Value::List(result))
                                }
                            }
                            "mean" | "average" | "avg" => {
                                if list.is_empty() {
                                    return Err("mean() requires non-empty list".to_string());
                                }
                                let mut sum = 0.0;
                                for item in list.iter() {
                                    match item {
                                        Value::Integer(i) => sum += *i as f64,
                                        Value::Float(f) => sum += f,
                                        _ => {
                                            return Err("mean() requires numeric values".to_string())
                                        }
                                    }
                                }
                                Ok(Value::Float(sum / list.len() as f64))
                            }
                            "median" => {
                                if list.is_empty() {
                                    return Err("median() requires non-empty list".to_string());
                                }
                                let mut sorted = list.clone();
                                sorted.sort_by(|a, b| match (a, b) {
                                    (Value::Integer(x), Value::Integer(y)) => x.cmp(y),
                                    (Value::Float(x), Value::Float(y)) => {
                                        x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal)
                                    }
                                    (Value::Integer(x), Value::Float(y)) => (*x as f64)
                                        .partial_cmp(y)
                                        .unwrap_or(std::cmp::Ordering::Equal),
                                    (Value::Float(x), Value::Integer(y)) => x
                                        .partial_cmp(&(*y as f64))
                                        .unwrap_or(std::cmp::Ordering::Equal),
                                    _ => std::cmp::Ordering::Equal,
                                });
                                let mid = sorted.len() / 2;
                                if sorted.len() % 2 == 0 {
                                    // Even number of elements - average the two middle ones
                                    match (&sorted[mid - 1], &sorted[mid]) {
                                        (Value::Integer(a), Value::Integer(b)) => {
                                            Ok(Value::Float((a + b) as f64 / 2.0))
                                        }
                                        (Value::Float(a), Value::Float(b)) => {
                                            Ok(Value::Float((a + b) / 2.0))
                                        }
                                        (Value::Integer(a), Value::Float(b)) => {
                                            Ok(Value::Float((*a as f64 + b) / 2.0))
                                        }
                                        (Value::Float(a), Value::Integer(b)) => {
                                            Ok(Value::Float((a + *b as f64) / 2.0))
                                        }
                                        _ => Ok(sorted[mid - 1].clone()),
                                    }
                                } else {
                                    Ok(sorted[mid].clone())
                                }
                            }
                            "sorted" => {
                                let mut sorted = list.clone();
                                sorted.sort_by(|a, b| match (a, b) {
                                    (Value::Integer(x), Value::Integer(y)) => x.cmp(y),
                                    (Value::Float(x), Value::Float(y)) => {
                                        x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal)
                                    }
                                    (Value::Integer(x), Value::Float(y)) => (*x as f64)
                                        .partial_cmp(y)
                                        .unwrap_or(std::cmp::Ordering::Equal),
                                    (Value::Float(x), Value::Integer(y)) => x
                                        .partial_cmp(&(*y as f64))
                                        .unwrap_or(std::cmp::Ordering::Equal),
                                    (Value::String(x), Value::String(y)) => x.cmp(y),
                                    _ => std::cmp::Ordering::Equal,
                                });
                                Ok(Value::List(sorted))
                            }
                            "reversed" => {
                                let mut reversed = list.clone();
                                reversed.reverse();
                                Ok(Value::List(reversed))
                            }
                            "unique" | "distinct" => {
                                let mut seen = std::collections::HashSet::new();
                                let mut unique = Vec::new();
                                for item in list {
                                    let key = format!("{:?}", item);
                                    if seen.insert(key) {
                                        unique.push(item.clone());
                                    }
                                }
                                Ok(Value::List(unique))
                            }
                            "empty" | "is_empty" => Ok(Value::Boolean(list.is_empty())),
                            "any" => {
                                Err(
                                    "list.any() requires a predicate function - use any(list, fn)"
                                        .to_string(),
                                )
                            }
                            "all" => {
                                Err(
                                    "list.all() requires a predicate function - use all(list, fn)"
                                        .to_string(),
                                )
                            }
                            "count" => {
                                Err(
                                    "list.count() requires an argument - use list.count(value)"
                                        .to_string(),
                                )
                            }
                            "contains" => {
                                Err("list.contains() requires an argument - use list.contains(value)".to_string())
                            }
                            "index" | "find" => {
                                Err(
                                    "list.index() requires an argument - use list.index(value)"
                                        .to_string(),
                                )
                            }
                            "join" => {
                                Err("list.join() requires a separator argument - use list.join(sep)".to_string())
                            }
                            _ => Err(format!("List method '{}' not found", field)),
                        }
                    }
                    Value::String(s) => {
                        // String methods
                        match field.as_str() {
                            "len" | "length" | "size" => Ok(Value::Integer(s.len() as i64)),
                            "first" => s
                                .chars()
                                .next()
                                .map(Value::Char)
                                .ok_or_else(|| "String is empty".to_string()),
                            "last" => s
                                .chars()
                                .last()
                                .map(Value::Char)
                                .ok_or_else(|| "String is empty".to_string()),
                            "empty" | "is_empty" => Ok(Value::Boolean(s.is_empty())),
                            "upper" | "uppercase" => Ok(Value::String(s.to_uppercase())),
                            "lower" | "lowercase" => Ok(Value::String(s.to_lowercase())),
                            "trim" => Ok(Value::String(s.trim().to_string())),
                            "trim_start" | "trim_left" => {
                                Ok(Value::String(s.trim_start().to_string()))
                            }
                            "trim_end" | "trim_right" => {
                                Ok(Value::String(s.trim_end().to_string()))
                            }
                            "lines" => {
                                let lines: Vec<Value> =
                                    s.lines().map(|l| Value::String(l.to_string())).collect();
                                Ok(Value::List(lines))
                            }
                            "chars" => {
                                let chars: Vec<Value> = s.chars().map(Value::Char).collect();
                                Ok(Value::List(chars))
                            }
                            "split" => {
                                Err("string.split() requires a separator argument - use string.split(sep)".to_string())
                            }
                            "contains" => {
                                Err("string.contains() requires a substring argument - use string.contains(substr)".to_string())
                            }
                            "starts_with" | "startswith" => {
                                Err(
                                    "string.starts_with() requires a prefix argument".to_string()
                                )
                            }
                            "ends_with" | "endswith" => {
                                Err(
                                    "string.ends_with() requires a suffix argument".to_string()
                                )
                            }
                            "replace" => {
                                Err("string.replace() requires old and new arguments - use string.replace(old, new)".to_string())
                            }
                            _ => Err(format!("String method '{}' not found", field)),
                        }
                    }
                    Value::Counter(counter) => match field.as_str() {
                        "most_common" => {
                            let mut items: Vec<_> = counter.iter().collect();
                            items.sort_by(|a, b| b.1.cmp(a.1));
                            let result: Vec<Value> = items
                                .iter()
                                .map(|(k, v)| {
                                    Value::Tuple(vec![
                                        Value::String(k.to_string()),
                                        Value::Integer(**v),
                                    ])
                                })
                                .collect();
                            Ok(Value::List(result))
                        }
                        "elements" | "keys" => {
                            let keys: Vec<Value> =
                                counter.keys().map(|k| Value::String(k.clone())).collect();
                            Ok(Value::List(keys))
                        }
                        "total" => {
                            let total: i64 = counter.values().sum();
                            Ok(Value::Integer(total))
                        }
                        "len" | "length" | "size" => Ok(Value::Integer(counter.len() as i64)),
                        _ => Err(format!("Counter method '{}' not found", field)),
                    },
                    Value::Interval(start, end) => {
                        match field.as_str() {
                            "start" => Ok(Value::Integer(start)),
                            "end" => Ok(Value::Integer(end)),
                            "len" | "length" | "size" => Ok(Value::Integer((end - start).abs())),
                            "overlaps" => {
                                // Returns a callable that checks if another interval overlaps
                                Err(
                                    "interval.overlaps() requires another interval argument"
                                        .to_string(),
                                )
                            }
                            "merge" => {
                                Err("interval.merge() requires another interval argument"
                                    .to_string())
                            }
                            "contains" => {
                                Err(
                                    "interval.contains() requires a value argument".to_string()
                                )
                            }
                            _ => Err(format!("Interval method '{}' not found", field)),
                        }
                    }
                    Value::Grid(grid) => {
                        let rows = grid.len() as i64;
                        let cols = if grid.is_empty() {
                            0
                        } else {
                            grid[0].len() as i64
                        };
                        match field.as_str() {
                            "rows" => Ok(Value::Integer(rows)),
                            "cols" | "columns" => Ok(Value::Integer(cols)),
                            "len" | "length" | "size" => Ok(Value::Integer(rows * cols)),
                            "neighbors" => {
                                Ok(Value::GridNeighbors(Box::new(Value::Grid(grid.clone()))))
                            }
                            _ => Err(format!("Grid method '{}' not found", field)),
                        }
                    }
                    Value::Matrix(mat) => {
                        // Use get_property for Matrix to get all the accessor methods
                        self.get_property(&Value::Matrix(mat), field)
                    }
                    _ => Err(format!(
                        "Cannot access field '{}' on type {}",
                        field,
                        match obj_val {
                            Value::Integer(_) => "integer",
                            Value::Float(_) => "float",
                            Value::Boolean(_) => "boolean",
                            Value::Char(_) => "char",
                            Value::Function { .. } => "function",
                            _ => "unknown",
                        }
                    )),
                }
            }

            AstNode::Index { object, index } => {
                let obj_val = self.eval_node(object)?;
                let idx_val = self.eval_node(index)?;

                match (obj_val, idx_val) {
                    (
                        Value::Enum {
                            name: enum_name,
                            variants,
                        },
                        val,
                    ) => {
                        // Reverse lookup: directions[1]
                        for (key, variant_val) in variants.iter() {
                            if variant_val == &val {
                                return Ok(Value::EnumVariant {
                                    enum_name: enum_name.clone(),
                                    variant_name: key.clone(),
                                    value: Box::new(variant_val.clone()),
                                });
                            }
                        }
                        Err(format!("Value '{}' not found in enum '{}'", val, enum_name))
                    }
                    (Value::List(list), Value::Integer(i)) => {
                        // CRITICAL FIX: Proper negative index handling with bounds checking
                        let idx = if i < 0 {
                            let abs_i = i.unsigned_abs() as usize;
                            if abs_i > list.len() {
                                return Err(format!(
                                    "Index {} out of bounds (length {})",
                                    i,
                                    list.len()
                                ));
                            }
                            list.len() - abs_i
                        } else {
                            i as usize
                        };

                        if idx < list.len() {
                            Ok(list[idx].clone())
                        } else {
                            Err(JError::index_out_of_bounds(i, list.len(), 0, 0).to_string())
                        }
                    }
                    (Value::SmallVec { cap: _, elements }, Value::Integer(idx_val)) => {
                        let idx = if idx_val < 0 {
                            let abs_i = idx_val.unsigned_abs() as usize;
                            if abs_i > elements.len() {
                                return Err(format!(
                                    "Index {} out of bounds (length {})",
                                    idx_val,
                                    elements.len()
                                ));
                            }
                            elements.len() - abs_i
                        } else {
                            idx_val as usize
                        };
                        if idx < elements.len() {
                            Ok(elements[idx].clone())
                        } else {
                            Err(JError::index_out_of_bounds(idx_val, elements.len(), 0, 0).to_string())
                        }
                    }
                    (Value::Grid(grid), Value::Integer(i)) => {
                        // CRITICAL FIX: Proper negative index handling
                        let idx = if i < 0 {
                            let abs_i = i.unsigned_abs() as usize;
                            if abs_i > grid.len() {
                                return Err(format!(
                                    "Grid row index {} out of bounds (rows {})",
                                    i,
                                    grid.len()
                                ));
                            }
                            grid.len() - abs_i
                        } else {
                            i as usize
                        };
                        if idx < grid.len() {
                            Ok(Value::List(grid[idx].clone()))
                        } else {
                            Err(format!(
                                "Grid row index {} out of bounds (rows {})",
                                i,
                                grid.len()
                            ))
                        }
                    }
                    (Value::String(s), Value::Integer(i)) => {
                        let chars: Vec<char> = s.chars().collect();
                        // CRITICAL FIX: Proper negative index handling
                        let idx = if i < 0 {
                            let abs_i = i.unsigned_abs() as usize;
                            if abs_i > chars.len() {
                                return Err(format!(
                                    "String index {} out of bounds (length {})",
                                    i,
                                    chars.len()
                                ));
                            }
                            chars.len() - abs_i
                        } else {
                            i as usize
                        };

                        if idx < chars.len() {
                            Ok(Value::Char(chars[idx]))
                        } else {
                            Err(format!(
                                "String index {} out of bounds (length {})",
                                i,
                                chars.len()
                            ))
                        }
                    }
                    (Value::RegexMatch(m), Value::Integer(i)) => {
                        if i == 0 {
                            Ok(Value::String(m.text.clone()))
                        } else if i >= 1 && (i as usize) <= m.groups.len() {
                            Ok(Value::String(m.groups[(i - 1) as usize].clone()))
                        } else {
                            Ok(Value::None)
                        }
                    }
                    (Value::RegexMatch(m), Value::String(name)) => {
                        m.named.get(&name).cloned().map(Value::String).ok_or_else(|| {
                            format!("No named group '{}' in match", name)
                        })
                    }
                    (Value::Dict(dict), Value::String(key)) => {
                        if let Some(value) = dict.get(&key) {
                            Ok(value.clone())
                        } else {
                            Err(JError::key_not_found(&key, 0, 0).to_string())
                        }
                    }
                    (Value::Dict(dict), Value::Integer(i)) => {
                        let key = i.to_string();
                        if let Some(value) = dict.get(&key) {
                            Ok(value.clone())
                        } else {
                            Err(JError::key_not_found(&key, 0, 0).to_string())
                        }
                    }
                    (Value::Tuple(tuple), Value::Integer(i)) => {
                        // CRITICAL FIX: Proper negative index handling
                        let idx = if i < 0 {
                            let abs_i = i.unsigned_abs() as usize;
                            if abs_i > tuple.len() {
                                return Err(format!(
                                    "Tuple index {} out of bounds (length {})",
                                    i,
                                    tuple.len()
                                ));
                            }
                            tuple.len() - abs_i
                        } else {
                            i as usize
                        };

                        if idx < tuple.len() {
                            Ok(tuple[idx].clone())
                        } else {
                            Err(format!(
                                "Tuple index {} out of bounds (length {})",
                                i,
                                tuple.len()
                            ))
                        }
                    }
                    (obj, idx) => Err(format!(
                        "Cannot index {} with {}",
                        match obj {
                            Value::Integer(_) => "integer",
                            Value::Float(_) => "float",
                            Value::Boolean(_) => "boolean",
                            Value::Char(_) => "char",
                            Value::Function { .. } => "function",
                            Value::Infinity(_) => "infinity",
                            Value::Emoji(_) => "emoji",
                            Value::Money(_, _) => "money",
                            Value::Hex(_) => "hex",
                            Value::Date(_) => "date",
                            Value::Time(_) => "time",
                            Value::DateTime(_) => "datetime",
                            Value::Range(_, _, _) => "range",
                            Value::Task(_) => "task",
                            Value::Channel(_) => "channel",
                            Value::None => "none",
                            _ => "unknown",
                        },
                        match idx {
                            Value::Integer(_) => "integer",
                            Value::String(_) => "string",
                            _ => "invalid index type",
                        }
                    )),
                }
            }

            AstNode::Slice {
                object,
                start,
                end,
                step,
            } => {
                let obj_val = self.eval_node(object)?;

                // Evaluate slice parameters
                let start_idx = if let Some(start_node) = start {
                    match self.eval_node(start_node)? {
                        Value::Integer(i) => Some(i),
                        _ => return Err("Slice start must be an integer".to_string()),
                    }
                } else {
                    None
                };

                let end_idx = if let Some(end_node) = end {
                    match self.eval_node(end_node)? {
                        Value::Integer(i) => Some(i),
                        _ => return Err("Slice end must be an integer".to_string()),
                    }
                } else {
                    None
                };

                let step_val = if let Some(step_node) = step {
                    match self.eval_node(step_node)? {
                        Value::Integer(i) => {
                            if i == 0 {
                                return Err("Slice step cannot be zero".to_string());
                            }
                            i
                        }
                        _ => return Err("Slice step must be an integer".to_string()),
                    }
                } else {
                    1
                };

                match obj_val {
                    Value::List(list) => {
                        let len = list.len() as i64;
                        let (start, end) =
                            self.normalize_slice_indices(start_idx, end_idx, len, step_val)?;

                        let mut result = Vec::new();
                        if step_val > 0 {
                            let mut i = start;
                            while i < end && i < len {
                                if i >= 0 {
                                    result.push(list[i as usize].clone());
                                }
                                i += step_val;
                            }
                        } else {
                            let mut i = start;
                            while i > end && i >= 0 {
                                if i < len {
                                    result.push(list[i as usize].clone());
                                }
                                i += step_val; // step_val is negative
                            }
                        }
                        Ok(Value::List(result))
                    }
                    Value::String(s) => {
                        let chars: Vec<char> = s.chars().collect();
                        let len = chars.len() as i64;
                        let (start, end) =
                            self.normalize_slice_indices(start_idx, end_idx, len, step_val)?;

                        let mut result_chars = Vec::new();
                        if step_val > 0 {
                            let mut i = start;
                            while i < end && i < len {
                                if i >= 0 {
                                    result_chars.push(chars[i as usize]);
                                }
                                i += step_val;
                            }
                        } else {
                            let mut i = start;
                            while i > end && i >= 0 {
                                if i < len {
                                    result_chars.push(chars[i as usize]);
                                }
                                i += step_val; // step_val is negative
                            }
                        }
                        let result_string: String = result_chars.into_iter().collect();
                        Ok(Value::String(result_string))
                    }
                    Value::Vector(vec) => {
                        let len = vec.len() as i64;
                        let (start, end) =
                            self.normalize_slice_indices(start_idx, end_idx, len, step_val)?;

                        let mut result = Vec::new();
                        if step_val > 0 {
                            let mut i = start;
                            while i < end && i < len {
                                if i >= 0 {
                                    result.push(vec[i as usize]);
                                }
                                i += step_val;
                            }
                        } else {
                            let mut i = start;
                            while i > end && i >= 0 {
                                if i < len {
                                    result.push(vec[i as usize]);
                                }
                                i += step_val; // step_val is negative
                            }
                        }
                        Ok(Value::Vector(result))
                    }
                    _ => Err("Cannot slice this type".to_string()),
                }
            }

            AstNode::If {
                condition,
                then_branch,
                else_branch,
            } => self.eval_if(condition, then_branch, else_branch),

            AstNode::Match { expr, arms } => self.eval_match(expr, arms),

            AstNode::Cond { value, branches } => self.eval_cond(value, branches),

            AstNode::When { value, branches } => self.eval_when(value, branches),

            AstNode::Unless {
                condition,
                then_branch,
                else_branch,
            } => self.eval_unless(condition, then_branch, else_branch),

            AstNode::Either {
                expr,
                true_body,
                false_body,
            } => self.eval_either(expr, true_body, false_body),

            AstNode::GuardReturn {
                condition,
                return_value,
            } => self.eval_guard_return(condition, return_value),

            AstNode::TightBlock { body } => {
                // Stack-only scope: run body in new scope (optional future: enforce no heap)
                self.push_scope();
                let result = self.eval_node(body);
                self.pop_scope();
                result
            }

            AstNode::BorrowSplit {
                target,
                index,
                left_var,
                right_var,
                body,
            } => {
                let list_val = self.eval_node(target)?;
                let list = match &list_val {
                    Value::List(l) => l.clone(),
                    _ => return Err("borrow_split target must be a list".to_string()),
                };
                let idx_val = self.eval_node(index)?;
                let idx = match idx_val {
                    Value::Integer(i) if i >= 0 => i as usize,
                    _ => return Err("borrow_split index must be a non-negative integer".to_string()),
                };
                if idx > list.len() {
                    return Err("borrow_split index out of bounds".to_string());
                }
                let mut left: Vec<Value> = list[..idx].to_vec();
                let mut right: Vec<Value> = list[idx..].to_vec();
                self.push_scope();
                self.set_variable(left_var.clone(), Value::List(left.clone()));
                self.set_variable(right_var.clone(), Value::List(right.clone()));
                let _ = self.eval_node(body)?;
                if let Ok(Value::List(l)) = self.get_variable(left_var) {
                    left = l;
                }
                if let Ok(Value::List(r)) = self.get_variable(right_var) {
                    right = r;
                }
                self.pop_scope();
                let merged: Vec<Value> = left.into_iter().chain(right).collect();
                if let AstNode::Identifier(name) = target.as_ref() {
                    self.set_variable(name.clone(), Value::List(merged));
                }
                Ok(Value::None)
            }

            AstNode::Switch { expr, cases } => self.eval_switch(expr, cases),

            AstNode::While { condition, body } => {
                let mut last_val = Value::None;

                loop {
                    let cond_result = self.eval_node(condition)?;
                    if !self.is_truthy(&cond_result) {
                        break;
                    }

                    match self.eval_node(body) {
                        Ok(val) => last_val = val,
                        Err(e) if e == "Break statement outside of loop" => break,
                        Err(e) if e == "Continue statement outside of loop" => continue,
                        Err(e) => return Err(e),
                    }
                }

                Ok(last_val)
            }

            AstNode::TryCatch {
                try_block,
                catch_var,
                catch_block,
                finally_block,
            } => self.eval_try_catch(try_block, catch_var, catch_block, finally_block),

            AstNode::Throw(expr) => self.eval_throw(expr),

            AstNode::For {
                var,
                iterable,
                body,
            } => {
                let iterable_val = self.eval_node(iterable)?;
                let mut last_val = Value::None;

                match iterable_val {
                    Value::List(list) => {
                        for item in list {
                            self.set_variable(var.clone(), item);
                            match self.eval_node(body) {
                                Ok(val) => last_val = val,
                                Err(e) if e == "Break statement outside of loop" => break,
                                Err(e) if e == "Continue statement outside of loop" => continue,
                                Err(e) => return Err(e),
                            }
                        }
                    }
                    Value::String(s) => {
                        for ch in s.chars() {
                            self.set_variable(var.clone(), Value::Char(ch));
                            match self.eval_node(body) {
                                Ok(val) => last_val = val,
                                Err(e) if e == "Break statement outside of loop" => break,
                                Err(e) if e == "Continue statement outside of loop" => continue,
                                Err(e) => return Err(e),
                            }
                        }
                    }
                    Value::Dict(dict) => {
                        for (key, _value) in dict {
                            // For simple iteration, iterate over keys
                            self.set_variable(var.clone(), Value::String(key));
                            match self.eval_node(body) {
                                Ok(val) => last_val = val,
                                Err(e) if e == "Break statement outside of loop" => break,
                                Err(e) if e == "Continue statement outside of loop" => continue,
                                Err(e) => return Err(e),
                            }
                        }
                    }
                    Value::Tuple(tuple) => {
                        for item in tuple {
                            self.set_variable(var.clone(), item);
                            match self.eval_node(body) {
                                Ok(val) => last_val = val,
                                Err(e) if e == "Break statement outside of loop" => break,
                                Err(e) if e == "Continue statement outside of loop" => continue,
                                Err(e) => return Err(e),
                            }
                        }
                    }
                    Value::Vector(vec) => {
                        for item in vec {
                            self.set_variable(var.clone(), Value::Float(item));
                            match self.eval_node(body) {
                                Ok(val) => last_val = val,
                                Err(e) if e == "Break statement outside of loop" => break,
                                Err(e) if e == "Continue statement outside of loop" => continue,
                                Err(e) => return Err(e),
                            }
                        }
                    }
                    _ => return Err(
                        "Can only iterate over lists, strings, dictionaries, tuples, and vectors"
                            .to_string(),
                    ),
                }

                Ok(last_val)
            }

            AstNode::ForReverse {
                var,
                iterable,
                body,
            } => {
                let iterable_val = self.eval_node(iterable)?;
                let mut last_val = Value::None;

                match iterable_val {
                    Value::List(mut list) => {
                        list.reverse();
                        for item in list {
                            self.set_variable(var.clone(), item);
                            last_val = self.eval_node(body)?;
                        }
                    }
                    Value::String(s) => {
                        let chars: Vec<char> = s.chars().rev().collect();
                        for ch in chars {
                            self.set_variable(var.clone(), Value::Char(ch));
                            last_val = self.eval_node(body)?;
                        }
                    }
                    Value::Vector(mut vec) => {
                        vec.reverse();
                        for item in vec {
                            self.set_variable(var.clone(), Value::Float(item));
                            last_val = self.eval_node(body)?;
                        }
                    }
                    _ => {
                        return Err(
                            "Can only reverse iterate over lists, strings, and vectors".to_string()
                        )
                    }
                }

                Ok(last_val)
            }

            AstNode::ForStep {
                var,
                start,
                step,
                condition,
                body,
            } => {
                let start_val = self.eval_node(start)?;
                let step_val = self.eval_node(step)?;
                let mut last_val = Value::None;

                match (start_val, step_val) {
                    (Value::Integer(s), Value::Integer(st)) => {
                        if st == 0 {
                            return Err("Step cannot be zero".to_string());
                        }

                        let mut current = s;
                        loop {
                            // Check condition if provided
                            if let Some(cond) = condition {
                                self.set_variable(var.clone(), Value::Integer(current));
                                let cond_result = self.eval_node(cond)?;
                                if !self.is_truthy(&cond_result) {
                                    break;
                                }
                            }

                            self.set_variable(var.clone(), Value::Integer(current));
                            last_val = self.eval_node(body)?;
                            current += st;

                            // Simple overflow protection
                            if current.abs() > 1_000_000 {
                                break;
                            }
                        }
                    }
                    _ => return Err("ForStep requires integer start and step values".to_string()),
                }

                Ok(last_val)
            }

            AstNode::ForIndexed {
                index_var,
                value_var,
                iterable,
                body,
            } => {
                let iterable_val = self.eval_node(iterable)?;
                let mut last_val = Value::None;

                match iterable_val {
                    Value::List(list) => {
                        for (index, item) in list.iter().enumerate() {
                            self.set_variable(index_var.clone(), Value::Integer(index as i64));
                            self.set_variable(value_var.clone(), item.clone());
                            last_val = self.eval_node(body)?;
                        }
                    }
                    Value::String(s) => {
                        for (index, ch) in s.chars().enumerate() {
                            self.set_variable(index_var.clone(), Value::Integer(index as i64));
                            self.set_variable(value_var.clone(), Value::Char(ch));
                            last_val = self.eval_node(body)?;
                        }
                    }
                    Value::Dict(dict) => {
                        for (index, (key, _value)) in dict.iter().enumerate() {
                            self.set_variable(index_var.clone(), Value::Integer(index as i64));
                            self.set_variable(value_var.clone(), Value::String(key.clone()));
                            last_val = self.eval_node(body)?;
                        }
                    }
                    Value::Tuple(tuple) => {
                        for (index, item) in tuple.iter().enumerate() {
                            self.set_variable(index_var.clone(), Value::Integer(index as i64));
                            self.set_variable(value_var.clone(), item.clone());
                            last_val = self.eval_node(body)?;
                        }
                    }
                    Value::Vector(vec) => {
                        for (index, item) in vec.iter().enumerate() {
                            self.set_variable(index_var.clone(), Value::Integer(index as i64));
                            self.set_variable(value_var.clone(), Value::Float(*item));
                            last_val = self.eval_node(body)?;
                        }
                    }
                    _ => return Err(
                        "Can only iterate over lists, strings, dictionaries, tuples, and vectors"
                            .to_string(),
                    ),
                }

                Ok(last_val)
            }

            AstNode::ForZip {
                vars,
                iterables,
                body,
            } => {
                if vars.len() != iterables.len() {
                    return Err(
                        "Number of variables must match number of iterables in zip".to_string()
                    );
                }

                // Evaluate all iterables
                let mut evaluated_iterables = Vec::new();
                for iterable in iterables {
                    evaluated_iterables.push(self.eval_node(iterable)?);
                }

                // Convert to lists for zipping
                let mut lists = Vec::new();
                for iterable_val in evaluated_iterables {
                    match iterable_val {
                        Value::List(list) => lists.push(list),
                        Value::String(s) => {
                            let chars: Vec<Value> = s.chars().map(Value::Char).collect();
                            lists.push(chars);
                        }
                        Value::Vector(vec) => {
                            let floats: Vec<Value> = vec.iter().map(|&f| Value::Float(f)).collect();
                            lists.push(floats);
                        }
                        Value::Tuple(tuple) => lists.push(tuple),
                        _ => {
                            return Err(
                                "Can only zip over lists, strings, vectors, and tuples".to_string()
                            )
                        }
                    }
                }

                // Find minimum length
                let min_len = lists.iter().map(|l| l.len()).min().unwrap_or(0);

                let mut last_val = Value::None;
                for i in 0..min_len {
                    // Set each variable to the corresponding element
                    for (var_idx, var_name) in vars.iter().enumerate() {
                        if var_idx < lists.len() {
                            self.set_variable(var_name.clone(), lists[var_idx][i].clone());
                        }
                    }
                    last_val = self.eval_node(body)?;
                }

                Ok(last_val)
            }

            AstNode::ForParallel {
                var,
                iterable,
                body,
                workers: _,
                ordered: _,
            } => {
                // For now, implement as sequential (true parallel would require threading)
                // This is a placeholder for future parallel execution
                let iterable_val = self.eval_node(iterable)?;
                let mut last_val = Value::None;

                match iterable_val {
                    Value::List(list) => {
                        for item in list {
                            self.set_variable(var.clone(), item);
                            last_val = self.eval_node(body)?;
                        }
                    }
                    Value::String(s) => {
                        for ch in s.chars() {
                            self.set_variable(var.clone(), Value::Char(ch));
                            last_val = self.eval_node(body)?;
                        }
                    }
                    Value::Vector(vec) => {
                        for item in vec {
                            self.set_variable(var.clone(), Value::Float(item));
                            last_val = self.eval_node(body)?;
                        }
                    }
                    _ => {
                        return Err("Can only parallel iterate over lists, strings, and vectors"
                            .to_string())
                    }
                }

                Ok(last_val)
            }

            // Enhanced for loop variants
            AstNode::ForChunked {
                var,
                iterable,
                chunk_size,
                body,
            } => {
                let iterable_val = self.eval_node(iterable)?;
                let chunk_size_val = self.eval_node(chunk_size)?;
                let mut last_val = Value::None;

                let chunk_size = match chunk_size_val {
                    Value::Integer(size) => {
                        if size <= 0 {
                            return Err("Chunk size must be positive".to_string());
                        }
                        size as usize
                    }
                    _ => return Err("Chunk size must be an integer".to_string()),
                };

                match iterable_val {
                    Value::List(list) => {
                        for chunk in list.chunks(chunk_size) {
                            self.set_variable(var.clone(), Value::List(chunk.to_vec()));
                            last_val = self.eval_node(body)?;
                        }
                    }
                    Value::String(s) => {
                        let chars: Vec<char> = s.chars().collect();
                        for chunk in chars.chunks(chunk_size) {
                            let chunk_chars: Vec<Value> =
                                chunk.iter().map(|&c| Value::Char(c)).collect();
                            self.set_variable(var.clone(), Value::List(chunk_chars));
                            last_val = self.eval_node(body)?;
                        }
                    }
                    _ => {
                        return Err("ForChunked can only iterate over lists and strings".to_string())
                    }
                }

                Ok(last_val)
            }

            AstNode::ForFiltered {
                var,
                iterable,
                filter,
                body,
            } => {
                let iterable_val = self.eval_node(iterable)?;
                let mut last_val = Value::None;

                match iterable_val {
                    Value::List(list) => {
                        for item in list {
                            self.set_variable(var.clone(), item.clone());

                            // Evaluate filter condition
                            let filter_result = self.eval_node(filter)?;
                            if self.is_truthy(&filter_result) {
                                last_val = self.eval_node(body)?;
                            }
                        }
                    }
                    Value::String(s) => {
                        for ch in s.chars() {
                            self.set_variable(var.clone(), Value::Char(ch));

                            let filter_result = self.eval_node(filter)?;
                            if self.is_truthy(&filter_result) {
                                last_val = self.eval_node(body)?;
                            }
                        }
                    }
                    _ => {
                        return Err(
                            "ForFiltered can only iterate over lists and strings".to_string()
                        )
                    }
                }

                Ok(last_val)
            }

            AstNode::ForWindowed {
                var,
                iterable,
                window_size,
                body,
            } => {
                let iterable_val = self.eval_node(iterable)?;
                let window_size_val = self.eval_node(window_size)?;
                let mut last_val = Value::None;

                let window_size = match window_size_val {
                    Value::Integer(size) => {
                        if size <= 0 {
                            return Err("Window size must be positive".to_string());
                        }
                        size as usize
                    }
                    _ => return Err("Window size must be an integer".to_string()),
                };

                match iterable_val {
                    Value::List(list) => {
                        if list.len() >= window_size {
                            for i in 0..=(list.len() - window_size) {
                                let window: Vec<Value> = list[i..i + window_size].to_vec();
                                self.set_variable(var.clone(), Value::List(window));
                                last_val = self.eval_node(body)?;
                            }
                        }
                    }
                    Value::String(s) => {
                        let chars: Vec<char> = s.chars().collect();
                        if chars.len() >= window_size {
                            for i in 0..=(chars.len() - window_size) {
                                let window: Vec<Value> = chars[i..i + window_size]
                                    .iter()
                                    .map(|&c| Value::Char(c))
                                    .collect();
                                self.set_variable(var.clone(), Value::List(window));
                                last_val = self.eval_node(body)?;
                            }
                        }
                    }
                    _ => {
                        return Err(
                            "ForWindowed can only iterate over lists and strings".to_string()
                        )
                    }
                }

                Ok(last_val)
            }

            // Advanced algorithm loops
            AstNode::SweepLoop {
                left_var,
                right_var,
                iterable,
                body,
            } => {
                let iterable_val = self.eval_node(iterable)?;
                let mut last_val = Value::None;

                match iterable_val {
                    Value::List(list) => {
                        let mut left = 0i64;
                        let mut right = 0i64;

                        // Initialize variables
                        self.set_variable(left_var.clone(), Value::Integer(left));
                        self.set_variable(right_var.clone(), Value::Integer(right));

                        // User controls the loop via break/continue
                        loop {
                            // Check bounds
                            if right >= list.len() as i64 {
                                break;
                            }

                            // Update variables before body execution
                            self.set_variable(left_var.clone(), Value::Integer(left));
                            self.set_variable(right_var.clone(), Value::Integer(right));

                            // Execute body - user must update left/right
                            last_val = self.eval_node(body)?;

                            // Read updated values
                            left = match self.get_variable(left_var)? {
                                Value::Integer(i) => i,
                                _ => {
                                    return Err("Sweep left variable must be an integer".to_string())
                                }
                            };
                            right = match self.get_variable(right_var)? {
                                Value::Integer(i) => i,
                                _ => {
                                    return Err(
                                        "Sweep right variable must be an integer".to_string()
                                    )
                                }
                            };

                            // Safety check to prevent infinite loops
                            if left < 0
                                || right < 0
                                || left > list.len() as i64
                                || right > list.len() as i64
                            {
                                break;
                            }
                        }
                    }
                    _ => return Err("Sweep loop requires a list".to_string()),
                }

                Ok(last_val)
            }

            AstNode::ShrinkLoop {
                left_var,
                right_var,
                iterable,
                body,
            } => {
                let iterable_val = self.eval_node(iterable)?;
                let mut last_val = Value::None;

                match iterable_val {
                    Value::List(list) => {
                        let mut left = 0i64;
                        let mut right = (list.len() as i64) - 1;

                        while left <= right {
                            self.set_variable(left_var.clone(), Value::Integer(left));
                            self.set_variable(right_var.clone(), Value::Integer(right));

                            last_val = self.eval_node(body)?;

                            // Read updated values
                            left = match self.get_variable(left_var)? {
                                Value::Integer(i) => i,
                                _ => {
                                    return Err(
                                        "Shrink left variable must be an integer".to_string()
                                    )
                                }
                            };
                            right = match self.get_variable(right_var)? {
                                Value::Integer(i) => i,
                                _ => {
                                    return Err(
                                        "Shrink right variable must be an integer".to_string()
                                    )
                                }
                            };
                        }
                    }
                    _ => return Err("Shrink loop requires a list".to_string()),
                }

                Ok(last_val)
            }

            AstNode::MeetLoop {
                left_var,
                right_var,
                iterable,
                body,
            } => {
                let iterable_val = self.eval_node(iterable)?;
                let mut last_val = Value::None;

                match iterable_val {
                    Value::List(list) => {
                        let mut left = 0i64;
                        let mut right = (list.len() as i64) - 1;

                        while left < right {
                            self.set_variable(left_var.clone(), Value::Integer(left));
                            self.set_variable(right_var.clone(), Value::Integer(right));

                            last_val = self.eval_node(body)?;

                            // Read updated values
                            left = match self.get_variable(left_var)? {
                                Value::Integer(i) => i,
                                _ => {
                                    return Err("Meet left variable must be an integer".to_string())
                                }
                            };
                            right = match self.get_variable(right_var)? {
                                Value::Integer(i) => i,
                                _ => {
                                    return Err("Meet right variable must be an integer".to_string())
                                }
                            };
                        }
                    }
                    _ => return Err("Meet loop requires a list".to_string()),
                }

                Ok(last_val)
            }

            AstNode::BinarySearchLoop {
                lo_var,
                hi_var,
                range,
                body,
                else_block,
            } => {
                let range_val = self.eval_node(range)?;
                let mut last_val = Value::None;
                let mut found = false;

                match range_val {
                    Value::List(list) => {
                        let mut lo = 0i64;
                        let mut hi = (list.len() as i64) - 1;

                        while lo <= hi {
                            self.set_variable(lo_var.clone(), Value::Integer(lo));
                            self.set_variable(hi_var.clone(), Value::Integer(hi));

                            last_val = self.eval_node(body)?;

                            // Check if user signaled a match (by setting a special variable or breaking)
                            // For now, user must update lo/hi to continue search
                            let new_lo = match self.get_variable(lo_var)? {
                                Value::Integer(i) => i,
                                _ => {
                                    return Err("Binary lo variable must be an integer".to_string())
                                }
                            };
                            let new_hi = match self.get_variable(hi_var)? {
                                Value::Integer(i) => i,
                                _ => {
                                    return Err("Binary hi variable must be an integer".to_string())
                                }
                            };

                            // If lo and hi didn't change, assume found
                            if new_lo == lo && new_hi == hi {
                                found = true;
                                break;
                            }

                            lo = new_lo;
                            hi = new_hi;
                        }
                    }
                    Value::Range(start, end, _step) => {
                        let mut lo = start;
                        let mut hi = end - 1;

                        while lo <= hi {
                            self.set_variable(lo_var.clone(), Value::Integer(lo));
                            self.set_variable(hi_var.clone(), Value::Integer(hi));

                            last_val = self.eval_node(body)?;

                            let new_lo = match self.get_variable(lo_var)? {
                                Value::Integer(i) => i,
                                _ => {
                                    return Err("Binary lo variable must be an integer".to_string())
                                }
                            };
                            let new_hi = match self.get_variable(hi_var)? {
                                Value::Integer(i) => i,
                                _ => {
                                    return Err("Binary hi variable must be an integer".to_string())
                                }
                            };

                            if new_lo == lo && new_hi == hi {
                                found = true;
                                break;
                            }

                            lo = new_lo;
                            hi = new_hi;
                        }
                    }
                    _ => return Err("Binary search loop requires a list or range".to_string()),
                }

                // Execute else block if not found
                if !found {
                    if let Some(else_node) = else_block.as_ref() {
                        last_val = self.eval_node(else_node)?;
                    }
                }

                Ok(last_val)
            }

            AstNode::DpLoop {
                table_name,
                dimensions,
                init_value,
                body,
            } => {
                // Evaluate dimensions
                let mut dim_sizes = Vec::new();
                for dim in dimensions {
                    match self.eval_node(dim)? {
                        Value::Integer(size) => {
                            if size <= 0 {
                                return Err("DP dimension must be positive".to_string());
                            }
                            dim_sizes.push(size as usize);
                        }
                        _ => return Err("DP dimensions must be integers".to_string()),
                    }
                }

                // Initialize table based on dimensions
                let init_val = self.eval_node(init_value)?;

                if dim_sizes.len() == 1 {
                    // 1D table
                    let table = vec![init_val.clone(); dim_sizes[0]];
                    self.set_variable(table_name.clone(), Value::List(table));
                } else if dim_sizes.len() == 2 {
                    // 2D table
                    let mut table = Vec::new();
                    for _ in 0..dim_sizes[0] {
                        let row = vec![init_val.clone(); dim_sizes[1]];
                        table.push(Value::List(row));
                    }
                    self.set_variable(table_name.clone(), Value::List(table));
                } else {
                    return Err("DP loop currently supports 1D and 2D tables".to_string());
                }

                // Execute body
                let last_val = self.eval_node(body)?;

                Ok(last_val)
            }

            AstNode::WhileNonzero { var, body } => {
                let mut last_val = Value::None;

                loop {
                    let var_val = self.get_variable(var)?;

                    match var_val {
                        Value::Integer(n) => {
                            if n == 0 {
                                break;
                            }
                        }
                        _ => return Err("while_nonzero requires an integer variable".to_string()),
                    }

                    last_val = self.eval_node(body)?;
                }

                Ok(last_val)
            }

            AstNode::WhileChange { var, init, body } => {
                let init_val = self.eval_node(init)?;
                self.set_variable(var.clone(), init_val.clone());

                // Execute body at least once, then continue while the value changes
                let mut last_val = self.eval_node(body)?;
                let mut prev_val = self.get_variable(var)?;

                loop {
                    if prev_val == self.get_variable(var)? {
                        break;
                    }

                    prev_val = self.get_variable(var)?;
                    last_val = self.eval_node(body)?;
                }

                Ok(last_val)
            }

            AstNode::WhileMatchLoop { var, body } => {
                // This is a pattern-matching while loop
                // For now, implement as a simple while loop that continues until var is None
                let mut last_val = Value::None;

                loop {
                    let var_val = self.get_variable(var)?;

                    if matches!(var_val, Value::None) {
                        break;
                    }

                    last_val = self.eval_node(body)?;
                }

                Ok(last_val)
            }

            AstNode::Range {
                start,
                end,
                inclusive,
                step,
            } => {
                let start_val = self.eval_node(start)?;
                let end_val = self.eval_node(end)?;

                match (start_val, end_val) {
                    (Value::Integer(s), Value::Integer(e)) => {
                        let mut range = Vec::new();
                        let step_val = if let Some(step_node) = step {
                            match self.eval_node(step_node)? {
                                Value::Integer(step) => step,
                                _ => return Err("Range step must be an integer".to_string()),
                            }
                        } else {
                            1
                        };

                        if step_val == 0 {
                            return Err("Range step cannot be zero".to_string());
                        }

                        let mut current = s;
                        if step_val > 0 {
                            while current < e || (*inclusive && current == e) {
                                range.push(Value::Integer(current));
                                current += step_val;
                            }
                        } else {
                            while current > e || (*inclusive && current == e) {
                                range.push(Value::Integer(current));
                                current += step_val;
                            }
                        }

                        Ok(Value::List(range))
                    }
                    _ => Err("Range bounds must be integers".to_string()),
                }
            }

            AstNode::Block(statements) => self.eval_block(statements),

            AstNode::Defer(_) => Ok(Value::None),

            AstNode::ConvergeLoop { body } => {
                let mut prev = Value::None;
                loop {
                    self.push_scope();
                    self.set_variable("_".to_string(), prev.clone());
                    let next = self.eval_node(body)?;
                    self.pop_scope();
                    if prev == next {
                        break Ok(next);
                    }
                    prev = next;
                }
            }

            AstNode::Expression(expr) => self.eval_node(expr),

            AstNode::Assignment { name, value } => self.eval_assignment(name, value),

            AstNode::DestructuringAssignment { targets, value } => {
                self.eval_destructuring_assignment(targets, value)
            }

            AstNode::Return(expr) => self.eval_return(expr),

            AstNode::TryExpression(expr) => {
                // For now, just evaluate the expression
                // In a full implementation, this would handle Result types
                self.eval_node(expr)
            }

            AstNode::ExecuteFile { filename } => {
                // Execute another J file
                self.execute_file(filename)
            }

            AstNode::Lambda { params, body } => Ok(Value::Function {
                name: "<lambda>".to_string(),
                params: params.clone(),
                body: body.clone(),
            }),

            AstNode::ListComprehension {
                expr,
                var,
                iterable,
                condition,
            } => {
                let iterable_val = self.eval_node(iterable)?;
                let mut result = Vec::new();

                match iterable_val {
                    Value::List(list) => {
                        for item in list {
                            self.set_variable(var.clone(), item);

                            // Check condition if provided
                            if let Some(cond) = condition {
                                let cond_result = self.eval_node(cond)?;
                                if !self.is_truthy(&cond_result) {
                                    continue;
                                }
                            }

                            let expr_result = self.eval_node(expr)?;
                            result.push(expr_result);
                        }
                    }
                    Value::String(s) => {
                        for ch in s.chars() {
                            self.set_variable(var.clone(), Value::Char(ch));

                            if let Some(cond) = condition {
                                let cond_result = self.eval_node(cond)?;
                                if !self.is_truthy(&cond_result) {
                                    continue;
                                }
                            }

                            let expr_result = self.eval_node(expr)?;
                            result.push(expr_result);
                        }
                    }
                    Value::Vector(vec) => {
                        for item in vec {
                            self.set_variable(var.clone(), Value::Float(item));

                            if let Some(cond) = condition {
                                let cond_result = self.eval_node(cond)?;
                                if !self.is_truthy(&cond_result) {
                                    continue;
                                }
                            }

                            let expr_result = self.eval_node(expr)?;
                            result.push(expr_result);
                        }
                    }
                    _ => {
                        return Err(
                            "List comprehension can only iterate over lists, strings, and vectors"
                                .to_string(),
                        )
                    }
                }

                Ok(Value::List(result))
            }

            AstNode::DictComprehension {
                key_expr,
                value_expr,
                var,
                iterable,
                condition,
            } => {
                let iterable_val = self.eval_node(iterable)?;
                let mut result = HashMap::new();

                match iterable_val {
                    Value::List(list) => {
                        for item in list {
                            self.set_variable(var.clone(), item);

                            // Check condition if provided
                            if let Some(cond) = condition {
                                let cond_result = self.eval_node(cond)?;
                                if !self.is_truthy(&cond_result) {
                                    continue;
                                }
                            }

                            let key_result = self.eval_node(key_expr)?;
                            let value_result = self.eval_node(value_expr)?;

                            let key =
                                match key_result {
                                    Value::String(s) => s,
                                    Value::Integer(i) => i.to_string(),
                                    _ => return Err(
                                        "Dictionary comprehension keys must be strings or integers"
                                            .to_string(),
                                    ),
                                };

                            result.insert(key, value_result);
                        }
                    }
                    Value::String(s) => {
                        for ch in s.chars() {
                            self.set_variable(var.clone(), Value::Char(ch));

                            if let Some(cond) = condition {
                                let cond_result = self.eval_node(cond)?;
                                if !self.is_truthy(&cond_result) {
                                    continue;
                                }
                            }

                            let key_result = self.eval_node(key_expr)?;
                            let value_result = self.eval_node(value_expr)?;

                            let key =
                                match key_result {
                                    Value::String(s) => s,
                                    Value::Integer(i) => i.to_string(),
                                    _ => return Err(
                                        "Dictionary comprehension keys must be strings or integers"
                                            .to_string(),
                                    ),
                                };

                            result.insert(key, value_result);
                        }
                    }
                    _ => {
                        return Err(
                            "Dictionary comprehension can only iterate over lists and strings"
                                .to_string(),
                        )
                    }
                }

                Ok(Value::Dict(result))
            }

            // Missing AST node patterns
            AstNode::TypeConversion { target_type, name } => {
                // Type conversion: type*variable
                let current_value = self.get_variable(name)?;

                let converted_value = match target_type.as_str() {
                    "str" => match current_value {
                        Value::Integer(i) => Value::String(i.to_string()),
                        Value::Float(f) => Value::String(f.to_string()),
                        Value::Boolean(b) => Value::String(b.to_string()),
                        Value::Char(c) => Value::String(c.to_string()),
                        Value::String(s) => Value::String(s),
                        _ => Value::String(current_value.to_string()),
                    },
                    "int" => match current_value {
                        Value::Integer(i) => Value::Integer(i),
                        Value::Float(f) => Value::Integer(f as i64),
                        Value::String(s) => s
                            .parse::<i64>()
                            .map(Value::Integer)
                            .unwrap_or(Value::Integer(0)),
                        Value::Boolean(b) => Value::Integer(if b { 1 } else { 0 }),
                        _ => Value::Integer(0),
                    },
                    "float" => match current_value {
                        Value::Integer(i) => Value::Float(i as f64),
                        Value::Float(f) => Value::Float(f),
                        Value::String(s) => s
                            .parse::<f64>()
                            .map(Value::Float)
                            .unwrap_or(Value::Float(0.0)),
                        Value::Boolean(b) => Value::Float(if b { 1.0 } else { 0.0 }),
                        _ => Value::Float(0.0),
                    },
                    "bool" => match current_value {
                        Value::Boolean(b) => Value::Boolean(b),
                        Value::Integer(i) => Value::Boolean(i != 0),
                        Value::Float(f) => Value::Boolean(f != 0.0),
                        Value::String(s) => Value::Boolean(!s.is_empty()),
                        Value::List(l) => Value::Boolean(!l.is_empty()),
                        _ => Value::Boolean(true),
                    },
                    "list" => match current_value {
                        Value::List(l) => Value::List(l),
                        Value::String(s) => {
                            let chars: Vec<Value> = s.chars().map(Value::Char).collect();
                            Value::List(chars)
                        }
                        Value::Vector(v) => {
                            let floats: Vec<Value> = v.iter().map(|&f| Value::Float(f)).collect();
                            Value::List(floats)
                        }
                        Value::Tuple(t) => Value::List(t),
                        _ => Value::List(vec![current_value]),
                    },
                    "vec" => match current_value {
                        Value::Vector(v) => Value::Vector(v),
                        Value::List(l) => {
                            let mut vec = Vec::new();
                            for item in l {
                                match item {
                                    Value::Integer(i) => vec.push(i as f64),
                                    Value::Float(f) => vec.push(f),
                                    _ => {
                                        return Err(
                                            "Cannot convert non-numeric list to vector".to_string()
                                        )
                                    }
                                }
                            }
                            Value::Vector(vec)
                        }
                        _ => return Err(format!("Cannot convert {} to vector", target_type)),
                    },
                    _ => return Err(format!("Unsupported type conversion to {}", target_type)),
                };

                // Create new variable with converted value (shadowing)
                self.set_variable(name.clone(), converted_value.clone());
                Ok(converted_value)
            }

            AstNode::Break => {
                // Break statement - should be handled by loop constructs
                Err("Break statement outside of loop".to_string())
            }

            AstNode::Continue => {
                // Continue statement - should be handled by loop constructs
                Err("Continue statement outside of loop".to_string())
            }

            AstNode::Underscore => {
                // Anonymous variable - return the current pipeline value if available
                Ok(self.get_variable("_").unwrap_or(Value::None))
            }

            // Advanced J features (not yet implemented)
            AstNode::AutoFunction { .. } => Err("AutoFunction not yet implemented".to_string()),

            AstNode::CheatBlock { .. } => Err("CheatBlock not yet implemented".to_string()),

            AstNode::LiveVariable { .. } => Err("LiveVariable not yet implemented".to_string()),

            AstNode::WhyExpression { .. } => Err("WhyExpression not yet implemented".to_string()),

            AstNode::BlendExpression { .. } => {
                Err("BlendExpression not yet implemented".to_string())
            }

            AstNode::EchoVariable { .. } => Err("EchoVariable not yet implemented".to_string()),

            AstNode::TraceBlock { .. } => Err("TraceBlock not yet implemented".to_string()),

            AstNode::GuardExpression { .. } => {
                Err("GuardExpression not yet implemented".to_string())
            }

            AstNode::LensView { .. } => Err("LensView not yet implemented".to_string()),

            // Memory management
            AstNode::ArenaAllocation { .. } => {
                Err("ArenaAllocation not yet implemented".to_string())
            }

            AstNode::StackAllocation { .. } => {
                Err("StackAllocation not yet implemented".to_string())
            }

            // Concurrency
            AstNode::TaskSpawn { body } => {
                // Create a task ID and spawn the task
                // For now, execute synchronously (true async would need runtime)
                let task_id = self.next_future_id;
                self.next_future_id += 1;

                // Execute the task body
                self.push_scope();
                let _result = self.eval_node(body)?;
                self.pop_scope();

                // Return a Task value
                Ok(Value::Task(task_id as u64))
            }

            AstNode::ChannelSend { channel, value } => {
                // Evaluate channel and value
                let chan_val = self.eval_node(channel)?;
                let _val = self.eval_node(value)?;

                // For now, just return success
                // Full implementation would need a channel queue
                match chan_val {
                    Value::Channel(_id) => {
                        // Store value in a hypothetical channel queue
                        // For now, just acknowledge the send
                        Ok(Value::Boolean(true))
                    }
                    _ => Err("ChannelSend requires a Channel value".to_string()),
                }
            }

            AstNode::ChannelReceive { channel } => {
                // Evaluate channel
                let chan_val = self.eval_node(channel)?;

                match chan_val {
                    Value::Channel(_id) => {
                        // For now, return None (no value in channel)
                        // Full implementation would need a channel queue
                        Ok(Value::None)
                    }
                    _ => Err("ChannelReceive requires a Channel value".to_string()),
                }
            }

            AstNode::ScopeBlock { .. } => Err("ScopeBlock not yet implemented".to_string()),

            // Testing
            AstNode::TestCase { .. } => Err("TestCase not yet implemented".to_string()),

            AstNode::PropertyTest { .. } => Err("PropertyTest not yet implemented".to_string()),

            AstNode::Assertion { .. } => Err("Assertion not yet implemented".to_string()),

            // Macros
            AstNode::MacroDefinition { .. } => {
                Err("MacroDefinition not yet implemented".to_string())
            }

            AstNode::MacroCall { .. } => Err("MacroCall not yet implemented".to_string()),

            // ===== NEW ADVANCED FEATURE HANDLERS =====

            // Traits
            AstNode::TraitDeclaration { name, methods } => {
                // Convert methods to TraitMethod structs
                let trait_methods: Vec<TraitMethod> = methods.iter().filter_map(|m| {
                    if let AstNode::FunctionDeclaration { name: method_name, params, return_type, body, .. } = m {
                        Some(TraitMethod {
                            name: method_name.clone(),
                            params: params.clone(),
                            return_type: return_type.clone(),
                            default_impl: if matches!(body.as_ref(), AstNode::Block(statements) if statements.is_empty()) {
                                None
                            } else {
                                Some(body.clone())
                            },
                        })
                    } else {
                        None
                    }
                }).collect();

                let trait_val = Value::Trait {
                    name: name.clone(),
                    methods: trait_methods,
                };

                self.set_variable(name.clone(), trait_val);
                Ok(Value::None)
            }

            // Async/Await
            AstNode::AsyncFunction {
                name,
                params,
                return_type: _,
                body,
            } => {
                // Create an async function that can be awaited
                // For now, it executes synchronously but returns a completed Future
                let param_names: Vec<String> = params.iter().map(|(_, p)| p.clone()).collect();
                let func = Value::Function {
                    name: format!("async_{}", name),
                    params: param_names,
                    body: body.clone(),
                };

                self.set_variable(name.clone(), func);
                Ok(Value::None)
            }

            AstNode::AwaitExpression { expr } => {
                let future_val = self.eval_node(expr)?;

                // If it's a Future, get its result
                if let Value::Future { result, state, .. } = future_val {
                    match state {
                        FutureState::Completed => {
                            if let Some(res) = result {
                                return Ok(*res);
                            } else {
                                return Ok(Value::None);
                            }
                        }
                        FutureState::Failed(err) => {
                            return Err(format!("Future failed: {}", err));
                        }
                        _ => {
                            return Err(
                                "Cannot await pending future (no async runtime)".to_string()
                            );
                        }
                    }
                }

                // If it's a regular function call, just execute it synchronously
                Ok(future_val)
            }

            // Module System
            AstNode::ModuleDeclaration { name, body } => {
                // Create a new scope for the module
                self.push_scope();
                let result = self.eval_node(body)?;

                // Collect exports from module scope
                let mut exports = HashMap::new();
                if let Some(scope) = self.locals.last() {
                    exports = scope.clone();
                }

                self.pop_scope();

                // Store module
                let module = Value::Module {
                    name: name.clone(),
                    path: format!("<inline:{}>", name),
                    exports,
                };

                self.set_variable(name.clone(), module);
                Ok(result)
            }

            AstNode::ImportStatement { module_path, items } => {
                let path = module_path.join("/");
                let module = self.load_module(&path)?;

                if let Value::Module { exports, .. } = module {
                    if items.is_empty() {
                        // Import all exports
                        for (name, value) in exports {
                            self.set_variable(name, value);
                        }
                    } else {
                        // Import specific items
                        for item in items {
                            if let Some(value) = exports.get(item) {
                                self.set_variable(item.clone(), value.clone());
                            } else {
                                return Err(format!("Module {} does not export '{}'", path, item));
                            }
                        }
                    }
                }

                Ok(Value::None)
            }

            AstNode::UseStatement { path } => {
                // For now, just mark the use statement as processed
                let path_str = path.join(".");
                self.set_variable(format!("__use_{}", path_str), Value::Boolean(true));
                Ok(Value::None)
            }

            // Generics
            AstNode::GenericFunction {
                name,
                type_params: _,
                params,
                return_type: _,
                body,
            } => {
                // For now, treat generic functions like regular functions
                // In a full implementation, this would support type parameters
                let param_names: Vec<String> = params.iter().map(|(_, p)| p.clone()).collect();
                let func = Value::Function {
                    name: name.clone(),
                    params: param_names,
                    body: body.clone(),
                };
                self.set_variable(name.clone(), func);
                Ok(Value::None)
            }

            AstNode::GenericClass {
                name,
                type_params: _,
                parent,
                traits: _,
                fields,
                methods,
                static_fields,
                static_methods,
            } => {
                // For now, treat generic classes like regular classes
                // In a full implementation, this would support type parameters

                // Convert fields to HashMap
                let mut field_map = HashMap::new();
                for field in fields {
                    if let Some(default_value) = &field.default_value {
                        let val = self.eval_node(default_value)?;
                        field_map.insert(field.name.clone(), val);
                    } else {
                        field_map.insert(field.name.clone(), Value::None);
                    }
                }

                // Convert methods to HashMap
                let mut method_map = HashMap::new();
                for method in methods {
                    if let AstNode::FunctionDeclaration {
                        name, params, body, ..
                    } = method
                    {
                        let param_names: Vec<String> =
                            params.iter().map(|(_, p)| p.clone()).collect();
                        let func = Value::Function {
                            name: name.clone(),
                            params: param_names,
                            body: body.clone(),
                        };
                        method_map.insert(name.clone(), func);
                    }
                }

                // Convert static fields to HashMap
                let mut static_field_map = HashMap::new();
                for field in static_fields {
                    if let Some(default_value) = &field.default_value {
                        let val = self.eval_node(default_value)?;
                        static_field_map.insert(field.name.clone(), val);
                    } else {
                        static_field_map.insert(field.name.clone(), Value::None);
                    }
                }

                // Convert static methods to HashMap
                let mut static_method_map = HashMap::new();
                for method in static_methods {
                    if let AstNode::FunctionDeclaration {
                        name, params, body, ..
                    } = method
                    {
                        let param_names: Vec<String> =
                            params.iter().map(|(_, p)| p.clone()).collect();
                        let func = Value::Function {
                            name: name.clone(),
                            params: param_names,
                            body: body.clone(),
                        };
                        static_method_map.insert(name.clone(), func);
                    }
                }

                let class_value = Value::Class {
                    name: name.clone(),
                    class_type: None, // GenericClass doesn't have class_type modifier
                    parent: parent.clone(),
                    fields: field_map,
                    methods: method_map,
                    static_fields: static_field_map,
                    static_methods: static_method_map,
                };
                self.set_variable(name.clone(), class_value);
                Ok(Value::None)
            }
            // jnew_features: run body or stub
            AstNode::ExtendType {
                target_type: _,
                methods: _,
            } => Ok(Value::None),
            AstNode::PhantomDecl { name: _ } => Ok(Value::None),
            AstNode::MemoVarDeclaration {
                var_type: _,
                name,
                params,
                body,
            } => {
                let param_names: Vec<String> = params.iter().map(|(_, p)| p.clone()).collect();
                let func = Value::Function {
                    name: name.clone(),
                    params: param_names,
                    body: body.clone(),
                };
                self.set_variable(name.clone(), func);
                Ok(Value::None)
            }
            AstNode::FuzzLoop {
                var_type: _,
                var_name,
                range_opt: _,
                condition,
                body,
                else_body,
            } => {
                self.push_scope();
                let mut result = Value::None;
                for i in 0..100 {
                    self.set_variable(var_name.clone(), Value::Integer(i));
                    let cond = self.eval_node(condition)?;
                    if let Value::Boolean(false) = cond {
                        if let Some(eb) = else_body {
                            result = self.eval_node(eb)?;
                        }
                        break;
                    }
                    result = self.eval_node(body)?;
                }
                self.pop_scope();
                Ok(result)
            }
            AstNode::WithinLoop {
                duration_expr: _,
                loop_var,
                iterable,
                body,
                else_body,
            } => {
                if let (Some(var), Some(iter)) = (loop_var, iterable) {
                    let list = self.eval_node(iter)?;
                    if let Value::List(items) = list {
                        self.push_scope();
                        let mut last = Value::None;
                        for item in items {
                            self.set_variable(var.clone(), item);
                            last = self.eval_node(body)?;
                        }
                        self.pop_scope();
                        Ok(last)
                    } else {
                        self.eval_node(body)?;
                        Ok(Value::None)
                    }
                } else {
                    self.eval_node(body)?;
                    if let Some(eb) = else_body {
                        self.eval_node(eb)
                    } else {
                        Ok(Value::None)
                    }
                }
            }
            AstNode::RollbackBlock { retries: _, body } => self.eval_node(body),
            AstNode::RetryKeyword => Err("retry only valid inside rollback".to_string()),
            AstNode::RaceBlock { branches } => {
                if branches.is_empty() {
                    Ok(Value::None)
                } else {
                    self.eval_node(&branches[0].1)
                }
            }
            AstNode::BarrierDecl { name, count: _ } => {
                self.set_variable(name.clone(), Value::Integer(0));
                Ok(Value::None)
            }
            AstNode::RetryBlock {
                attempts: _,
                backoff: _,
                jitter: _,
                body,
            } => self.eval_node(body),
            AstNode::SecureBlock { body } => self.eval_node(body),
            AstNode::ComponentDecl {
                name,
                deps: _,
                fields: _,
                methods: _,
            } => {
                self.set_variable(name.clone(), Value::None);
                Ok(Value::None)
            }
            AstNode::ContractDecl { name, methods: _ } => {
                self.set_variable(name.clone(), Value::None);
                Ok(Value::None)
            }
            AstNode::WorkspaceBlock {
                members: _,
                rules: _,
            } => Ok(Value::None),
            AstNode::TaskDecl {
                name: _,
                needs: _,
                body,
            } => self.eval_node(body),
            AstNode::EnvSchema { name: _, fields: _ } => Ok(Value::None),
            AstNode::PacketDecl { name: _, fields: _ } => Ok(Value::None),
            AstNode::FloodLoop { start: _, body } => self.eval_node(body),
            AstNode::WindowLoop {
                var,
                iterable,
                size,
                shrink_condition,
                body,
            } => {
                let list = self.eval_node(iterable)?;
                if let Value::List(items) = list {
                    self.push_scope();
                    let mut last = Value::None;

                    // Determine window size
                    let window_size = if let Some(size_expr) = size {
                        match self.eval_node(size_expr)? {
                            Value::Integer(s) => s as usize,
                            _ => return Err("Window size must be an integer".to_string()),
                        }
                    } else {
                        1 // Default: growing window
                    };

                    if let Some(shrink_cond) = shrink_condition {
                        // Sliding window with shrink condition
                        let mut left = 0;
                        let mut right = 0;

                        while right < items.len() {
                            // Expand window
                            right += 1;
                            let slice: Vec<Value> = items[left..right].to_vec();
                            self.set_variable(var.clone(), Value::List(slice.clone()));

                            // Check shrink condition
                            let should_shrink = match self.eval_node(shrink_cond)? {
                                Value::Boolean(b) => b,
                                _ => false,
                            };

                            // Shrink window if condition met
                            while should_shrink && left < right {
                                left += 1;
                                let slice: Vec<Value> = items[left..right].to_vec();
                                self.set_variable(var.clone(), Value::List(slice));

                                let still_shrink = match self.eval_node(shrink_cond)? {
                                    Value::Boolean(b) => b,
                                    _ => false,
                                };
                                if !still_shrink {
                                    break;
                                }
                            }

                            last = self.eval_node(body)?;
                        }
                    } else if window_size == 1 {
                        // Growing window (original behavior)
                        for i in 0..items.len() {
                            let slice: Vec<Value> = items[..=i].to_vec();
                            self.set_variable(var.clone(), Value::List(slice));
                            last = self.eval_node(body)?;
                        }
                    } else {
                        // Fixed-size sliding window
                        for i in 0..=(items.len().saturating_sub(window_size)) {
                            let slice: Vec<Value> = items[i..i + window_size].to_vec();
                            self.set_variable(var.clone(), Value::List(slice));
                            last = self.eval_node(body)?;
                        }
                    }

                    self.pop_scope();
                    Ok(last)
                } else {
                    Err("window loop requires a list".to_string())
                }
            }
            AstNode::SolverBlock {
                name: _,
                options: _,
                body,
            } => self.eval_node(body),

            // New algorithm helper nodes
            AstNode::IntervalLiteral { start, end } => {
                let start_val = self.eval_node(start)?;
                let end_val = self.eval_node(end)?;
                match (start_val, end_val) {
                    (Value::Integer(s), Value::Integer(e)) => Ok(Value::Interval(s, e)),
                    _ => Err("Interval requires integer start and end".to_string()),
                }
            }

            AstNode::GroupBy { collection, key_fn } => {
                let coll = self.eval_node(collection)?;
                if let Value::List(items) = coll {
                    let mut groups: HashMap<String, Vec<Value>> = HashMap::new();

                    for item in items {
                        // Call the key function with the item
                        let key_val = match self.eval_node(key_fn)? {
                            Value::Function { params, body, .. } => {
                                if params.len() != 1 {
                                    return Err(
                                        "group_by key function must take exactly 1 parameter"
                                            .to_string(),
                                    );
                                }
                                self.push_scope();
                                self.set_variable(params[0].clone(), item.clone());
                                let result = self.eval_node(&body)?;
                                self.pop_scope();
                                result
                            }
                            _ => return Err("group_by requires a function as key".to_string()),
                        };

                        // Convert key to string
                        let key_str = key_val.to_string();
                        groups.entry(key_str).or_default().push(item);
                    }

                    // Convert to dict of lists
                    let mut result = HashMap::new();
                    for (key, values) in groups {
                        result.insert(key, Value::List(values));
                    }
                    Ok(Value::Dict(result))
                } else {
                    Err("group_by requires a list".to_string())
                }
            }

            AstNode::Partition {
                collection,
                predicate,
            } => {
                let coll = self.eval_node(collection)?;
                if let Value::List(items) = coll {
                    let mut true_items = Vec::new();
                    let mut false_items = Vec::new();

                    for item in items {
                        // Call the predicate function with the item
                        let pred_result = match self.eval_node(predicate)? {
                            Value::Function { params, body, .. } => {
                                if params.len() != 1 {
                                    return Err(
                                        "partition predicate must take exactly 1 parameter"
                                            .to_string(),
                                    );
                                }
                                self.push_scope();
                                self.set_variable(params[0].clone(), item.clone());
                                let result = self.eval_node(&body)?;
                                self.pop_scope();
                                result
                            }
                            _ => {
                                return Err("partition requires a function as predicate".to_string())
                            }
                        };

                        match pred_result {
                            Value::Boolean(true) => true_items.push(item),
                            Value::Boolean(false) => false_items.push(item),
                            _ => {
                                return Err("partition predicate must return a boolean".to_string())
                            }
                        }
                    }

                    // Return tuple of (true_items, false_items)
                    Ok(Value::Tuple(vec![
                        Value::List(true_items),
                        Value::List(false_items),
                    ]))
                } else {
                    Err("partition requires a list".to_string())
                }
            }

            AstNode::DeferAttach { resource, cleanup } => {
                let resource_val = self.eval_node(resource)?;
                if let Some(frame) = self.defer_stack.last_mut() {
                    frame.push((*cleanup.clone(), Some(resource_val.clone())));
                }
                Ok(resource_val)
            }

            // Generators and comprehensions
            AstNode::Generator { params, body } => {
                // Create a generator function that yields values
                Ok(Value::Function {
                    name: "<generator>".to_string(),
                    params: params.clone(),
                    body: body.clone(),
                })
            }

            AstNode::Yield { value } => {
                // Yield a value from a generator
                let val = self.eval_node(value)?;
                // In a real implementation, this would suspend execution
                // For now, we'll just return the value
                Ok(val)
            }

            // OOB Features Implementation
            AstNode::FlowBlock { bindings } => {
                // Create reactive scope with dependency tracking
                let flow_vars: HashMap<String, Value> = HashMap::new();
                let mut derived_vars = HashMap::new();
                
                // First pass: evaluate direct assignments
                for (name, binding_type, expr) in bindings {
                    match binding_type {
                        crate::parser::FlowBindingType::Direct => {
                            let value = self.eval_node(expr)?;
                            self.set_variable(name.clone(), value);
                        }
                        crate::parser::FlowBindingType::Derived => {
                            // Store expression for lazy evaluation
                            derived_vars.insert(name.clone(), expr.clone());
                        }
                    }
                }
                
                // Second pass: evaluate derived values
                for (name, expr) in derived_vars {
                    let value = self.eval_node(&expr)?;
                    self.set_variable(name.clone(), value);
                }
                
                // Store flow_vars for future reference
                let _ = flow_vars;
                Ok(Value::None)
            }

            AstNode::ProbeDeclaration { target, hooks } => {
                // Attach runtime inspection hooks to a variable
                // For now, just evaluate hooks and store them
                // In a full implementation, this would register callbacks
                for (_hook_name, callback) in hooks {
                    // Store probe hook (simplified - would need proper hook system)
                    let _hook_fn = self.eval_node(callback)?;
                    // TODO: Register hook for target variable with _hook_name
                }
                // Store target for future reference
                let _ = target;
                Ok(Value::None)
            }

            AstNode::FuseHint { target } => {
                // Compile-time fusion hint - just evaluate the target for now
                // In a full compiler, this would mark the function for aggressive inlining
                self.eval_node(target)
            }

            AstNode::FusePipeline { stages } => {
                // Fuse pipeline stages into single operation
                // For interpreter, just execute sequentially
                let mut result = Value::None;
                for stage in stages {
                    result = self.eval_node(stage)?;
                }
                Ok(result)
            }

            AstNode::VeilBlock { body } => {
                // Execute block in constant-time mode
                // For interpreter, just execute normally
                // In a full implementation, this would use constant-time operations
                self.eval_node(body)
            }

            AstNode::VeilGet { collection, key } => {
                // Constant-time get operation
                let coll = self.eval_node(collection)?;
                let k = self.eval_node(key)?;
                
                match coll {
                    Value::Dict(dict) => {
                        let key_str = match k {
                            Value::String(s) => s,
                            Value::Integer(i) => i.to_string(),
                            _ => return Err("Veil get requires string or integer key".to_string()),
                        };
                        // In constant-time implementation, would scan entire dict
                        Ok(dict.get(&key_str).cloned().unwrap_or(Value::None))
                    }
                    Value::List(list) => {
                        if let Value::Integer(idx) = k {
                            let index = if idx < 0 {
                                (list.len() as i64 + idx) as usize
                            } else {
                                idx as usize
                            };
                            // In constant-time implementation, would scan entire list
                            Ok(list.get(index).cloned().unwrap_or(Value::None))
                        } else {
                            Err("Veil get on list requires integer index".to_string())
                        }
                    }
                    _ => Err("Veil get requires dict or list".to_string()),
                }
            }

            AstNode::VeilSet { collection, key, value } => {
                // Constant-time set operation
                let coll = self.eval_node(collection)?;
                let k = self.eval_node(key)?;
                let v = self.eval_node(value)?;
                
                match coll {
                    Value::Dict(mut dict) => {
                        let key_str = match k {
                            Value::String(s) => s,
                            Value::Integer(i) => i.to_string(),
                            _ => return Err("Veil set requires string or integer key".to_string()),
                        };
                        dict.insert(key_str, v);
                        Ok(Value::Dict(dict))
                    }
                    Value::List(mut list) => {
                        if let Value::Integer(idx) = k {
                            let index = if idx < 0 {
                                (list.len() as i64 + idx) as usize
                            } else {
                                idx as usize
                            };
                            if index < list.len() {
                                list[index] = v;
                            }
                            Ok(Value::List(list))
                        } else {
                            Err("Veil set on list requires integer index".to_string())
                        }
                    }
                    Value::SmallVec { cap, mut elements } => {
                        if let Value::Integer(idx) = k {
                            let index = if idx < 0 {
                                (elements.len() as i64 + idx) as usize
                            } else {
                                idx as usize
                            };
                            if index < elements.len() {
                                elements[index] = v;
                            }
                            Ok(Value::SmallVec { cap, elements })
                        } else {
                            Err("Veil set on smallvec requires integer index".to_string())
                        }
                    }
                    _ => Err("Veil set requires dict or list".to_string()),
                }
            }

            AstNode::WarpTemplate { name, params, body } => {
                // Compile-time template expansion
                // Store as a function for now
                Ok(Value::Function {
                    name: name.clone(),
                    params: params.iter().map(|(_, n)| n.clone()).collect(),
                    body: body.clone(),
                })
            }

            AstNode::GhostDeclaration { var_type: _, name, value } => {
                // Ghost variable - only exists in debug builds
                // For interpreter, always evaluate
                #[cfg(debug_assertions)]
                {
                    let val = self.eval_node(value)?;
                    self.set_variable(name.clone(), val);
                }
                #[cfg(not(debug_assertions))]
                {
                    // In release, ghost variables are stripped
                    let _ = (name, value);
                }
                Ok(Value::None)
            }
        }
    }
}
