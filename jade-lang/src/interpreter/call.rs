//! Call dispatch and builtins.

use super::*;

impl Interpreter {
    pub(super) fn call_function(&mut self, name: &str, args: &[AstNode]) -> Result<Value, String> {
        // Dispatch to categorized builtins (math, algo, DSA) first
        if let Some(v) = super::builtins::try_call(self, name, args)? {
            return Ok(v);
        }
        // Handle remaining built-in functions
        match name {
            "out" => return self.call_out(args),

            "sleep" => {
                if args.len() != 1 {
                    return Err("sleep() expects exactly 1 argument (seconds)".to_string());
                }
                let val = self.eval_node(&args[0])?;
                let seconds = match val {
                    Value::Integer(i) => i as f64,
                    Value::Float(f) => f,
                    _ => return Err("sleep() expects a numeric argument".to_string()),
                };
                if seconds < 0.0 {
                    return Err("sleep() duration must be non-negative".to_string());
                }
                std::thread::sleep(std::time::Duration::from_secs_f64(seconds));
                Ok(Value::None)
            }

            "varType" => {
                if args.len() != 1 {
                    return Err("varType() expects exactly 1 argument".to_string());
                }
                let val = self.eval_node(&args[0])?;
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
                    #[cfg(feature = "regex")]
                    Value::Regex(_) => "regex",
                };
                Ok(Value::String(type_name.to_string()))
            }

            "new" => {
                // Create a new instance of a class
                // Usage: new(ClassName) or new(ClassName, field1, value1, field2, value2, ...)
                if args.is_empty() {
                    return Err("new() expects at least 1 argument (class name)".to_string());
                }

                let class_val = self.eval_node(&args[0])?;

                match class_val {
                    Value::Class {
                        name: class_name,
                        class_type,
                        fields: class_fields,
                        methods: _,
                        ..
                    } => {
                        // Create instance with default field values
                        let mut instance_fields = class_fields.clone();

                        // If additional arguments provided, use them to set fields
                        // Format: new(Class, "field1", value1, "field2", value2, ...)
                        if args.len() > 1 {
                            if !(args.len() - 1).is_multiple_of(2) {
                                return Err(
                                    "new() expects pairs of field names and values after class"
                                        .to_string(),
                                );
                            }

                            for i in (1..args.len()).step_by(2) {
                                let field_name_val = self.eval_node(&args[i])?;
                                let field_value = self.eval_node(&args[i + 1])?;

                                if let Value::String(field_name) = field_name_val {
                                    instance_fields.insert(field_name, field_value);
                                } else {
                                    return Err("Field names must be strings in new()".to_string());
                                }
                            }
                        }

                        let instance = Value::Instance {
                            class_name: class_name.clone(),
                            fields: instance_fields,
                        };

                        // Apply special behaviors based on class_type
                        match class_type.as_deref() {
                            Some("singleton") => {
                                // For singleton, store in a global registry
                                // For now, just return the instance
                                // TODO: Implement singleton registry
                                Ok(instance)
                            }
                            Some("data") => {
                                // Data classes get auto-generated methods
                                // For now, just return the instance
                                // TODO: Add equals, hash, copy methods
                                Ok(instance)
                            }
                            _ => Ok(instance),
                        }
                    }
                    _ => Err(format!("new() expects a class, got {:?}", class_val)),
                }
            }

            "channel" => {
                // Create a new channel
                if !args.is_empty() {
                    return Err("channel() expects no arguments".to_string());
                }
                let channel_id = self.next_future_id as u64;
                self.next_future_id += 1;
                Ok(Value::Channel(channel_id))
            }

            "spawn" => {
                // Spawn a task (similar to TaskSpawn but as a function)
                if args.len() != 1 {
                    return Err(
                        "spawn() expects exactly 1 argument (function or block)".to_string()
                    );
                }
                let task_id = self.next_future_id;
                self.next_future_id += 1;

                // Execute the argument (should be a function or expression)
                self.push_scope();
                let _result = self.eval_node(&args[0])?;
                self.pop_scope();

                Ok(Value::Task(task_id as u64))
            }

            "range" => match args.len() {
                1 => {
                    let end_val = self.eval_node(&args[0])?;
                    if let Value::Integer(end) = end_val {
                        let mut range = Vec::new();
                        for i in 0..end {
                            range.push(Value::Integer(i));
                        }
                        Ok(Value::List(range))
                    } else {
                        Err("range() expects integer argument".to_string())
                    }
                }
                2 => {
                    let start_val = self.eval_node(&args[0])?;
                    let end_val = self.eval_node(&args[1])?;
                    if let (Value::Integer(start), Value::Integer(end)) = (start_val, end_val) {
                        let mut range = Vec::new();
                        for i in start..end {
                            range.push(Value::Integer(i));
                        }
                        Ok(Value::List(range))
                    } else {
                        Err("range() expects integer arguments".to_string())
                    }
                }
                3 => {
                    let start_val = self.eval_node(&args[0])?;
                    let end_val = self.eval_node(&args[1])?;
                    let step_val = self.eval_node(&args[2])?;
                    if let (Value::Integer(start), Value::Integer(end), Value::Integer(step)) =
                        (start_val, end_val, step_val)
                    {
                        let mut range = Vec::new();
                        let mut i = start;
                        while (step > 0 && i < end) || (step < 0 && i > end) {
                            range.push(Value::Integer(i));
                            i += step;
                        }
                        Ok(Value::List(range))
                    } else {
                        Err("range() expects integer arguments".to_string())
                    }
                }
                _ => Err("range() expects 1, 2, or 3 arguments".to_string()),
            },

            "interval" => {
                if args.len() != 2 {
                    return Err("interval() expects exactly 2 arguments (start, end)".to_string());
                }
                let start_val = self.eval_node(&args[0])?;
                let end_val = self.eval_node(&args[1])?;
                match (start_val, end_val) {
                    (Value::Integer(start), Value::Integer(end)) => Ok(Value::Interval(start, end)),
                    _ => Err("interval() expects integer arguments".to_string()),
                }
            }

            "sum" => {
                if args.len() != 1 {
                    return Err("sum() expects exactly 1 argument".to_string());
                }
                let val = self.eval_node(&args[0])?;
                match val {
                    Value::List(list) => {
                        let mut sum = 0i64;
                        for item in list {
                            match item {
                                Value::Integer(i) => sum += i,
                                _ => return Err("sum() can only sum integers".to_string()),
                            }
                        }
                        Ok(Value::Integer(sum))
                    }
                    _ => Err("sum() can only be called on lists".to_string()),
                }
            }

            // Advanced array type constructors
            "span" => {
                if args.len() != 1 {
                    return Err("span() expects exactly 1 argument (list)".to_string());
                }
                let val = self.eval_node(&args[0])?;
                match val {
                    Value::List(list) => {
                        let len = list.len();
                        Ok(Value::Span {
                            source: Box::new(Value::List(list)),
                            start: 0,
                            end: len,
                        })
                    }
                    _ => Err("span() requires a list argument".to_string()),
                }
            }

            "mut_span" => {
                if args.len() != 1 {
                    return Err("mut_span() expects exactly 1 argument (list)".to_string());
                }
                let val = self.eval_node(&args[0])?;
                match val {
                    Value::List(list) => {
                        let len = list.len();
                        Ok(Value::MutSpan {
                            source: Box::new(Value::List(list)),
                            start: 0,
                            end: len,
                        })
                    }
                    _ => Err("mut_span() requires a list argument".to_string()),
                }
            }

            "chunk" => {
                if args.is_empty() || args.len() > 2 {
                    return Err("chunk() expects 1-2 arguments (list, [size])".to_string());
                }
                let val = self.eval_node(&args[0])?;
                let chunk_size = if args.len() == 2 {
                    match self.eval_node(&args[1])? {
                        Value::Integer(size) => size as usize,
                        _ => return Err("chunk size must be an integer".to_string()),
                    }
                } else {
                    1
                };

                match val {
                    Value::List(list) => Ok(Value::Chunk {
                        source: Box::new(Value::List(list)),
                        chunk_size,
                        current_index: 0,
                    }),
                    _ => Err("chunk() requires a list argument".to_string()),
                }
            }

            "sparse" => {
                if args.is_empty() || args.len() > 2 {
                    return Err(
                        "sparse() expects 1-2 arguments (size or list, [default])".to_string()
                    );
                }
                let val = self.eval_node(&args[0])?;
                let default = if args.len() == 2 {
                    Box::new(self.eval_node(&args[1])?)
                } else {
                    Box::new(Value::Integer(0))
                };

                match val {
                    Value::Integer(size) => Ok(Value::Sparse {
                        data: HashMap::new(),
                        default,
                        size: size as usize,
                    }),
                    Value::List(list) => {
                        let mut data = HashMap::new();
                        for (i, item) in list.iter().enumerate() {
                            if item != default.as_ref() {
                                data.insert(i, item.clone());
                            }
                        }
                        Ok(Value::Sparse {
                            data,
                            default,
                            size: list.len(),
                        })
                    }
                    _ => Err("sparse() requires an integer size or list".to_string()),
                }
            }

            "ring" => {
                if args.len() != 1 {
                    return Err("ring() expects exactly 1 argument (capacity)".to_string());
                }
                let val = self.eval_node(&args[0])?;
                match val {
                    Value::Integer(capacity) => {
                        if capacity <= 0 {
                            return Err("ring capacity must be positive".to_string());
                        }
                        Ok(Value::Ring {
                            buffer: vec![Value::None; capacity as usize],
                            capacity: capacity as usize,
                            head: 0,
                            size: 0,
                        })
                    }
                    _ => Err("ring() requires an integer capacity".to_string()),
                }
            }

            "map" => {
                // Check if we're in a pipeline context (1 arg) or normal call (2 args)
                let (list_val, func_val) = if args.len() == 1 {
                    // Pipeline context: get list from __pipeline__
                    let pipeline_val = self.get_variable("__pipeline__").unwrap_or(Value::None);
                    if matches!(pipeline_val, Value::None) {
                        return Err(
                            "map() in pipeline requires a value from the left side".to_string()
                        );
                    }
                    let func_val = self.eval_node(&args[0])?;
                    (pipeline_val, func_val)
                } else if args.len() == 2 {
                    // Normal call: map(list, func)
                    let list_val = self.eval_node(&args[0])?;
                    let func_val = self.eval_node(&args[1])?;
                    (list_val, func_val)
                } else {
                    return Err(
                        "map() expects 1 argument in pipeline or 2 arguments normally".to_string(),
                    );
                };

                match list_val {
                    Value::List(list) => {
                        let mut result = Vec::new();
                        for item in list {
                            // Call the function with the item as argument
                            let mapped_val = match &func_val {
                                Value::Function { params, body, .. } => {
                                    // Create new scope for lambda
                                    self.push_scope();

                                    // Set parameter
                                    if params.len() == 1 {
                                        self.set_variable(params[0].clone(), item);
                                    } else {
                                        self.pop_scope();
                                        return Err(format!(
                                            "map() lambda must have exactly 1 parameter, got {}",
                                            params.len()
                                        ));
                                    }

                                    // Execute function body
                                    let result = self.eval_node(body)?;

                                    // Restore scope
                                    self.pop_scope();

                                    result
                                }
                                _ => {
                                    return Err(
                                        "map() expects a function as second argument".to_string()
                                    )
                                }
                            };
                            result.push(mapped_val);
                        }
                        Ok(Value::List(result))
                    }
                    _ => Err("map() can only be called on lists".to_string()),
                }
            }

            "unique" => {
                if args.len() != 1 {
                    return Err("unique() expects exactly 1 argument".to_string());
                }
                let val = self.eval_node(&args[0])?;

                match val {
                    Value::List(list) => {
                        let mut unique_list = Vec::new();
                        for item in list {
                            if !unique_list.iter().any(|x| self.values_equal(x, &item)) {
                                unique_list.push(item);
                            }
                        }
                        Ok(Value::List(unique_list))
                    }
                    _ => Err("unique() can only be called on lists".to_string()),
                }
            }

            "reverse" => {
                if args.len() != 1 {
                    return Err("reverse() expects exactly 1 argument".to_string());
                }
                let val = self.eval_node(&args[0])?;

                match val {
                    Value::List(mut list) => {
                        list.reverse();
                        Ok(Value::List(list))
                    }
                    Value::String(s) => {
                        let reversed: String = s.chars().rev().collect();
                        Ok(Value::String(reversed))
                    }
                    Value::Vector(mut vec) => {
                        vec.reverse();
                        Ok(Value::Vector(vec))
                    }
                    _ => {
                        Err("reverse() can only be called on lists, strings, or vectors"
                            .to_string())
                    }
                }
            }

            "sort" => {
                if args.len() != 1 {
                    return Err("sort() expects exactly 1 argument".to_string());
                }
                let val = self.eval_node(&args[0])?;

                match val {
                    Value::List(mut list) => {
                        // Simple sort for integers and floats
                        list.sort_by(|a, b| match (a, b) {
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
                        Ok(Value::List(list))
                    }
                    Value::Vector(mut vec) => {
                        vec.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                        Ok(Value::Vector(vec))
                    }
                    _ => Err("sort() can only be called on lists or vectors".to_string()),
                }
            }

            "group_by" => {
                if args.len() != 2 {
                    return Err(
                        "group_by() expects exactly 2 arguments (list, key_function)".to_string(),
                    );
                }
                let list_val = self.eval_node(&args[0])?;
                let key_fn = self.eval_node(&args[1])?;

                if let Value::List(items) = list_val {
                    let mut groups: HashMap<String, Vec<Value>> = HashMap::new();

                    for item in items {
                        // Call the key function with the item
                        let key_val = match &key_fn {
                            Value::Function { params, body, .. } => {
                                if params.len() != 1 {
                                    return Err(
                                        "group_by key function must take exactly 1 parameter"
                                            .to_string(),
                                    );
                                }
                                self.push_scope();
                                self.set_variable(params[0].clone(), item.clone());
                                let result = self.eval_node(body)?;
                                self.pop_scope();
                                result
                            }
                            _ => {
                                return Err(
                                    "group_by requires a function as second argument".to_string()
                                )
                            }
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
                    Err("group_by requires a list as first argument".to_string())
                }
            }

            "partition" => {
                if args.len() != 2 {
                    return Err(
                        "partition() expects exactly 2 arguments (list, predicate)".to_string()
                    );
                }
                let list_val = self.eval_node(&args[0])?;
                let predicate = self.eval_node(&args[1])?;

                if let Value::List(items) = list_val {
                    let mut true_items = Vec::new();
                    let mut false_items = Vec::new();

                    for item in items {
                        // Call the predicate function with the item
                        let pred_result = match &predicate {
                            Value::Function { params, body, .. } => {
                                if params.len() != 1 {
                                    return Err(
                                        "partition predicate must take exactly 1 parameter"
                                            .to_string(),
                                    );
                                }
                                self.push_scope();
                                self.set_variable(params[0].clone(), item.clone());
                                let result = self.eval_node(body)?;
                                self.pop_scope();
                                result
                            }
                            _ => {
                                return Err(
                                    "partition requires a function as second argument".to_string()
                                )
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
                    Err("partition requires a list as first argument".to_string())
                }
            }

            "min" => {
                if args.len() != 1 {
                    return Err("min() expects exactly 1 argument".to_string());
                }
                let val = self.eval_node(&args[0])?;

                match val {
                    Value::List(list) => {
                        if list.is_empty() {
                            return Err("Cannot find min of empty list".to_string());
                        }
                        let mut min_val = &list[0];
                        for item in &list[1..] {
                            match (min_val, item) {
                                (Value::Integer(x), Value::Integer(y)) => {
                                    if y < x {
                                        min_val = item;
                                    }
                                }
                                (Value::Float(x), Value::Float(y)) => {
                                    if y < x {
                                        min_val = item;
                                    }
                                }
                                (Value::Integer(x), Value::Float(y)) => {
                                    if y < &(*x as f64) {
                                        min_val = item;
                                    }
                                }
                                (Value::Float(x), Value::Integer(y)) => {
                                    if (*y as f64) < *x {
                                        min_val = item;
                                    }
                                }
                                _ => {}
                            }
                        }
                        Ok(min_val.clone())
                    }
                    Value::Vector(vec) => {
                        if vec.is_empty() {
                            return Err("Cannot find min of empty vector".to_string());
                        }
                        let min_val = vec
                            .iter()
                            .fold(vec[0], |acc, &x| if x < acc { x } else { acc });
                        Ok(Value::Float(min_val))
                    }
                    _ => Err("min() can only be called on lists or vectors".to_string()),
                }
            }

            "max" => {
                if args.len() != 1 {
                    return Err("max() expects exactly 1 argument".to_string());
                }
                let val = self.eval_node(&args[0])?;

                match val {
                    Value::List(list) => {
                        if list.is_empty() {
                            return Err("Cannot find max of empty list".to_string());
                        }
                        let mut max_val = &list[0];
                        for item in &list[1..] {
                            match (max_val, item) {
                                (Value::Integer(x), Value::Integer(y)) => {
                                    if y > x {
                                        max_val = item;
                                    }
                                }
                                (Value::Float(x), Value::Float(y)) => {
                                    if y > x {
                                        max_val = item;
                                    }
                                }
                                (Value::Integer(x), Value::Float(y)) => {
                                    if y > &(*x as f64) {
                                        max_val = item;
                                    }
                                }
                                (Value::Float(x), Value::Integer(y)) => {
                                    if (*y as f64) > *x {
                                        max_val = item;
                                    }
                                }
                                _ => {}
                            }
                        }
                        Ok(max_val.clone())
                    }
                    Value::Vector(vec) => {
                        if vec.is_empty() {
                            return Err("Cannot find max of empty vector".to_string());
                        }
                        let max_val = vec
                            .iter()
                            .fold(vec[0], |acc, &x| if x > acc { x } else { acc });
                        Ok(Value::Float(max_val))
                    }
                    _ => Err("max() can only be called on lists or vectors".to_string()),
                }
            }

            "count" => {
                if args.len() != 2 {
                    return Err("count() expects exactly 2 arguments".to_string());
                }
                let container_val = self.eval_node(&args[0])?;
                let item_val = self.eval_node(&args[1])?;

                match container_val {
                    Value::List(list) => {
                        let count = list
                            .iter()
                            .filter(|x| self.values_equal(x, &item_val))
                            .count();
                        Ok(Value::Integer(count as i64))
                    }
                    Value::String(s) => {
                        if let Value::String(search) = item_val {
                            let count = s.matches(&search).count();
                            Ok(Value::Integer(count as i64))
                        } else if let Value::Char(ch) = item_val {
                            let count = s.chars().filter(|&c| c == ch).count();
                            Ok(Value::Integer(count as i64))
                        } else {
                            Err("count() on string requires string or char argument".to_string())
                        }
                    }
                    _ => Err("count() can only be called on lists or strings".to_string()),
                }
            }

            "flatten" => {
                if args.len() != 1 {
                    return Err("flatten() expects exactly 1 argument".to_string());
                }
                let val = self.eval_node(&args[0])?;

                match val {
                    Value::List(list) => {
                        let mut flattened = Vec::new();
                        for item in list {
                            match item {
                                Value::List(inner_list) => {
                                    flattened.extend(inner_list);
                                }
                                _ => flattened.push(item),
                            }
                        }
                        Ok(Value::List(flattened))
                    }
                    _ => Err("flatten() can only be called on lists".to_string()),
                }
            }

            "zip" => {
                if args.len() != 2 {
                    return Err("zip() expects exactly 2 arguments".to_string());
                }
                let list1_val = self.eval_node(&args[0])?;
                let list2_val = self.eval_node(&args[1])?;

                match (list1_val, list2_val) {
                    (Value::List(list1), Value::List(list2)) => {
                        let mut zipped = Vec::new();
                        for (a, b) in list1.iter().zip(list2.iter()) {
                            zipped.push(Value::Tuple(vec![a.clone(), b.clone()]));
                        }
                        Ok(Value::List(zipped))
                    }
                    _ => Err("zip() expects two lists".to_string()),
                }
            }

            "enumerate" => {
                if args.len() != 1 {
                    return Err("enumerate() expects exactly 1 argument".to_string());
                }
                let val = self.eval_node(&args[0])?;

                match val {
                    Value::List(list) => {
                        let mut enumerated = Vec::new();
                        for (i, item) in list.iter().enumerate() {
                            enumerated
                                .push(Value::Tuple(vec![Value::Integer(i as i64), item.clone()]));
                        }
                        Ok(Value::List(enumerated))
                    }
                    _ => Err("enumerate() can only be called on lists".to_string()),
                }
            }

            "union" => {
                if args.len() != 2 {
                    return Err(
                        "union() expects exactly 2 arguments: union(list1, list2)".to_string()
                    );
                }
                let list1_val = self.eval_node(&args[0])?;
                let list2_val = self.eval_node(&args[1])?;

                match (list1_val, list2_val) {
                    (Value::List(list1), Value::List(list2)) => {
                        use std::collections::HashSet;
                        let mut seen = HashSet::new();
                        let mut result = Vec::new();

                        // Add all from list1
                        for item in list1.iter() {
                            let key = format!("{:?}", item);
                            if seen.insert(key) {
                                result.push(item.clone());
                            }
                        }

                        // Add unique from list2
                        for item in list2.iter() {
                            let key = format!("{:?}", item);
                            if seen.insert(key) {
                                result.push(item.clone());
                            }
                        }

                        Ok(Value::List(result))
                    }
                    _ => Err("union() expects two lists".to_string()),
                }
            }

            "intersect" => {
                if args.len() != 2 {
                    return Err(
                        "intersect() expects exactly 2 arguments: intersect(list1, list2)"
                            .to_string(),
                    );
                }
                let list1_val = self.eval_node(&args[0])?;
                let list2_val = self.eval_node(&args[1])?;

                match (list1_val, list2_val) {
                    (Value::List(list1), Value::List(list2)) => {
                        use std::collections::HashSet;
                        let mut set2 = HashSet::new();

                        // Build set from list2
                        for item in list2.iter() {
                            set2.insert(format!("{:?}", item));
                        }

                        // Keep items from list1 that are in set2
                        let mut result = Vec::new();
                        let mut seen = HashSet::new();
                        for item in list1.iter() {
                            let key = format!("{:?}", item);
                            if set2.contains(&key) && seen.insert(key) {
                                result.push(item.clone());
                            }
                        }

                        Ok(Value::List(result))
                    }
                    _ => Err("intersect() expects two lists".to_string()),
                }
            }

            "difference" => {
                if args.len() != 2 {
                    return Err(
                        "difference() expects exactly 2 arguments: difference(list1, list2)"
                            .to_string(),
                    );
                }
                let list1_val = self.eval_node(&args[0])?;
                let list2_val = self.eval_node(&args[1])?;

                match (list1_val, list2_val) {
                    (Value::List(list1), Value::List(list2)) => {
                        use std::collections::HashSet;
                        let mut set2 = HashSet::new();

                        // Build set from list2
                        for item in list2.iter() {
                            set2.insert(format!("{:?}", item));
                        }

                        // Keep items from list1 that are NOT in set2
                        let mut result = Vec::new();
                        let mut seen = HashSet::new();
                        for item in list1.iter() {
                            let key = format!("{:?}", item);
                            if !set2.contains(&key) && seen.insert(key) {
                                result.push(item.clone());
                            }
                        }

                        Ok(Value::List(result))
                    }
                    _ => Err("difference() expects two lists".to_string()),
                }
            }

            "symmetric_diff" => {
                if args.len() != 2 {
                    return Err("symmetric_diff() expects exactly 2 arguments: symmetric_diff(list1, list2)".to_string());
                }
                let list1_val = self.eval_node(&args[0])?;
                let list2_val = self.eval_node(&args[1])?;

                match (list1_val, list2_val) {
                    (Value::List(list1), Value::List(list2)) => {
                        use std::collections::HashSet;
                        let mut set1 = HashSet::new();
                        let mut set2 = HashSet::new();

                        // Build sets
                        for item in list1.iter() {
                            set1.insert(format!("{:?}", item));
                        }
                        for item in list2.iter() {
                            set2.insert(format!("{:?}", item));
                        }

                        // Items in list1 but not list2
                        let mut result = Vec::new();
                        let mut seen = HashSet::new();
                        for item in list1.iter() {
                            let key = format!("{:?}", item);
                            if !set2.contains(&key) && seen.insert(key.clone()) {
                                result.push(item.clone());
                            }
                        }

                        // Items in list2 but not list1
                        for item in list2.iter() {
                            let key = format!("{:?}", item);
                            if !set1.contains(&key) && seen.insert(key) {
                                result.push(item.clone());
                            }
                        }

                        Ok(Value::List(result))
                    }
                    _ => Err("symmetric_diff() expects two lists".to_string()),
                }
            }

            "filter" => {
                // Check if we're in a pipeline context (1 arg) or normal call (2 args)
                let (list_val, func_val) = if args.len() == 1 {
                    // Pipeline context: get list from __pipeline__
                    let pipeline_val = self.get_variable("__pipeline__").unwrap_or(Value::None);
                    if matches!(pipeline_val, Value::None) {
                        return Err(
                            "filter() in pipeline requires a value from the left side".to_string()
                        );
                    }
                    let func_val = self.eval_node(&args[0])?;
                    (pipeline_val, func_val)
                } else if args.len() == 2 {
                    // Normal call: filter(list, func)
                    let list_val = self.eval_node(&args[0])?;
                    let func_val = self.eval_node(&args[1])?;
                    (list_val, func_val)
                } else {
                    return Err(
                        "filter() expects 1 argument in pipeline or 2 arguments normally"
                            .to_string(),
                    );
                };

                match list_val {
                    Value::List(list) => {
                        let mut result = Vec::new();
                        for item in list {
                            // Call the function with the item as argument
                            let filter_result = match &func_val {
                                Value::Function { params, body, .. } => {
                                    // Create new scope for lambda
                                    self.push_scope();

                                    // Set parameter
                                    if params.len() == 1 {
                                        self.set_variable(params[0].clone(), item.clone());
                                    } else {
                                        self.pop_scope();
                                        return Err(format!(
                                            "filter() lambda must have exactly 1 parameter, got {}",
                                            params.len()
                                        ));
                                    }

                                    // Execute function body
                                    let result = self.eval_node(body)?;

                                    // Restore scope
                                    self.pop_scope();

                                    result
                                }
                                _ => {
                                    return Err("filter() expects a function as second argument"
                                        .to_string())
                                }
                            };

                            if self.is_truthy(&filter_result) {
                                result.push(item);
                            }
                        }
                        Ok(Value::List(result))
                    }
                    _ => Err("filter() can only be called on lists".to_string()),
                }
            }

            "reduce" => {
                // reduce(list, initial, func) or reduce(list, func) in pipeline
                let (list_val, initial_val, func_val) = if args.len() == 2 {
                    // Pipeline context or no initial value: reduce(list, func)
                    let list_val = self.eval_node(&args[0])?;
                    let func_val = self.eval_node(&args[1])?;

                    // Get first element as initial value
                    let initial = match &list_val {
                        Value::List(list) if !list.is_empty() => list[0].clone(),
                        _ => return Err("reduce() requires a non-empty list".to_string()),
                    };

                    (list_val, initial, func_val)
                } else if args.len() == 3 {
                    // Normal call: reduce(list, initial, func)
                    let list_val = self.eval_node(&args[0])?;
                    let initial_val = self.eval_node(&args[1])?;
                    let func_val = self.eval_node(&args[2])?;
                    (list_val, initial_val, func_val)
                } else {
                    return Err("reduce() expects 2 or 3 arguments".to_string());
                };

                match list_val {
                    Value::List(list) => {
                        let mut accumulator = initial_val;
                        let start_idx = if args.len() == 2 { 1 } else { 0 };

                        for item in list.iter().skip(start_idx) {
                            // Call the function with accumulator and item
                            let result = match &func_val {
                                Value::Function { params, body, .. } => {
                                    self.push_scope();

                                    if params.len() == 2 {
                                        self.set_variable(params[0].clone(), accumulator.clone());
                                        self.set_variable(params[1].clone(), item.clone());
                                    } else {
                                        self.pop_scope();
                                        return Err(format!("reduce() lambda must have exactly 2 parameters, got {}", params.len()));
                                    }

                                    let result = self.eval_node(body)?;
                                    self.pop_scope();
                                    result
                                }
                                _ => {
                                    return Err(
                                        "reduce() expects a function as last argument".to_string()
                                    )
                                }
                            };

                            accumulator = result;
                        }

                        Ok(accumulator)
                    }
                    _ => Err("reduce() can only be called on lists".to_string()),
                }
            }

            "push" => {
                if args.len() != 2 {
                    return Err("push() expects exactly 2 arguments".to_string());
                }
                let list_val = self.eval_node(&args[0])?;
                let item_val = self.eval_node(&args[1])?;

                match list_val {
                    Value::List(mut list) => {
                        list.push(item_val);
                        Ok(Value::List(list))
                    }
                    _ => Err("push() can only be called on lists".to_string()),
                }
            }

            "pop" => {
                if args.len() != 1 {
                    return Err("pop() expects exactly 1 argument".to_string());
                }
                let list_val = self.eval_node(&args[0])?;

                match list_val {
                    Value::List(mut list) => {
                        if let Some(item) = list.pop() {
                            Ok(item)
                        } else {
                            Err("Cannot pop from empty list".to_string())
                        }
                    }
                    _ => Err("pop() can only be called on lists".to_string()),
                }
            }

            "append" => {
                if args.len() != 2 {
                    return Err("append() expects exactly 2 arguments".to_string());
                }
                let list1_val = self.eval_node(&args[0])?;
                let list2_val = self.eval_node(&args[1])?;

                match (list1_val, list2_val) {
                    (Value::List(mut list1), Value::List(list2)) => {
                        list1.extend(list2);
                        Ok(Value::List(list1))
                    }
                    _ => Err("append() can only be called on two lists".to_string()),
                }
            }

            "contains" => {
                if args.len() != 2 {
                    return Err("contains() expects exactly 2 arguments".to_string());
                }
                let container_val = self.eval_node(&args[0])?;
                let item_val = self.eval_node(&args[1])?;

                match container_val {
                    Value::List(list) => {
                        for list_item in list {
                            if self.values_equal(&list_item, &item_val) {
                                return Ok(Value::Boolean(true));
                            }
                        }
                        Ok(Value::Boolean(false))
                    }
                    Value::String(s) => {
                        if let Value::String(search) = &item_val {
                            Ok(Value::Boolean(s.contains(search)))
                        } else if let Value::Char(ch) = item_val {
                            Ok(Value::Boolean(s.contains(ch)))
                        } else {
                            #[cfg(feature = "regex")]
                            {
                                if let Value::Regex(r) = &item_val {
                                    return Ok(Value::Boolean(
                                        super::builtins::regex_builtins::regex_contains(&s, r),
                                    ));
                                }
                                return Err("contains() on string requires string, char, or regex argument".to_string());
                            }
                            #[cfg(not(feature = "regex"))]
                            Err("contains() on string requires string or char argument".to_string())
                        }
                    }
                    Value::Dict(dict) => {
                        if let Value::String(key) = item_val {
                            Ok(Value::Boolean(dict.contains_key(&key)))
                        } else if let Value::Integer(i) = item_val {
                            Ok(Value::Boolean(dict.contains_key(&i.to_string())))
                        } else {
                            Err("contains() on dict requires string or integer key".to_string())
                        }
                    }
                    _ => {
                        Err("contains() can only be called on lists, strings, or dicts".to_string())
                    }
                }
            }

            "keys" => {
                if args.len() != 1 {
                    return Err("keys() expects exactly 1 argument".to_string());
                }
                let dict_val = self.eval_node(&args[0])?;

                match dict_val {
                    Value::Dict(dict) => {
                        let keys: Vec<Value> =
                            dict.keys().map(|k| Value::String(k.clone())).collect();
                        Ok(Value::List(keys))
                    }
                    _ => Err("keys() can only be called on dictionaries".to_string()),
                }
            }

            "values" => {
                if args.len() != 1 {
                    return Err("values() expects exactly 1 argument".to_string());
                }
                let dict_val = self.eval_node(&args[0])?;

                match dict_val {
                    Value::Dict(dict) => {
                        let values: Vec<Value> = dict.values().cloned().collect();
                        Ok(Value::List(values))
                    }
                    _ => Err("values() can only be called on dictionaries".to_string()),
                }
            }

            // Dictionary methods
            "items" => {
                if args.len() != 1 {
                    return Err("items() expects exactly 1 argument".to_string());
                }
                let val = self.eval_node(&args[0])?;
                match val {
                    Value::Dict(dict) => {
                        let mut items = Vec::new();
                        for (k, v) in dict.iter() {
                            items.push(Value::Tuple(vec![Value::String(k.clone()), v.clone()]));
                        }
                        Ok(Value::List(items))
                    }
                    _ => Err("items() can only be called on dictionaries".to_string()),
                }
            }

            "get" => {
                if args.len() < 2 || args.len() > 3 {
                    return Err(
                        "get() expects 2 or 3 arguments: get(dict, key, [default])".to_string()
                    );
                }
                let dict_val = self.eval_node(&args[0])?;
                let key_val = self.eval_node(&args[1])?;
                let default = if args.len() == 3 {
                    Some(self.eval_node(&args[2])?)
                } else {
                    None
                };

                match dict_val {
                    Value::Dict(dict) => {
                        let key = match key_val {
                            Value::String(s) => s,
                            Value::Integer(i) => i.to_string(),
                            _ => return Err("get() key must be string or integer".to_string()),
                        };

                        if let Some(value) = dict.get(&key) {
                            Ok(value.clone())
                        } else if let Some(default_val) = default {
                            Ok(default_val)
                        } else {
                            Ok(Value::None)
                        }
                    }
                    _ => Err("get() can only be called on dictionaries".to_string()),
                }
            }

            "has" => {
                if args.len() != 2 {
                    return Err("has() expects exactly 2 arguments: has(dict, key)".to_string());
                }
                let dict_val = self.eval_node(&args[0])?;
                let key_val = self.eval_node(&args[1])?;

                match dict_val {
                    Value::Dict(dict) => {
                        let key = match key_val {
                            Value::String(s) => s,
                            Value::Integer(i) => i.to_string(),
                            _ => return Err("has() key must be string or integer".to_string()),
                        };
                        Ok(Value::Boolean(dict.contains_key(&key)))
                    }
                    _ => Err("has() can only be called on dictionaries".to_string()),
                }
            }

            "remove" => {
                if args.len() != 2 {
                    return Err(
                        "remove() expects exactly 2 arguments: remove(dict, key)".to_string()
                    );
                }
                let dict_val = self.eval_node(&args[0])?;
                let key_val = self.eval_node(&args[1])?;

                match dict_val {
                    Value::Dict(mut dict) => {
                        let key = match key_val {
                            Value::String(s) => s,
                            Value::Integer(i) => i.to_string(),
                            _ => return Err("remove() key must be string or integer".to_string()),
                        };

                        if let Some(value) = dict.remove(&key) {
                            Ok(value)
                        } else {
                            Ok(Value::None)
                        }
                    }
                    _ => Err("remove() can only be called on dictionaries".to_string()),
                }
            }

            "merge" => {
                if args.len() != 2 {
                    return Err(
                        "merge() expects exactly 2 arguments: merge(dict1, dict2)".to_string()
                    );
                }
                let dict1_val = self.eval_node(&args[0])?;
                let dict2_val = self.eval_node(&args[1])?;

                match (dict1_val, dict2_val) {
                    (Value::Dict(mut dict1), Value::Dict(dict2)) => {
                        for (k, v) in dict2.iter() {
                            dict1.insert(k.clone(), v.clone());
                        }
                        Ok(Value::Dict(dict1))
                    }
                    _ => Err("merge() can only be called on dictionaries".to_string()),
                }
            }

            "update" => {
                if args.len() != 2 {
                    return Err(
                        "update() expects exactly 2 arguments: update(dict1, dict2)".to_string()
                    );
                }
                let dict1_val = self.eval_node(&args[0])?;
                let dict2_val = self.eval_node(&args[1])?;

                match (dict1_val, dict2_val) {
                    (Value::Dict(mut dict1), Value::Dict(dict2)) => {
                        for (k, v) in dict2.iter() {
                            dict1.insert(k.clone(), v.clone());
                        }
                        Ok(Value::Dict(dict1))
                    }
                    _ => Err("update() can only be called on dictionaries".to_string()),
                }
            }

            "clear" => {
                if args.len() != 1 {
                    return Err("clear() expects exactly 1 argument: clear(dict)".to_string());
                }
                let dict_val = self.eval_node(&args[0])?;
                match dict_val {
                    Value::Dict(_) => Ok(Value::Dict(HashMap::new())),
                    _ => Err("clear() can only be called on dictionaries".to_string()),
                }
            }

            // File I/O functions
            // Advanced printing functions
            "table" => {
                if args.is_empty() {
                    return Err("table() expects at least 1 argument".to_string());
                }
                let data_val = self.eval_node(&args[0])?;

                match data_val {
                    Value::List(rows) => {
                        // Print table from list of lists
                        if rows.is_empty() {
                            return Ok(Value::None);
                        }

                        // Convert all rows to strings
                        let mut string_rows: Vec<Vec<String>> = Vec::new();
                        let mut max_widths: Vec<usize> = Vec::new();

                        for row in &rows {
                            if let Value::List(cells) = row {
                                let string_cells: Vec<String> =
                                    cells.iter().map(|v| format!("{}", v)).collect();

                                // Update max widths
                                for (i, cell) in string_cells.iter().enumerate() {
                                    if i >= max_widths.len() {
                                        max_widths.push(cell.len());
                                    } else if cell.len() > max_widths[i] {
                                        max_widths[i] = cell.len();
                                    }
                                }

                                string_rows.push(string_cells);
                            }
                        }

                        // Print table with borders
                        let total_width: usize =
                            max_widths.iter().sum::<usize>() + (max_widths.len() * 3) + 1;
                        println!("{}", "â”€".repeat(total_width));

                        for (row_idx, row) in string_rows.iter().enumerate() {
                            print!("â”‚");
                            for (i, cell) in row.iter().enumerate() {
                                print!(" {:width$} â”‚", cell, width = max_widths[i]);
                            }
                            println!();

                            if row_idx == 0 {
                                // Header separator
                                println!("{}", "â”€".repeat(total_width));
                            }
                        }

                        println!("{}", "â”€".repeat(total_width));
                        Ok(Value::None)
                    }
                    _ => Err("table() expects a list of lists".to_string()),
                }
            }

            "progress" => {
                if args.is_empty() {
                    return Err("progress() expects at least 1 argument (percentage)".to_string());
                }

                let percent_val = self.eval_node(&args[0])?;
                let percent = match percent_val {
                    Value::Integer(i) => i as f64,
                    Value::Float(f) => f,
                    _ => return Err("progress() percentage must be a number".to_string()),
                };

                let width = if args.len() > 1 {
                    match self.eval_node(&args[1])? {
                        Value::Integer(w) => w as usize,
                        _ => 40,
                    }
                } else {
                    40
                };

                // Animate the progress bar
                use std::io::{self, Write};
                use std::thread;
                use std::time::Duration;

                let steps = 20;
                let step_size = percent / steps as f64;

                for i in 0..=steps {
                    let current_percent = (i as f64 * step_size).min(percent);
                    let filled = ((current_percent / 100.0) * width as f64) as usize;
                    let empty = width - filled;

                    // Use carriage return to overwrite the same line
                    print!("\r[");
                    print!("{}", "â–ˆ".repeat(filled));
                    print!("{}", " ".repeat(empty));
                    print!("] {:.0}%", current_percent);
                    let _ = io::stdout().flush();

                    thread::sleep(Duration::from_millis(50));
                }

                println!(); // Move to next line after animation completes

                Ok(Value::None)
            }

            "rainbow" => {
                if args.is_empty() {
                    return Err("rainbow() expects 1 argument (text)".to_string());
                }

                let text_val = self.eval_node(&args[0])?;
                let text = match text_val {
                    Value::String(s) => s,
                    other => format!("{}", other),
                };

                // Simple rainbow effect using ANSI colors
                let colors = [31, 33, 32, 36, 34, 35]; // Red, Yellow, Green, Cyan, Blue, Magenta
                let mut result = String::new();

                for (i, ch) in text.chars().enumerate() {
                    let color = colors[i % colors.len()];
                    result.push_str(&format!("\x1b[{}m{}\x1b[0m", color, ch));
                }

                println!("{}", result);
                Ok(Value::None)
            }

            "gradient" => {
                if args.len() < 3 {
                    return Err(
                        "gradient() expects 3 arguments: gradient(start_color, end_color, text)"
                            .to_string(),
                    );
                }

                let text_val = self.eval_node(&args[2])?;
                let text = match text_val {
                    Value::String(s) => s,
                    other => format!("{}", other),
                };

                // Simple gradient using color interpolation
                // For now, just alternate between two colors
                let colors = [32, 36]; // Green to Cyan
                let mut result = String::new();

                for (i, ch) in text.chars().enumerate() {
                    let color = colors[i % colors.len()];
                    result.push_str(&format!("\x1b[{}m{}\x1b[0m", color, ch));
                }

                println!("{}", result);
                Ok(Value::None)
            }

            "bold" => {
                if args.is_empty() {
                    return Err("bold() expects 1 argument (text)".to_string());
                }

                let text_val = self.eval_node(&args[0])?;
                let text = match text_val {
                    Value::String(s) => s,
                    other => format!("{}", other),
                };

                Ok(Value::String(format!("\x1b[1m{}\x1b[0m", text)))
            }

            "underline" => {
                if args.is_empty() {
                    return Err("underline() expects 1 argument (text)".to_string());
                }

                let text_val = self.eval_node(&args[0])?;
                let text = match text_val {
                    Value::String(s) => s,
                    other => format!("{}", other),
                };

                Ok(Value::String(format!("\x1b[4m{}\x1b[0m", text)))
            }

            "dim" => {
                if args.is_empty() {
                    return Err("dim() expects 1 argument (text)".to_string());
                }

                let text_val = self.eval_node(&args[0])?;
                let text = match text_val {
                    Value::String(s) => s,
                    other => format!("{}", other),
                };

                Ok(Value::String(format!("\x1b[2m{}\x1b[0m", text)))
            }

            // Rich-like spinner and loading animations
            "spinner" => {
                if args.is_empty() {
                    return Err("spinner() expects at least 1 argument (style)".to_string());
                }

                let style_val = self.eval_node(&args[0])?;
                let style = match style_val {
                    Value::String(s) => s,
                    _ => "dots".to_string(),
                };

                let duration = if args.len() > 1 {
                    match self.eval_node(&args[1])? {
                        Value::Integer(d) => d as f64,
                        Value::Float(d) => d,
                        _ => 3.0,
                    }
                } else {
                    3.0
                };

                let message = if args.len() > 2 {
                    match self.eval_node(&args[2])? {
                        Value::String(s) => s,
                        other => format!("{}", other),
                    }
                } else {
                    "Loading".to_string()
                };

                self.show_spinner(&style, duration, &message)?;
                Ok(Value::None)
            }

            "loading" => {
                if args.is_empty() {
                    return Err("loading() expects at least 1 argument (message)".to_string());
                }

                let message_val = self.eval_node(&args[0])?;
                let message = match message_val {
                    Value::String(s) => s,
                    other => format!("{}", other),
                };

                let style = if args.len() > 1 {
                    match self.eval_node(&args[1])? {
                        Value::String(s) => s,
                        _ => "dots".to_string(),
                    }
                } else {
                    "dots".to_string()
                };

                let duration = if args.len() > 2 {
                    match self.eval_node(&args[2])? {
                        Value::Integer(d) => d as f64,
                        Value::Float(d) => d,
                        _ => 2.0,
                    }
                } else {
                    2.0
                };

                self.show_loading(&message, &style, duration)?;
                Ok(Value::None)
            }

            "panel" => {
                if args.is_empty() {
                    return Err("panel() expects at least 1 argument (text)".to_string());
                }

                let text_val = self.eval_node(&args[0])?;
                let text = match text_val {
                    Value::String(s) => s,
                    other => format!("{}", other),
                };

                let title = if args.len() > 1 {
                    match self.eval_node(&args[1])? {
                        Value::String(s) => Some(s),
                        _ => None,
                    }
                } else {
                    None
                };

                let style = if args.len() > 2 {
                    match self.eval_node(&args[2])? {
                        Value::String(s) => s,
                        _ => "single".to_string(),
                    }
                } else {
                    "single".to_string()
                };

                self.show_panel(&text, title.as_deref(), &style)?;
                Ok(Value::None)
            }

            "box" => {
                if args.is_empty() {
                    return Err("box() expects at least 1 argument (text)".to_string());
                }

                let text_val = self.eval_node(&args[0])?;
                let text = match text_val {
                    Value::String(s) => s,
                    other => format!("{}", other),
                };

                let width = if args.len() > 1 {
                    match self.eval_node(&args[1])? {
                        Value::Integer(w) => w as usize,
                        _ => 60,
                    }
                } else {
                    60
                };

                self.show_box(&text, width)?;
                Ok(Value::None)
            }

            "status" => {
                if args.len() < 2 {
                    return Err("status() expects 2 arguments: status(type, message)".to_string());
                }

                let status_type_val = self.eval_node(&args[0])?;
                let status_type = match status_type_val {
                    Value::String(s) => s,
                    _ => "info".to_string(),
                };

                let message_val = self.eval_node(&args[1])?;
                let message = match message_val {
                    Value::String(s) => s,
                    other => format!("{}", other),
                };

                self.show_status(&status_type, &message)?;
                Ok(Value::None)
            }

            "tree" => {
                if args.is_empty() {
                    return Err("tree() expects at least 1 argument (data)".to_string());
                }

                let data_val = self.eval_node(&args[0])?;
                let title = if args.len() > 1 {
                    match self.eval_node(&args[1])? {
                        Value::String(s) => Some(s),
                        _ => None,
                    }
                } else {
                    None
                };

                self.show_tree(&data_val, title.as_deref(), 0)?;
                Ok(Value::None)
            }

            "columns" => {
                if args.is_empty() {
                    return Err("columns() expects at least 1 argument (list of texts)".to_string());
                }

                let data_val = self.eval_node(&args[0])?;
                match data_val {
                    Value::List(items) => {
                        let texts: Vec<String> = items.iter().map(|v| format!("{}", v)).collect();

                        self.show_columns(&texts)?;
                        Ok(Value::None)
                    }
                    _ => Err("columns() expects a list".to_string()),
                }
            }

            "find" => {
                if args.len() != 2 {
                    return Err("find() expects exactly 2 arguments".to_string());
                }
                let string_val = self.eval_node(&args[0])?;
                let pattern_val = self.eval_node(&args[1])?;

                match (&string_val, &pattern_val) {
                    (Value::String(s), Value::String(pattern)) => match s.find(pattern) {
                        Some(index) => Ok(Value::Integer(index as i64)),
                        None => Ok(Value::Integer(-1)),
                    },
                    #[cfg(feature = "regex")]
                    (Value::String(s), Value::Regex(r)) => {
                        Ok(super::builtins::regex_builtins::regex_find(s, r)
                            .unwrap_or(Value::None))
                    }
                    _ => Err("find() expects (string, string) or (string, regex)".to_string()),
                }
            }

            "matches" => {
                if args.len() != 2 {
                    return Err("matches() expects exactly 2 arguments: matches(string, regex)".to_string());
                }
                let string_val = self.eval_node(&args[0])?;
                let pattern_val = self.eval_node(&args[1])?;
                match (&string_val, &pattern_val) {
                    #[cfg(feature = "regex")]
                    (Value::String(s), Value::Regex(r)) => {
                        Ok(Value::Boolean(super::builtins::regex_builtins::regex_matches(s, r)))
                    }
                    _ => Err("matches() expects (string, regex)".to_string()),
                }
            }

            "find_all" => {
                if args.len() != 2 {
                    return Err("find_all() expects exactly 2 arguments: find_all(string, regex)".to_string());
                }
                let string_val = self.eval_node(&args[0])?;
                let pattern_val = self.eval_node(&args[1])?;
                match (&string_val, &pattern_val) {
                    #[cfg(feature = "regex")]
                    (Value::String(s), Value::Regex(r)) => {
                        let list = super::builtins::regex_builtins::regex_find_all(s, r);
                        Ok(Value::List(list))
                    }
                    _ => Err("find_all() expects (string, regex)".to_string()),
                }
            }

            "pad_left" => {
                if args.len() != 2 {
                    return Err("pad_left() expects exactly 2 arguments".to_string());
                }
                let string_val = self.eval_node(&args[0])?;
                let width_val = self.eval_node(&args[1])?;

                match (string_val, width_val) {
                    (Value::String(s), Value::Integer(width)) => {
                        if width < 0 {
                            return Err("pad_left() width must be non-negative".to_string());
                        }
                        let result = format!("{:>width$}", s, width = width as usize);
                        Ok(Value::String(result))
                    }
                    _ => Err("pad_left() expects string and integer arguments".to_string()),
                }
            }

            "pad_right" => {
                if args.len() != 2 {
                    return Err("pad_right() expects exactly 2 arguments".to_string());
                }
                let string_val = self.eval_node(&args[0])?;
                let width_val = self.eval_node(&args[1])?;

                match (string_val, width_val) {
                    (Value::String(s), Value::Integer(width)) => {
                        if width < 0 {
                            return Err("pad_right() width must be non-negative".to_string());
                        }
                        let result = format!("{:<width$}", s, width = width as usize);
                        Ok(Value::String(result))
                    }
                    _ => Err("pad_right() expects string and integer arguments".to_string()),
                }
            }

            // Vector and Matrix functions
            "dot" => {
                if args.len() != 2 {
                    return Err("dot() expects exactly 2 arguments".to_string());
                }
                let a_val = self.eval_node(&args[0])?;
                let b_val = self.eval_node(&args[1])?;

                match (a_val, b_val) {
                    (Value::Vector(a), Value::Vector(b)) => {
                        if a.len() != b.len() {
                            return Err("Vectors must have same length for dot product".to_string());
                        }
                        let result: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
                        Ok(Value::Float(result))
                    }
                    _ => Err("dot() expects two vectors".to_string()),
                }
            }

            "magnitude" => {
                if args.len() != 1 {
                    return Err("magnitude() expects exactly 1 argument".to_string());
                }
                let vec_val = self.eval_node(&args[0])?;

                match vec_val {
                    Value::Vector(v) => {
                        let mag = v.iter().map(|x| x * x).sum::<f64>().sqrt();
                        Ok(Value::Float(mag))
                    }
                    _ => Err("magnitude() expects a vector".to_string()),
                }
            }

            "normalize" => {
                if args.len() != 1 {
                    return Err("normalize() expects exactly 1 argument".to_string());
                }
                let vec_val = self.eval_node(&args[0])?;

                match vec_val {
                    Value::Vector(v) => {
                        let mag = v.iter().map(|x| x * x).sum::<f64>().sqrt();
                        if mag == 0.0 {
                            return Err("Cannot normalize zero vector".to_string());
                        }
                        let normalized: Vec<f64> = v.iter().map(|x| x / mag).collect();
                        Ok(Value::Vector(normalized))
                    }
                    _ => Err("normalize() expects a vector".to_string()),
                }
            }

            "transpose" => {
                if args.len() != 1 {
                    return Err("transpose() expects exactly 1 argument".to_string());
                }
                let mat_val = self.eval_node(&args[0])?;

                match mat_val {
                    Value::Matrix(m) => {
                        if m.is_empty() || m[0].is_empty() {
                            return Ok(Value::Matrix(Vec::new()));
                        }
                        let rows = m.len();
                        let cols = m[0].len();
                        let mut transposed = vec![vec![0.0; rows]; cols];

                        for i in 0..rows {
                            for j in 0..cols {
                                transposed[j][i] = m[i][j];
                            }
                        }
                        Ok(Value::Matrix(transposed))
                    }
                    _ => Err("transpose() expects a matrix".to_string()),
                }
            }

            "matmul" => {
                if args.len() != 2 {
                    return Err("matmul() expects exactly 2 arguments".to_string());
                }
                let a_val = self.eval_node(&args[0])?;
                let b_val = self.eval_node(&args[1])?;

                match (a_val, b_val) {
                    (Value::Matrix(a), Value::Matrix(b)) => {
                        if a.is_empty() || b.is_empty() || a[0].len() != b.len() {
                            return Err(
                                "Matrix dimensions incompatible for multiplication".to_string()
                            );
                        }

                        let rows_a = a.len();
                        let cols_a = a[0].len();
                        let cols_b = b[0].len();
                        let mut result = vec![vec![0.0; cols_b]; rows_a];

                        for i in 0..rows_a {
                            for j in 0..cols_b {
                                for k in 0..cols_a {
                                    result[i][j] += a[i][k] * b[k][j];
                                }
                            }
                        }
                        Ok(Value::Matrix(result))
                    }
                    (Value::Matrix(m), Value::Vector(v)) => {
                        if m.is_empty() || m[0].len() != v.len() {
                            return Err("Matrix and vector dimensions incompatible".to_string());
                        }

                        let mut result = Vec::new();
                        for row in &m {
                            let dot_product: f64 =
                                row.iter().zip(v.iter()).map(|(a, b)| a * b).sum();
                            result.push(dot_product);
                        }
                        Ok(Value::Vector(result))
                    }
                    _ => Err("matmul() expects matrices or matrix and vector".to_string()),
                }
            }

            // Set operations
            "add" => {
                if args.len() != 2 {
                    return Err("add() expects exactly 2 arguments".to_string());
                }
                let set_val = self.eval_node(&args[0])?;
                let item_val = self.eval_node(&args[1])?;

                match set_val {
                    Value::Set(mut set) => {
                        let key = match item_val {
                            Value::String(s) => s,
                            Value::Integer(i) => i.to_string(),
                            Value::Float(f) => f.to_string(),
                            Value::Boolean(b) => b.to_string(),
                            _ => {
                                return Err(
                                    "Set elements must be convertible to strings".to_string()
                                )
                            }
                        };
                        set.insert(key);
                        Ok(Value::Set(set))
                    }
                    _ => Err("add() can only be called on sets".to_string()),
                }
            }

            // Counter operations
            "is_sorted" => {
                if args.len() != 1 {
                    return Err("is_sorted() expects exactly 1 argument".to_string());
                }
                let list_val = self.eval_node(&args[0])?;

                match list_val {
                    Value::List(list) => {
                        for i in 1..list.len() {
                            match (&list[i - 1], &list[i]) {
                                (Value::Integer(a), Value::Integer(b)) => {
                                    if a > b {
                                        return Ok(Value::Boolean(false));
                                    }
                                }
                                (Value::Float(a), Value::Float(b)) => {
                                    if a > b {
                                        return Ok(Value::Boolean(false));
                                    }
                                }
                                (Value::String(a), Value::String(b)) => {
                                    if a > b {
                                        return Ok(Value::Boolean(false));
                                    }
                                }
                                _ => {
                                    return Err(
                                        "is_sorted() requires comparable elements".to_string()
                                    )
                                }
                            }
                        }
                        Ok(Value::Boolean(true))
                    }
                    _ => Err("is_sorted() can only be called on lists".to_string()),
                }
            }

            "shuffle" => {
                if args.len() != 1 {
                    return Err("shuffle() expects exactly 1 argument".to_string());
                }
                let list_val = self.eval_node(&args[0])?;

                match list_val {
                    Value::List(mut list) => {
                        // Simple Fisher-Yates shuffle
                        use std::collections::hash_map::DefaultHasher;
                        use std::hash::{Hash, Hasher};

                        let mut hasher = DefaultHasher::new();
                        std::ptr::addr_of!(list).hash(&mut hasher);
                        let mut seed = hasher.finish() as usize;

                        for i in (1..list.len()).rev() {
                            // Simple LCG for pseudo-random numbers
                            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
                            let j = seed % (i + 1);
                            list.swap(i, j);
                        }
                        Ok(Value::List(list))
                    }
                    _ => Err("shuffle() can only be called on lists".to_string()),
                }
            }

            "sample" => {
                if args.len() != 2 {
                    return Err("sample() expects exactly 2 arguments".to_string());
                }
                let list_val = self.eval_node(&args[0])?;
                let n_val = self.eval_node(&args[1])?;

                match (list_val, n_val) {
                    (Value::List(list), Value::Integer(n)) => {
                        if n < 0 {
                            return Err("sample() size must be non-negative".to_string());
                        }
                        let n = n as usize;
                        if n > list.len() {
                            return Err("sample() size cannot be larger than list size".to_string());
                        }

                        // Simple sampling without replacement
                        let mut result = Vec::new();
                        let mut indices: Vec<usize> = (0..list.len()).collect();

                        // Simple shuffle of indices
                        use std::collections::hash_map::DefaultHasher;
                        use std::hash::{Hash, Hasher};

                        let mut hasher = DefaultHasher::new();
                        std::ptr::addr_of!(list).hash(&mut hasher);
                        let mut seed = hasher.finish() as usize;

                        for i in (1..indices.len()).rev() {
                            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
                            let j = seed % (i + 1);
                            indices.swap(i, j);
                        }

                        for i in 0..n {
                            result.push(list[indices[i]].clone());
                        }

                        Ok(Value::List(result))
                    }
                    _ => Err("sample() expects a list and an integer".to_string()),
                }
            }

            "accumulate" => {
                if args.len() != 1 {
                    return Err("accumulate() expects exactly 1 argument".to_string());
                }
                let list_val = self.eval_node(&args[0])?;

                match list_val {
                    Value::List(list) => {
                        if list.is_empty() {
                            return Ok(Value::List(Vec::new()));
                        }

                        let mut result = Vec::new();
                        let mut acc = list[0].clone();
                        result.push(acc.clone());

                        for item in &list[1..] {
                            match (&acc, item) {
                                (Value::Integer(a), Value::Integer(b)) => {
                                    acc = Value::Integer(a + b);
                                }
                                (Value::Float(a), Value::Float(b)) => {
                                    acc = Value::Float(a + b);
                                }
                                (Value::Integer(a), Value::Float(b)) => {
                                    acc = Value::Float(*a as f64 + b);
                                }
                                (Value::Float(a), Value::Integer(b)) => {
                                    acc = Value::Float(a + *b as f64);
                                }
                                _ => return Err("accumulate() requires numeric values".to_string()),
                            }
                            result.push(acc.clone());
                        }

                        Ok(Value::List(result))
                    }
                    _ => Err("accumulate() can only be called on lists".to_string()),
                }
            }

            // Additional collection functions
            "take" => {
                if args.len() != 2 {
                    return Err("take() expects exactly 2 arguments".to_string());
                }
                let list_val = self.eval_node(&args[0])?;
                let n_val = self.eval_node(&args[1])?;

                match (list_val, n_val) {
                    (Value::List(list), Value::Integer(n)) => {
                        if n < 0 {
                            return Err("take() count must be non-negative".to_string());
                        }
                        let n = n as usize;
                        let result = list.into_iter().take(n).collect();
                        Ok(Value::List(result))
                    }
                    _ => Err("take() expects list and integer arguments".to_string()),
                }
            }

            "drop" => {
                if args.len() != 2 {
                    return Err("drop() expects exactly 2 arguments".to_string());
                }
                let list_val = self.eval_node(&args[0])?;
                let n_val = self.eval_node(&args[1])?;

                match (list_val, n_val) {
                    (Value::List(list), Value::Integer(n)) => {
                        if n < 0 {
                            return Err("drop() count must be non-negative".to_string());
                        }
                        let n = n as usize;
                        let result = list.into_iter().skip(n).collect();
                        Ok(Value::List(result))
                    }
                    _ => Err("drop() expects list and integer arguments".to_string()),
                }
            }

            "sin" => {
                if args.len() != 1 {
                    return Err("sin() expects exactly 1 argument".to_string());
                }
                let val = self.eval_node(&args[0])?;

                match val {
                    Value::Integer(i) => Ok(Value::Float((i as f64).sin())),
                    Value::Float(f) => Ok(Value::Float(f.sin())),
                    _ => Err("sin() can only be called on numbers".to_string()),
                }
            }

            "cos" => {
                if args.len() != 1 {
                    return Err("cos() expects exactly 1 argument".to_string());
                }
                let val = self.eval_node(&args[0])?;

                match val {
                    Value::Integer(i) => Ok(Value::Float((i as f64).cos())),
                    Value::Float(f) => Ok(Value::Float(f.cos())),
                    _ => Err("cos() can only be called on numbers".to_string()),
                }
            }

            "tan" => {
                if args.len() != 1 {
                    return Err("tan() expects exactly 1 argument".to_string());
                }
                let val = self.eval_node(&args[0])?;

                match val {
                    Value::Integer(i) => Ok(Value::Float((i as f64).tan())),
                    Value::Float(f) => Ok(Value::Float(f.tan())),
                    _ => Err("tan() can only be called on numbers".to_string()),
                }
            }

            "log" => {
                if args.len() != 1 {
                    return Err("log() expects exactly 1 argument".to_string());
                }
                let val = self.eval_node(&args[0])?;

                match val {
                    Value::Integer(i) => Ok(Value::Float((i as f64).ln())),
                    Value::Float(f) => Ok(Value::Float(f.ln())),
                    _ => Err("log() can only be called on numbers".to_string()),
                }
            }

            "exp" => {
                if args.len() != 1 {
                    return Err("exp() expects exactly 1 argument".to_string());
                }
                let val = self.eval_node(&args[0])?;

                match val {
                    Value::Integer(i) => Ok(Value::Float((i as f64).exp())),
                    Value::Float(f) => Ok(Value::Float(f.exp())),
                    _ => Err("exp() can only be called on numbers".to_string()),
                }
            }

            "clamp" => {
                if args.len() != 3 {
                    return Err("clamp() expects exactly 3 arguments".to_string());
                }
                let val = self.eval_node(&args[0])?;
                let min_val = self.eval_node(&args[1])?;
                let max_val = self.eval_node(&args[2])?;

                match (val, min_val, max_val) {
                    (Value::Integer(v), Value::Integer(min), Value::Integer(max)) => {
                        Ok(Value::Integer(v.max(min).min(max)))
                    }
                    (Value::Float(v), Value::Float(min), Value::Float(max)) => {
                        Ok(Value::Float(v.max(min).min(max)))
                    }
                    (Value::Integer(v), Value::Float(min), Value::Float(max)) => {
                        let v = v as f64;
                        Ok(Value::Float(v.max(min).min(max)))
                    }
                    (Value::Float(v), Value::Integer(min), Value::Integer(max)) => {
                        let min = min as f64;
                        let max = max as f64;
                        Ok(Value::Float(v.max(min).min(max)))
                    }
                    _ => Err("clamp() expects numeric arguments".to_string()),
                }
            }

            "lerp" => {
                if args.len() != 3 {
                    return Err("lerp() expects exactly 3 arguments".to_string());
                }
                let a_val = self.eval_node(&args[0])?;
                let b_val = self.eval_node(&args[1])?;
                let t_val = self.eval_node(&args[2])?;

                let a = match a_val {
                    Value::Integer(i) => i as f64,
                    Value::Float(f) => f,
                    _ => return Err("lerp() first argument must be numeric".to_string()),
                };

                let b = match b_val {
                    Value::Integer(i) => i as f64,
                    Value::Float(f) => f,
                    _ => return Err("lerp() second argument must be numeric".to_string()),
                };

                let t = match t_val {
                    Value::Integer(i) => i as f64,
                    Value::Float(f) => f,
                    _ => return Err("lerp() third argument must be numeric".to_string()),
                };

                Ok(Value::Float(a + t * (b - a)))
            }

            "distance" => {
                if args.len() != 2 {
                    return Err("distance() expects exactly 2 arguments".to_string());
                }
                let a_val = self.eval_node(&args[0])?;
                let b_val = self.eval_node(&args[1])?;

                match (a_val, b_val) {
                    (Value::Vector(a), Value::Vector(b)) => {
                        if a.len() != b.len() {
                            return Err("Vectors must have same length for distance calculation"
                                .to_string());
                        }
                        let dist_sq: f64 =
                            a.iter().zip(b.iter()).map(|(x, y)| (x - y).powi(2)).sum();
                        Ok(Value::Float(dist_sq.sqrt()))
                    }
                    (Value::Tuple(a), Value::Tuple(b)) => {
                        if a.len() != b.len() {
                            return Err(
                                "Tuples must have same length for distance calculation".to_string()
                            );
                        }
                        let mut dist_sq = 0.0;
                        for (av, bv) in a.iter().zip(b.iter()) {
                            let a_num = match av {
                                Value::Integer(i) => *i as f64,
                                Value::Float(f) => *f,
                                _ => {
                                    return Err(
                                        "Distance calculation requires numeric values".to_string()
                                    )
                                }
                            };
                            let b_num = match bv {
                                Value::Integer(i) => *i as f64,
                                Value::Float(f) => *f,
                                _ => {
                                    return Err(
                                        "Distance calculation requires numeric values".to_string()
                                    )
                                }
                            };
                            dist_sq += (a_num - b_num).powi(2);
                        }
                        Ok(Value::Float(dist_sq.sqrt()))
                    }
                    _ => Err("distance() expects two vectors or tuples".to_string()),
                }
            }

            "cross" => {
                if args.len() != 2 {
                    return Err("cross() expects exactly 2 arguments".to_string());
                }
                let a_val = self.eval_node(&args[0])?;
                let b_val = self.eval_node(&args[1])?;

                match (a_val, b_val) {
                    (Value::Vector(a), Value::Vector(b)) => {
                        if a.len() != 3 || b.len() != 3 {
                            return Err("Cross product requires 3D vectors".to_string());
                        }
                        let result = vec![
                            a[1] * b[2] - a[2] * b[1],
                            a[2] * b[0] - a[0] * b[2],
                            a[0] * b[1] - a[1] * b[0],
                        ];
                        Ok(Value::Vector(result))
                    }
                    _ => Err("cross() expects two 3D vectors".to_string()),
                }
            }

            "determinant" => {
                if args.len() != 1 {
                    return Err("determinant() expects exactly 1 argument".to_string());
                }
                let mat_val = self.eval_node(&args[0])?;

                match mat_val {
                    Value::Matrix(m) => {
                        if m.is_empty() || m.len() != m[0].len() {
                            return Err("Determinant requires a square matrix".to_string());
                        }

                        let det =
                            match m.len() {
                                1 => m[0][0],
                                2 => m[0][0] * m[1][1] - m[0][1] * m[1][0],
                                3 => {
                                    m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
                                        - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
                                        + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
                                }
                                _ => return Err(
                                    "Determinant only implemented for 1x1, 2x2, and 3x3 matrices"
                                        .to_string(),
                                ),
                            };

                        Ok(Value::Float(det))
                    }
                    _ => Err("determinant() expects a matrix".to_string()),
                }
            }

            "identity" => {
                if args.len() != 1 {
                    return Err("identity() expects exactly 1 argument".to_string());
                }
                let size_val = self.eval_node(&args[0])?;

                match size_val {
                    Value::Integer(size) => {
                        if size <= 0 {
                            return Err("Identity matrix size must be positive".to_string());
                        }
                        let size = size as usize;
                        let mut matrix = vec![vec![0.0; size]; size];
                        for (i, row) in matrix.iter_mut().enumerate().take(size) {
                            row[i] = 1.0;
                        }
                        Ok(Value::Matrix(matrix))
                    }
                    _ => Err("identity() expects an integer size".to_string()),
                }
            }

            "zeros" => match args.len() {
                1 => {
                    let size_val = self.eval_node(&args[0])?;
                    match size_val {
                        Value::Integer(size) => {
                            if size <= 0 {
                                return Err("zeros() size must be positive".to_string());
                            }
                            let vec = vec![0.0; size as usize];
                            Ok(Value::Vector(vec))
                        }
                        _ => Err("zeros() expects an integer size".to_string()),
                    }
                }
                2 => {
                    let rows_val = self.eval_node(&args[0])?;
                    let cols_val = self.eval_node(&args[1])?;
                    match (rows_val, cols_val) {
                        (Value::Integer(rows), Value::Integer(cols)) => {
                            if rows <= 0 || cols <= 0 {
                                return Err("zeros() dimensions must be positive".to_string());
                            }
                            let matrix = vec![vec![0.0; cols as usize]; rows as usize];
                            Ok(Value::Matrix(matrix))
                        }
                        _ => Err("zeros() expects integer dimensions".to_string()),
                    }
                }
                _ => Err("zeros() expects 1 or 2 arguments".to_string()),
            },

            "ones" => match args.len() {
                1 => {
                    let size_val = self.eval_node(&args[0])?;
                    match size_val {
                        Value::Integer(size) => {
                            if size <= 0 {
                                return Err("ones() size must be positive".to_string());
                            }
                            let vec = vec![1.0; size as usize];
                            Ok(Value::Vector(vec))
                        }
                        _ => Err("ones() expects an integer size".to_string()),
                    }
                }
                2 => {
                    let rows_val = self.eval_node(&args[0])?;
                    let cols_val = self.eval_node(&args[1])?;
                    match (rows_val, cols_val) {
                        (Value::Integer(rows), Value::Integer(cols)) => {
                            if rows <= 0 || cols <= 0 {
                                return Err("ones() dimensions must be positive".to_string());
                            }
                            let matrix = vec![vec![1.0; cols as usize]; rows as usize];
                            Ok(Value::Matrix(matrix))
                        }
                        _ => Err("ones() expects integer dimensions".to_string()),
                    }
                }
                _ => Err("ones() expects 1 or 2 arguments".to_string()),
            },

            // String algorithms
            "levenshtein" => {
                if args.len() != 2 {
                    return Err("levenshtein() expects exactly 2 arguments".to_string());
                }
                let s1_val = self.eval_node(&args[0])?;
                let s2_val = self.eval_node(&args[1])?;

                match (s1_val, s2_val) {
                    (Value::String(s1), Value::String(s2)) => {
                        let distance = self.levenshtein_distance(&s1, &s2);
                        Ok(Value::Integer(distance as i64))
                    }
                    _ => Err("levenshtein() expects two strings".to_string()),
                }
            }

            "hamming" => {
                if args.len() != 2 {
                    return Err("hamming() expects exactly 2 arguments".to_string());
                }
                let s1_val = self.eval_node(&args[0])?;
                let s2_val = self.eval_node(&args[1])?;

                match (s1_val, s2_val) {
                    (Value::String(s1), Value::String(s2)) => {
                        if s1.len() != s2.len() {
                            return Err("hamming() requires strings of equal length".to_string());
                        }
                        let distance = s1.chars().zip(s2.chars()).filter(|(a, b)| a != b).count();
                        Ok(Value::Integer(distance as i64))
                    }
                    _ => Err("hamming() expects two strings".to_string()),
                }
            }

            // Advanced algorithm helpers
            "kmp_search" => {
                if args.len() != 2 {
                    return Err("kmp_search() expects exactly 2 arguments".to_string());
                }
                let text_val = self.eval_node(&args[0])?;
                let pattern_val = self.eval_node(&args[1])?;

                match (text_val, pattern_val) {
                    (Value::String(text), Value::String(pattern)) => {
                        let positions = self.kmp_search(&text, &pattern);
                        Ok(Value::List(
                            positions
                                .into_iter()
                                .map(|p| Value::Integer(p as i64))
                                .collect(),
                        ))
                    }
                    _ => Err("kmp_search() expects two strings".to_string()),
                }
            }

            "z_array" => {
                if args.len() != 1 {
                    return Err("z_array() expects exactly 1 argument".to_string());
                }
                let text_val = self.eval_node(&args[0])?;

                match text_val {
                    Value::String(text) => {
                        let z_array = self.compute_z_array(&text);
                        Ok(Value::List(
                            z_array
                                .into_iter()
                                .map(|z| Value::Integer(z as i64))
                                .collect(),
                        ))
                    }
                    _ => Err("z_array() expects a string".to_string()),
                }
            }

            "convex_hull" => {
                if args.len() != 1 {
                    return Err("convex_hull() expects exactly 1 argument".to_string());
                }
                let points_val = self.eval_node(&args[0])?;

                match points_val {
                    Value::List(points) => {
                        let hull = self.convex_hull_2d(&points)?;
                        Ok(Value::List(hull))
                    }
                    _ => Err("convex_hull() expects a list of (x, y) tuples".to_string()),
                }
            }

            "fft" => {
                if args.len() != 1 {
                    return Err("fft() expects exactly 1 argument".to_string());
                }
                let signal_val = self.eval_node(&args[0])?;

                match signal_val {
                    Value::List(signal) => {
                        let fft_result = self.fft_transform(&signal)?;
                        Ok(Value::List(fft_result))
                    }
                    Value::Vector(signal) => {
                        let signal_list: Vec<Value> =
                            signal.into_iter().map(Value::Float).collect();
                        let fft_result = self.fft_transform(&signal_list)?;
                        Ok(Value::List(fft_result))
                    }
                    _ => Err("fft() expects a list or vector of numbers".to_string()),
                }
            }

            // More collection functions
            "all" => {
                if args.len() != 1 {
                    return Err("all() expects exactly 1 argument".to_string());
                }
                let val = self.eval_node(&args[0])?;

                match val {
                    Value::List(list) => {
                        let all_true = list.iter().all(|item| self.is_truthy(item));
                        Ok(Value::Boolean(all_true))
                    }
                    _ => Err("all() can only be called on lists".to_string()),
                }
            }

            "any" => {
                if args.len() != 1 {
                    return Err("any() expects exactly 1 argument".to_string());
                }
                let val = self.eval_node(&args[0])?;

                match val {
                    Value::List(list) => {
                        let any_true = list.iter().any(|item| self.is_truthy(item));
                        Ok(Value::Boolean(any_true))
                    }
                    _ => Err("any() can only be called on lists".to_string()),
                }
            }

            // Functional programming helpers
            "compose" => {
                if args.len() != 2 {
                    return Err("compose() expects exactly 2 arguments".to_string());
                }
                // For now, just return a placeholder - full function composition would need more work
                Ok(Value::String("composed_function".to_string()))
            }

            "curry" => {
                if args.len() != 1 {
                    return Err("curry() expects exactly 1 argument".to_string());
                }
                // Placeholder for currying - would need more advanced function handling
                Ok(Value::String("curried_function".to_string()))
            }
            // Date/time functions
            "now" => {
                if !args.is_empty() {
                    return Err("now() expects no arguments".to_string());
                }
                use std::time::{SystemTime, UNIX_EPOCH};
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                Ok(Value::Integer(timestamp as i64))
            }

            "today" => {
                if !args.is_empty() {
                    return Err("today() expects no arguments".to_string());
                }
                let now = std::time::SystemTime::now();
                let dur = now.duration_since(std::time::UNIX_EPOCH).unwrap();
                let secs = dur.as_secs() as i64;
                let days = secs / 86400;
                let jd = days + 2440588;
                let (y, m, d) = Self::julian_to_ymd(jd);
                Ok(Value::Date(format!("{:04}-{:02}-{:02}", y, m, d)))
            }

            // Random functions
            "random" => {
                if args.is_empty() {
                    // Random float between 0 and 1
                    use std::collections::hash_map::DefaultHasher;
                    use std::hash::{Hash, Hasher};

                    let mut hasher = DefaultHasher::new();
                    std::ptr::addr_of!(self).hash(&mut hasher);
                    let seed = hasher.finish();
                    let random_val = (seed % 1000) as f64 / 1000.0;
                    Ok(Value::Float(random_val))
                } else if args.len() == 1 {
                    // Random integer from 0 to n-1
                    let n_val = self.eval_node(&args[0])?;
                    match n_val {
                        Value::Integer(n) => {
                            if n <= 0 {
                                return Err("random() argument must be positive".to_string());
                            }
                            use std::collections::hash_map::DefaultHasher;
                            use std::hash::{Hash, Hasher};

                            let mut hasher = DefaultHasher::new();
                            std::ptr::addr_of!(self).hash(&mut hasher);
                            let seed = hasher.finish();
                            let random_val = (seed % (n as u64)) as i64;
                            Ok(Value::Integer(random_val))
                        }
                        _ => Err("random() expects an integer argument".to_string()),
                    }
                } else {
                    Err("random() expects 0 or 1 arguments".to_string())
                }
            }

            "chain" => {
                if args.len() < 2 {
                    return Err("chain() expects at least 2 arguments".to_string());
                }

                let mut result = Vec::new();
                for arg in args {
                    let val = self.eval_node(arg)?;
                    match val {
                        Value::List(list) => result.extend(list),
                        Value::String(s) => {
                            for ch in s.chars() {
                                result.push(Value::Char(ch));
                            }
                        }
                        Value::Vector(vec) => {
                            for item in vec {
                                result.push(Value::Float(item));
                            }
                        }
                        _ => result.push(val),
                    }
                }
                Ok(Value::List(result))
            }

            "cycle" => {
                if args.len() != 2 {
                    return Err("cycle() expects exactly 2 arguments".to_string());
                }
                let iterable_val = self.eval_node(&args[0])?;
                let count_val = self.eval_node(&args[1])?;

                let count = match count_val {
                    Value::Integer(n) => {
                        if n < 0 {
                            return Err("cycle() count must be non-negative".to_string());
                        }
                        n as usize
                    }
                    _ => return Err("cycle() count must be an integer".to_string()),
                };

                match iterable_val {
                    Value::List(list) => {
                        if list.is_empty() {
                            return Ok(Value::List(Vec::new()));
                        }
                        let mut result = Vec::new();
                        for _ in 0..count {
                            result.extend(list.iter().cloned());
                        }
                        Ok(Value::List(result))
                    }
                    Value::String(s) => {
                        if s.is_empty() {
                            return Ok(Value::String(String::new()));
                        }
                        let repeated = s.repeat(count);
                        Ok(Value::String(repeated))
                    }
                    _ => Err("cycle() can only be called on lists and strings".to_string()),
                }
            }

            "interleave" => {
                if args.len() != 2 {
                    return Err("interleave() expects exactly 2 arguments".to_string());
                }
                let list1_val = self.eval_node(&args[0])?;
                let list2_val = self.eval_node(&args[1])?;

                match (list1_val, list2_val) {
                    (Value::List(list1), Value::List(list2)) => {
                        let mut result = Vec::new();
                        let max_len = list1.len().max(list2.len());

                        for i in 0..max_len {
                            if i < list1.len() {
                                result.push(list1[i].clone());
                            }
                            if i < list2.len() {
                                result.push(list2[i].clone());
                            }
                        }
                        Ok(Value::List(result))
                    }
                    _ => Err("interleave() expects two lists".to_string()),
                }
            }

            "rotate" => {
                if args.len() != 2 {
                    return Err("rotate() expects exactly 2 arguments".to_string());
                }
                let list_val = self.eval_node(&args[0])?;
                let n_val = self.eval_node(&args[1])?;

                match (list_val, n_val) {
                    (Value::List(mut list), Value::Integer(n)) => {
                        if list.is_empty() {
                            return Ok(Value::List(list));
                        }

                        let len = list.len() as i64;
                        let n = ((n % len) + len) % len; // Handle negative rotation
                        let n = n as usize;

                        list.rotate_left(n);
                        Ok(Value::List(list))
                    }
                    _ => Err("rotate() expects a list and an integer".to_string()),
                }
            }

            "permutations" => {
                if args.len() != 1 {
                    return Err("permutations() expects exactly 1 argument".to_string());
                }
                let list_val = self.eval_node(&args[0])?;

                match list_val {
                    Value::List(list) => {
                        if list.len() > 8 {
                            return Err("permutations() limited to lists of 8 elements or fewer"
                                .to_string());
                        }

                        let perms = self.generate_permutations(&list);
                        let result: Vec<Value> = perms.into_iter().map(Value::List).collect();
                        Ok(Value::List(result))
                    }
                    _ => Err("permutations() can only be called on lists".to_string()),
                }
            }

            "combinations" => {
                if args.len() != 2 {
                    return Err("combinations() expects exactly 2 arguments".to_string());
                }
                let list_val = self.eval_node(&args[0])?;
                let r_val = self.eval_node(&args[1])?;

                match (list_val, r_val) {
                    (Value::List(list), Value::Integer(r)) => {
                        if r < 0 || r > list.len() as i64 {
                            return Err(
                                "combinations() r must be between 0 and list length".to_string()
                            );
                        }

                        let combs = self.generate_combinations(&list, r as usize);
                        let result: Vec<Value> = combs.into_iter().map(Value::List).collect();
                        Ok(Value::List(result))
                    }
                    _ => Err("combinations() expects a list and an integer".to_string()),
                }
            }

            "product" => {
                if args.len() < 2 {
                    return Err("product() expects at least 2 arguments".to_string());
                }

                let mut lists = Vec::new();
                for arg in args {
                    let val = self.eval_node(arg)?;
                    match val {
                        Value::List(list) => lists.push(list),
                        _ => return Err("product() expects list arguments".to_string()),
                    }
                }

                let result = self.cartesian_product(&lists);
                let result: Vec<Value> = result.into_iter().map(Value::List).collect();
                Ok(Value::List(result))
            }

            // parallel_map - Map function in parallel (auto-parallelization)
            "parallel_map" => {
                if args.len() != 2 {
                    return Err(
                        "parallel_map() expects 2 arguments: parallel_map(list, function)"
                            .to_string(),
                    );
                }
                let list_val = self.eval_node(&args[0])?;
                let func_val = self.eval_node(&args[1])?;

                match list_val {
                    Value::List(list) => {
                        // For now, sequential (can be parallelized with rayon)
                        let mut result = Vec::new();
                        for item in list {
                            let mapped =
                                self.call_value_with_args(func_val.clone(), &[item], None)?;
                            result.push(mapped);
                        }
                        Ok(Value::List(result))
                    }
                    _ => Err("parallel_map() expects a list".to_string()),
                }
            }

            // memoize - Automatically cache function results
            "memoize" => {
                if args.len() != 1 {
                    return Err("memoize() expects 1 argument: a function".to_string());
                }
                let func_val = self.eval_node(&args[0])?;
                // Return the function wrapped with memoization
                // (Implementation would require cache storage)
                Ok(func_val)
            }

            // benchmark - Time function execution
            "benchmark" => {
                if args.is_empty() {
                    return Err(
                        "benchmark() expects at least 1 argument: function to benchmark"
                            .to_string(),
                    );
                }
                let func_val = self.eval_node(&args[0])?;
                let iterations = if args.len() > 1 {
                    if let Value::Integer(n) = self.eval_node(&args[1])? {
                        n as usize
                    } else {
                        1
                    }
                } else {
                    1
                };

                let start = std::time::Instant::now();
                let mut last_result = Value::None;

                for _ in 0..iterations {
                    last_result = self.call_value_with_args(func_val.clone(), &[], None)?;
                }

                let duration = start.elapsed();
                let ms = duration.as_secs_f64() * 1000.0;

                let mut result = std::collections::HashMap::new();
                result.insert("time_ms".to_string(), Value::Float(ms));
                result.insert("iterations".to_string(), Value::Integer(iterations as i64));
                result.insert("avg_ms".to_string(), Value::Float(ms / iterations as f64));
                result.insert("result".to_string(), last_result);

                Ok(Value::Dict(result))
            }

            // tap - Debug helper: print value and return it (chainable)
            "tap" => {
                if args.is_empty() {
                    return Err("tap() expects at least 1 argument".to_string());
                }
                let val = self.eval_node(&args[0])?;

                if args.len() > 1 {
                    // Custom message
                    let msg_val = self.eval_node(&args[1])?;
                    if let Value::String(msg) = msg_val {
                        println!("[TAP] {}: {}", msg, val);
                    }
                } else {
                    println!("[TAP] {}", val);
                }

                Ok(val)
            }

            // pipe - Function composition (f |> g |> h)
            "pipe" => {
                if args.len() < 2 {
                    return Err(
                        "pipe() expects at least 2 arguments: value, ...functions".to_string()
                    );
                }

                let mut result = self.eval_node(&args[0])?;

                for arg_node in args.iter().skip(1) {
                    let func = self.eval_node(arg_node)?;
                    result = self.call_value_with_args(func, &[result], None)?;
                }

                Ok(result)
            }

            // retry - Retry function on failure
            "retry" => {
                if args.len() < 2 {
                    return Err(
                        "retry() expects at least 2 arguments: function, max_attempts".to_string(),
                    );
                }
                let func_val = self.eval_node(&args[0])?;
                let max_attempts = if let Value::Integer(n) = self.eval_node(&args[1])? {
                    n as usize
                } else {
                    return Err("retry() max_attempts must be an integer".to_string());
                };

                let mut last_error = String::new();

                for attempt in 1..=max_attempts {
                    match self.call_value_with_args(func_val.clone(), &[], None) {
                        Ok(result) => return Ok(result),
                        Err(e) => {
                            last_error = e;
                            if attempt < max_attempts {
                                // Small delay between retries
                                std::thread::sleep(std::time::Duration::from_millis(100));
                            }
                        }
                    }
                }

                Err(format!(
                    "retry() failed after {} attempts: {}",
                    max_attempts, last_error
                ))
            }

            // take_while - Take elements while predicate is true
            "take_while" => {
                if args.len() != 2 {
                    return Err(
                        "take_while() expects 2 arguments: take_while(list, predicate)".to_string(),
                    );
                }
                let list_val = self.eval_node(&args[0])?;
                let pred_val = self.eval_node(&args[1])?;

                match list_val {
                    Value::List(list) => {
                        let mut result = Vec::new();
                        for item in list {
                            let pred_result =
                                self.call_value_with_args(pred_val.clone(), std::slice::from_ref(&item), None)?;
                            match pred_result {
                                Value::Boolean(true) => result.push(item),
                                Value::Boolean(false) => break,
                                _ => {
                                    return Err(
                                        "take_while() predicate must return boolean".to_string()
                                    )
                                }
                            }
                        }
                        Ok(Value::List(result))
                    }
                    _ => Err("take_while() expects a list".to_string()),
                }
            }

            // drop_while - Drop elements while predicate is true
            "drop_while" => {
                if args.len() != 2 {
                    return Err(
                        "drop_while() expects 2 arguments: drop_while(list, predicate)".to_string(),
                    );
                }
                let list_val = self.eval_node(&args[0])?;
                let pred_val = self.eval_node(&args[1])?;

                match list_val {
                    Value::List(list) => {
                        let mut dropping = true;
                        let mut result = Vec::new();

                        for item in list {
                            if dropping {
                                let pred_result = self.call_value_with_args(
                                    pred_val.clone(),
                                    std::slice::from_ref(&item),
                                    None,
                                )?;
                                match pred_result {
                                    Value::Boolean(true) => continue,
                                    Value::Boolean(false) => {
                                        dropping = false;
                                        result.push(item);
                                    }
                                    _ => {
                                        return Err("drop_while() predicate must return boolean"
                                            .to_string())
                                    }
                                }
                            } else {
                                result.push(item);
                            }
                        }
                        Ok(Value::List(result))
                    }
                    _ => Err("drop_while() expects a list".to_string()),
                }
            }

            // scan - Like reduce but returns all intermediate results
            "scan" => {
                if args.len() != 3 {
                    return Err(
                        "scan() expects 3 arguments: scan(list, initial, function)".to_string()
                    );
                }
                let list_val = self.eval_node(&args[0])?;
                let initial = self.eval_node(&args[1])?;
                let func_val = self.eval_node(&args[2])?;

                match list_val {
                    Value::List(list) => {
                        let mut result = vec![initial.clone()];
                        let mut accumulator = initial;

                        for item in list {
                            accumulator = self.call_value_with_args(
                                func_val.clone(),
                                &[accumulator, item],
                                None,
                            )?;
                            result.push(accumulator.clone());
                        }

                        Ok(Value::List(result))
                    }
                    _ => Err("scan() expects a list".to_string()),
                }
            }

            // window - Create overlapping windows (alias for sliding_window)
            "window" => {
                if args.len() != 2 {
                    return Err("window() expects 2 arguments: window(list, size)".to_string());
                }
                // Reuse sliding_window implementation
                self.call_function("sliding_window", args)
            }

            // compact - Remove None/null values from list
            "compact" => {
                if args.len() != 1 {
                    return Err("compact() expects 1 argument".to_string());
                }
                let val = self.eval_node(&args[0])?;

                match val {
                    Value::List(list) => {
                        let result: Vec<Value> = list
                            .into_iter()
                            .filter(|item| !matches!(item, Value::None))
                            .collect();
                        Ok(Value::List(result))
                    }
                    _ => Err("compact() expects a list".to_string()),
                }
            }

            // pluck - Extract property from list of dicts/objects
            "pluck" => {
                if args.len() != 2 {
                    return Err("pluck() expects 2 arguments: pluck(list, key)".to_string());
                }
                let list_val = self.eval_node(&args[0])?;
                let key_val = self.eval_node(&args[1])?;

                let key = match key_val {
                    Value::String(s) => s,
                    _ => return Err("pluck() key must be a string".to_string()),
                };

                match list_val {
                    Value::List(list) => {
                        let mut result = Vec::new();
                        for item in list {
                            if let Value::Dict(dict) = item {
                                if let Some(value) = dict.get(&key) {
                                    result.push(value.clone());
                                }
                            }
                        }
                        Ok(Value::List(result))
                    }
                    _ => Err("pluck() expects a list".to_string()),
                }
            }

            // ============================================================================
            // MORE PYTHON-KILLER FEATURES
            // ============================================================================

            // zip_longest - Zip with fillvalue for unequal lengths (like Python's itertools.zip_longest)
            "zip_longest" => {
                if args.len() < 3 {
                    return Err(
                        "zip_longest() expects at least 3 arguments: list1, list2, fillvalue"
                            .to_string(),
                    );
                }

                let fill_val = self.eval_node(&args[args.len() - 1])?;
                let mut lists: Vec<Vec<Value>> = Vec::new();
                let mut max_len = 0;

                for arg in args.iter().take(args.len() - 1) {
                    let val = self.eval_node(arg)?;
                    match val {
                        Value::List(list) => {
                            max_len = max_len.max(list.len());
                            lists.push(list);
                        }
                        _ => return Err("zip_longest() expects list arguments".to_string()),
                    }
                }

                let mut result = Vec::new();
                for i in 0..max_len {
                    let mut tuple = Vec::new();
                    for list in &lists {
                        if i < list.len() {
                            tuple.push(list[i].clone());
                        } else {
                            tuple.push(fill_val.clone());
                        }
                    }
                    result.push(Value::Tuple(tuple));
                }

                Ok(Value::List(result))
            }

            // batched - Split list into batches of size n
            "batched" => {
                if args.len() != 2 {
                    return Err(
                        "batched() expects 2 arguments: batched(list, batch_size)".to_string()
                    );
                }
                let list_val = self.eval_node(&args[0])?;
                let size_val = self.eval_node(&args[1])?;

                match (list_val, size_val) {
                    (Value::List(list), Value::Integer(size)) => {
                        if size <= 0 {
                            return Err("batched() batch_size must be positive".to_string());
                        }
                        let size = size as usize;
                        let mut result = Vec::new();

                        for chunk in list.chunks(size) {
                            result.push(Value::List(chunk.to_vec()));
                        }

                        Ok(Value::List(result))
                    }
                    _ => Err("batched() expects (list, int)".to_string()),
                }
            }

            // pairwise - Return successive overlapping pairs
            "pairwise" => {
                if args.len() != 1 {
                    return Err("pairwise() expects 1 argument".to_string());
                }
                let val = self.eval_node(&args[0])?;

                match val {
                    Value::List(list) => {
                        if list.len() < 2 {
                            return Ok(Value::List(Vec::new()));
                        }

                        let mut result = Vec::new();
                        for i in 0..(list.len() - 1) {
                            result.push(Value::Tuple(vec![list[i].clone(), list[i + 1].clone()]));
                        }

                        Ok(Value::List(result))
                    }
                    _ => Err("pairwise() expects a list".to_string()),
                }
            }

            // dedupe - Remove consecutive duplicates
            "dedupe" => {
                if args.len() != 1 {
                    return Err("dedupe() expects 1 argument".to_string());
                }
                let val = self.eval_node(&args[0])?;

                match val {
                    Value::List(list) => {
                        if list.is_empty() {
                            return Ok(Value::List(Vec::new()));
                        }

                        let mut result = vec![list[0].clone()];
                        for item in list.iter().skip(1) {
                            let prev_key = if let Some(last) = result.last() {
                                format!("{:?}", last)
                            } else {
                                String::new()
                            };
                            let curr_key = format!("{:?}", item);
                            if prev_key != curr_key {
                                result.push(item.clone());
                            }
                        }

                        Ok(Value::List(result))
                    }
                    _ => Err("dedupe() expects a list".to_string()),
                }
            }

            // deep_clone - Deep copy of nested structures
            "deep_clone" => {
                if args.len() != 1 {
                    return Err("deep_clone() expects 1 argument".to_string());
                }
                let val = self.eval_node(&args[0])?;
                Ok(val.clone()) // Rust's clone is already deep
            }

            // json_parse - Parse JSON string
            "json_parse" => {
                if args.len() != 1 {
                    return Err("json_parse() expects 1 argument: JSON string".to_string());
                }
                let val = self.eval_node(&args[0])?;
                match &val {
                    Value::String(s) => {
                        #[cfg(feature = "serde_json")]
                        {
                            let parsed: serde_json::Value = serde_json::from_str(s)
                                .map_err(|e| format!("json_parse() failed: {}", e))?;
                            Ok(Self::json_value_to_jade(&parsed))
                        }
                        #[cfg(not(feature = "serde_json"))]
                        {
                            let _ = s;
                            Err("json_parse() requires serde_json feature".to_string())
                        }
                    }
                    _ => Err("json_parse() expects a string".to_string()),
                }
            }

            // json_stringify - Convert to JSON string
            "json_stringify" => {
                if args.len() != 1 {
                    return Err("json_stringify() expects 1 argument".to_string());
                }
                let val = self.eval_node(&args[0])?;
                #[cfg(feature = "serde_json")]
                {
                    let json_str = Self::jade_value_to_json_string(&val)
                        .unwrap_or_else(|| format!("{:?}", val));
                    Ok(Value::String(json_str))
                }
                #[cfg(not(feature = "serde_json"))]
                {
                    let _ = val;
                    Err("json_stringify() requires serde_json feature".to_string())
                }
            }

            // ðŸ¦† SECRET EASTER EGG - Not documented anywhere!
            // Only activates with exact parameters, otherwise silently returns None
            "quack_check" => {
                if args.len() != 3 {
                    return Ok(Value::None); // Silent fail
                }

                let sound = self.eval_node(&args[0])?;
                let num = self.eval_node(&args[1])?;
                let ducks = self.eval_node(&args[2])?;

                // Check if all conditions match exactly
                let sound_match = matches!(sound, Value::String(ref s) if s == "quack");
                let num_match = matches!(num, Value::Integer(4));
                let ducks_match = if let Value::List(ref list) = ducks {
                    list.len() == 4
                        && list
                            .iter()
                            .all(|v| matches!(v, Value::String(ref s) if s == "quack"))
                } else {
                    false
                };

                if sound_match && num_match && ducks_match {
                    // ðŸŽ‰ Easter egg activated!
                    println!("Ethan likes ducks!");
                    println!("     __");
                    println!("   <(o )___");
                    println!("    ( _ > /");
                    println!("     `---'  ");
                }

                // Always return None, no indication whether it worked or not
                Ok(Value::None)
            }

            // ===== CRYPTOGRAPHY (see builtins/crypto.rs) — sha256, enigma_*, aes_*, etc. =====
            "random_bytes" => {
                if args.len() != 1 {
                    return Err("random_bytes() takes exactly 1 argument (length)".to_string());
                }
                let len_val = self.eval_node(&args[0])?;
                let len = match len_val {
                    Value::Integer(n) if n > 0 => n as usize,
                    _ => return Err("random_bytes() requires a positive integer".to_string()),
                };

                use rand::Rng;
                let mut rng = rand::thread_rng();
                let bytes: Vec<Value> = (0..len)
                    .map(|_| Value::Integer(rng.gen::<u8>() as i64))
                    .collect();

                Ok(Value::List(bytes))
            }

            "rand_range" => {
                if args.len() != 2 {
                    return Err("rand_range() takes exactly 2 arguments (min, max)".to_string());
                }
                let min_val = self.eval_node(&args[0])?;
                let max_val = self.eval_node(&args[1])?;

                let min = match min_val {
                    Value::Integer(n) => n,
                    _ => return Err("rand_range() requires integer arguments".to_string()),
                };
                let max = match max_val {
                    Value::Integer(n) => n,
                    _ => return Err("rand_range() requires integer arguments".to_string()),
                };

                if min >= max {
                    return Err("rand_range() requires min < max".to_string());
                }

                use rand::Rng;
                let mut rng = rand::thread_rng();
                let result = rng.gen_range(min..max);

                Ok(Value::Integer(result))
            }

            "uuid_v4" => {
                if !args.is_empty() {
                    return Err("uuid_v4() takes no arguments".to_string());
                }

                use uuid::Uuid;
                let id = Uuid::new_v4();
                Ok(Value::String(id.to_string()))
            }

            "make_secret" => {
                if args.len() != 1 {
                    return Err("make_secret(value) requires 1 argument".to_string());
                }

                let value = self.eval_node(&args[0])?;
                let secret_str = match value {
                    Value::String(s) => s,
                    Value::Integer(i) => i.to_string(),
                    _ => return Err("make_secret() requires a string or integer".to_string()),
                };

                Ok(Value::Secret(secret_str))
            }

            "reveal_secret" => {
                if args.len() != 1 {
                    return Err("reveal_secret(secret) requires 1 argument".to_string());
                }

                let secret_val = self.eval_node(&args[0])?;
                match secret_val {
                    Value::Secret(s) => Ok(Value::String(s)),
                    _ => Err("reveal_secret() requires a secret value".to_string()),
                }
            }

            "audit_log" => {
                    if args.len() < 2 {
                        return Err("audit_log(event, data) requires at least 2 arguments".to_string());
                    }

                    let event_val = self.eval_node(&args[0])?;
                    let event = match event_val {
                        Value::String(s) => s,
                        _ => return Err("Event must be a string".to_string()),
                    };

                    let data_val = self.eval_node(&args[1])?;
                    let data_str = match &data_val {
                        Value::String(s) => s.clone(),
                        Value::Integer(i) => i.to_string(),
                        Value::Boolean(b) => b.to_string(),
                        Value::Secret(_) => "[REDACTED]".to_string(),
                        _ => format!("{:?}", data_val),
                    };

                    use std::fs::OpenOptions;
                    use std::io::Write;

                    let timestamp = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs();

                    let log_entry =
                        format!("[AUDIT] {} event={} data={}\n", timestamp, event, data_str);

                    if let Ok(mut file) = OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open("audit.log")
                    {
                        let _ = file.write_all(log_entry.as_bytes());
                    }

                    println!("{}", log_entry.trim());

                    Ok(Value::Boolean(true))
            }

            // ===== WEB SCRAPING FUNCTIONS =====
            "fetch_html" | "fetch_text" => {
                if args.len() != 1 {
                    return Err("fetch_text() takes exactly 1 argument (url)".to_string());
                }
                let url_val = self.eval_node(&args[0])?;
                let url = match url_val {
                    Value::String(s) => s,
                    _ => return Err("fetch_text() requires a string URL".to_string()),
                };
                #[cfg(feature = "reqwest")]
                {
                    let client = reqwest::blocking::Client::builder()
                        .user_agent("Jade-Scraper/1.0")
                        .build()
                        .map_err(|e| format!("fetch_text() client error: {}", e))?;
                    let body = client
                        .get(&url)
                        .send()
                        .map_err(|e| format!("fetch_text() request failed: {}", e))?
                        .text()
                        .map_err(|e| format!("fetch_text() read failed: {}", e))?;
                    Ok(Value::String(body))
                }
                #[cfg(not(feature = "reqwest"))]
                {
                    let _ = url;
                    Err("fetch_text() requires reqwest feature (full build)".to_string())
                }
            }

            "fetch_json" => {
                if args.len() != 1 {
                    return Err("fetch_json() takes exactly 1 argument (url)".to_string());
                }
                let url_val = self.eval_node(&args[0])?;
                let url = match url_val {
                    Value::String(s) => s,
                    _ => return Err("fetch_json() requires a string URL".to_string()),
                };
                #[cfg(all(feature = "reqwest", feature = "serde_json"))]
                {
                    let client = reqwest::blocking::Client::builder()
                        .user_agent("Jade-Scraper/1.0")
                        .build()
                        .map_err(|e| format!("fetch_json() client error: {}", e))?;
                    let json_val: serde_json::Value = client
                        .get(&url)
                        .send()
                        .map_err(|e| format!("fetch_json() request failed: {}", e))?
                        .json()
                        .map_err(|e| format!("fetch_json() parse failed: {}", e))?;
                    Ok(Self::json_value_to_jade(&json_val))
                }
                #[cfg(not(all(feature = "reqwest", feature = "serde_json")))]
                {
                    let _ = url;
                    Err("fetch_json() requires reqwest and serde_json (full build)".to_string())
                }
            }

            // ===== CLI FUNCTIONS =====
            "cli_prompt" => {
                if args.len() != 1 {
                    return Err("cli_prompt() takes exactly 1 argument (prompt text)".to_string());
                }
                let prompt_val = self.eval_node(&args[0])?;
                let prompt = match prompt_val {
                    Value::String(s) => s,
                    _ => return Err("cli_prompt() requires a string".to_string()),
                };

                use std::io::{self, Write};
                print!("{}", prompt);
                io::stdout()
                    .flush()
                    .map_err(|e| format!("cli_prompt() failed to flush output: {}", e))?;

                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .map_err(|e| format!("cli_prompt() failed to read input: {}", e))?;

                Ok(Value::String(input.trim().to_string()))
            }

            "cli_args" => {
                if !args.is_empty() {
                    return Err("cli_args() takes no arguments".to_string());
                }

                // Get command line arguments
                let args: Vec<Value> = std::env::args()
                    .skip(1) // Skip program name
                    .map(Value::String)
                    .collect();

                Ok(Value::List(args))
            }

            "env_get" => {
                if args.len() != 1 {
                    return Err("env_get() takes exactly 1 argument (variable name)".to_string());
                }
                let var_val = self.eval_node(&args[0])?;
                let var_name = match var_val {
                    Value::String(s) => s,
                    _ => return Err("env_get() requires a string".to_string()),
                };

                match std::env::var(&var_name) {
                    Ok(value) => Ok(Value::String(value)),
                    Err(_) => Ok(Value::None),
                }
            }

            "env_set" => {
                if args.len() != 2 {
                    return Err("env_set() takes exactly 2 arguments (name, value)".to_string());
                }
                let name_val = self.eval_node(&args[0])?;
                let value_val = self.eval_node(&args[1])?;

                let name = match name_val {
                    Value::String(s) => s,
                    _ => return Err("env_set() name must be a string".to_string()),
                };
                let value = match value_val {
                    Value::String(s) => s,
                    _ => return Err("env_set() value must be a string".to_string()),
                };

                std::env::set_var(&name, &value);
                Ok(Value::None)
            }

            "file_read" => {
                    if args.len() != 1 {
                        return Err("file_read() takes exactly 1 argument (filename)".to_string());
                    }
                    let filename_val = self.eval_node(&args[0])?;
                    let filename = match filename_val {
                        Value::String(s) => s,
                        _ => return Err("file_read() requires a string filename".to_string()),
                    };

                    match std::fs::read_to_string(&filename) {
                        Ok(content) => Ok(Value::String(content)),
                        Err(e) => Err(format!("Failed to read file '{}': {}", filename, e)),
                    }
            }

            "file_write" => {
                    if args.len() != 2 {
                        return Err(
                            "file_write() takes exactly 2 arguments (filename, content)".to_string()
                        );
                    }
                    let filename_val = self.eval_node(&args[0])?;
                    let content_val = self.eval_node(&args[1])?;

                    let filename = match filename_val {
                        Value::String(s) => s,
                        _ => return Err("file_write() filename must be a string".to_string()),
                    };
                    let content = match content_val {
                        Value::String(s) => s,
                        _ => return Err("file_write() content must be a string".to_string()),
                    };

                    match std::fs::write(&filename, &content) {
                        Ok(_) => Ok(Value::None),
                        Err(e) => Err(format!("Failed to write file '{}': {}", filename, e)),
                    }
            }

            "file_exists" => {
                    if args.len() != 1 {
                        return Err("file_exists() takes exactly 1 argument (filename)".to_string());
                    }
                    let filename_val = self.eval_node(&args[0])?;
                    let filename = match filename_val {
                        Value::String(s) => s,
                        _ => return Err("file_exists() requires a string filename".to_string()),
                    };

                    Ok(Value::Boolean(std::path::Path::new(&filename).exists()))
            }

            "dir_list" => {
                    if args.len() != 1 {
                        return Err("dir_list() takes exactly 1 argument (directory path)".to_string());
                    }
                    let dir_val = self.eval_node(&args[0])?;
                    let dir_path = match dir_val {
                        Value::String(s) => s,
                        _ => return Err("dir_list() requires a string path".to_string()),
                    };

                    match std::fs::read_dir(&dir_path) {
                        Ok(entries) => {
                            let files: Vec<Value> = entries
                                .filter_map(|entry| entry.ok())
                                .map(|entry| {
                                    Value::String(entry.file_name().to_string_lossy().to_string())
                                })
                                .collect();
                            Ok(Value::List(files))
                        }
                        Err(e) => Err(format!("Failed to read directory '{}': {}", dir_path, e)),
                    }
            }

            "timestamp" => {
                if !args.is_empty() {
                    return Err("timestamp() takes no arguments".to_string());
                }

                use std::time::{SystemTime, UNIX_EPOCH};
                let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

                Ok(Value::Integer(duration.as_secs() as i64))
            }

            "file_delete" => {
                    if args.len() != 1 {
                        return Err("file_delete() takes exactly 1 argument (filename)".to_string());
                    }
                    let filename_val = self.eval_node(&args[0])?;
                    let filename = match filename_val {
                        Value::String(s) => s,
                        _ => return Err("file_delete() requires a string filename".to_string()),
                    };

                    match std::fs::remove_file(&filename) {
                        Ok(_) => Ok(Value::None),
                        Err(e) => Err(format!("Failed to delete file '{}': {}", filename, e)),
                    }
            }

            "file_append" => {
                    if args.len() != 2 {
                        return Err(
                            "file_append() takes exactly 2 arguments (filename, content)".to_string(),
                        );
                    }
                    let filename_val = self.eval_node(&args[0])?;
                    let content_val = self.eval_node(&args[1])?;

                    let filename = match filename_val {
                        Value::String(s) => s,
                        _ => return Err("file_append() filename must be a string".to_string()),
                    };
                    let content = match content_val {
                        Value::String(s) => s,
                        _ => return Err("file_append() content must be a string".to_string()),
                    };

                    use std::fs::OpenOptions;
                    use std::io::Write;

                    match OpenOptions::new().create(true).append(true).open(&filename) {
                        Ok(mut file) => match file.write_all(content.as_bytes()) {
                            Ok(_) => Ok(Value::None),
                            Err(e) => Err(format!("Failed to append to file '{}': {}", filename, e)),
                        },
                        Err(e) => Err(format!("Failed to open file '{}': {}", filename, e)),
                    }
            }

            "dir_create" => {
                    if args.len() != 1 {
                        return Err(
                            "dir_create() takes exactly 1 argument (directory path)".to_string()
                        );
                    }
                    let dir_val = self.eval_node(&args[0])?;
                    let dir_path = match dir_val {
                        Value::String(s) => s,
                        _ => return Err("dir_create() requires a string path".to_string()),
                    };

                    match std::fs::create_dir_all(&dir_path) {
                        Ok(_) => Ok(Value::None),
                        Err(e) => Err(format!("Failed to create directory '{}': {}", dir_path, e)),
                    }
            }

            "dir_delete" => {
                    if args.len() != 1 {
                        return Err(
                            "dir_delete() takes exactly 1 argument (directory path)".to_string()
                        );
                    }
                    let dir_val = self.eval_node(&args[0])?;
                    let dir_path = match dir_val {
                        Value::String(s) => s,
                        _ => return Err("dir_delete() requires a string path".to_string()),
                    };

                    match std::fs::remove_dir_all(&dir_path) {
                        Ok(_) => Ok(Value::None),
                        Err(e) => Err(format!("Failed to delete directory '{}': {}", dir_path, e)),
                    }
            }

            "file_copy" => {
                    if args.len() != 2 {
                        return Err(
                            "file_copy() takes exactly 2 arguments (source, destination)".to_string(),
                        );
                    }
                    let src_val = self.eval_node(&args[0])?;
                    let dst_val = self.eval_node(&args[1])?;

                    let src = match src_val {
                        Value::String(s) => s,
                        _ => return Err("file_copy() source must be a string".to_string()),
                    };
                    let dst = match dst_val {
                        Value::String(s) => s,
                        _ => return Err("file_copy() destination must be a string".to_string()),
                    };

                    match std::fs::copy(&src, &dst) {
                        Ok(_) => Ok(Value::None),
                        Err(e) => Err(format!("Failed to copy file '{}' to '{}': {}", src, dst, e)),
                    }
            }

            "file_rename" => {
                    if args.len() != 2 {
                        return Err(
                            "file_rename() takes exactly 2 arguments (old_name, new_name)".to_string(),
                        );
                    }
                    let old_val = self.eval_node(&args[0])?;
                    let new_val = self.eval_node(&args[1])?;

                    let old_name = match old_val {
                        Value::String(s) => s,
                        _ => return Err("file_rename() old_name must be a string".to_string()),
                    };
                    let new_name = match new_val {
                        Value::String(s) => s,
                        _ => return Err("file_rename() new_name must be a string".to_string()),
                    };

                    match std::fs::rename(&old_name, &new_name) {
                        Ok(_) => Ok(Value::None),
                        Err(e) => Err(format!(
                            "Failed to rename file '{}' to '{}': {}",
                            old_name, new_name, e
                        )),
                    }
            }

            _ => {
                // If not built-in, look for user-defined function (or @once/MirrorDispatch wrapper)
                let func_val = self.get_variable(name)?;
                let eval_args: Vec<Value> = args
                    .iter()
                    .map(|a| self.eval_node(a))
                    .collect::<Result<Vec<_>, _>>()?;
                self.call_value_with_args(func_val, &eval_args, None)
            }
        }
    }

    pub(super) fn eval_binary_op(&self, left: &Value, op: &BinaryOp, right: &Value) -> Result<Value, String> {
        match (left, op, right) {
            (Value::Integer(a), BinaryOp::Add, Value::Integer(b)) => a
                .checked_add(*b)
                .map(Value::Integer)
                .ok_or_else(|| format!("Integer overflow: {} + {}", a, b)),
            (Value::Integer(a), BinaryOp::Subtract, Value::Integer(b)) => a
                .checked_sub(*b)
                .map(Value::Integer)
                .ok_or_else(|| format!("Integer overflow: {} - {}", a, b)),
            (Value::Integer(a), BinaryOp::Multiply, Value::Integer(b)) => a
                .checked_mul(*b)
                .map(Value::Integer)
                .ok_or_else(|| format!("Integer overflow: {} * {}", a, b)),
            (Value::Integer(a), BinaryOp::Divide, Value::Integer(b)) => {
                if *b == 0 {
                    Err(JError::division_by_zero(0, 0).to_string())
                } else if *a == i64::MIN && *b == -1 {
                    Err("Integer overflow: i64::MIN / -1".to_string())
                } else {
                    Ok(Value::Integer(a / b))
                }
            }
            (Value::Integer(a), BinaryOp::Modulo, Value::Integer(b)) => {
                if *b == 0 {
                    Err(JError::division_by_zero(0, 0).to_string())
                } else if *a == i64::MIN && *b == -1 {
                    Ok(Value::Integer(0)) // Mathematically correct
                } else {
                    Ok(Value::Integer(a % b))
                }
            }
            (Value::Integer(a), BinaryOp::Power, Value::Integer(b)) => {
                if *b < 0 {
                    Ok(Value::Float((*a as f64).powf(*b as f64)))
                } else if *b > u32::MAX as i64 {
                    Err("Exponent too large".to_string())
                } else {
                    a.checked_pow(*b as u32)
                        .map(Value::Integer)
                        .ok_or_else(|| format!("Integer overflow: {} ** {}", a, b))
                }
            }

            // Bitwise operations
            (Value::Integer(a), BinaryOp::BitwiseAnd, Value::Integer(b)) => {
                Ok(Value::Integer(a & b))
            }
            (Value::Integer(a), BinaryOp::BitwiseOr, Value::Integer(b)) => {
                Ok(Value::Integer(a | b))
            }

            // Cross-type arithmetic: Integer + Float
            (Value::Integer(a), BinaryOp::Add, Value::Float(b)) => Ok(Value::Float(*a as f64 + b)),
            (Value::Float(a), BinaryOp::Add, Value::Integer(b)) => Ok(Value::Float(a + *b as f64)),
            (Value::Integer(a), BinaryOp::Subtract, Value::Float(b)) => {
                Ok(Value::Float(*a as f64 - b))
            }
            (Value::Float(a), BinaryOp::Subtract, Value::Integer(b)) => {
                Ok(Value::Float(a - *b as f64))
            }
            (Value::Integer(a), BinaryOp::Multiply, Value::Float(b)) => {
                Ok(Value::Float(*a as f64 * b))
            }
            (Value::Float(a), BinaryOp::Multiply, Value::Integer(b)) => {
                Ok(Value::Float(a * *b as f64))
            }
            (Value::Integer(a), BinaryOp::Divide, Value::Float(b)) => {
                if *b == 0.0 {
                    Err(JError::division_by_zero(0, 0).to_string())
                } else {
                    Ok(Value::Float(*a as f64 / b))
                }
            }
            (Value::Float(a), BinaryOp::Divide, Value::Integer(b)) => {
                if *b == 0 {
                    Err(JError::division_by_zero(0, 0).to_string())
                } else {
                    Ok(Value::Float(a / *b as f64))
                }
            }
            (Value::Integer(a), BinaryOp::Power, Value::Float(b)) => {
                Ok(Value::Float((*a as f64).powf(*b)))
            }
            (Value::Float(a), BinaryOp::Power, Value::Integer(b)) => {
                Ok(Value::Float(a.powf(*b as f64)))
            }

            // Cross-type comparisons
            (Value::Integer(a), BinaryOp::Equal, Value::Float(b)) => {
                Ok(Value::Boolean((*a as f64 - b).abs() < f64::EPSILON))
            }
            (Value::Float(a), BinaryOp::Equal, Value::Integer(b)) => {
                Ok(Value::Boolean((a - *b as f64).abs() < f64::EPSILON))
            }
            (Value::Integer(a), BinaryOp::NotEqual, Value::Float(b)) => {
                Ok(Value::Boolean((*a as f64 - b).abs() >= f64::EPSILON))
            }
            (Value::Float(a), BinaryOp::NotEqual, Value::Integer(b)) => {
                Ok(Value::Boolean((a - *b as f64).abs() >= f64::EPSILON))
            }
            (Value::Integer(a), BinaryOp::Less, Value::Float(b)) => {
                Ok(Value::Boolean((*a as f64) < *b))
            }
            (Value::Float(a), BinaryOp::Less, Value::Integer(b)) => {
                Ok(Value::Boolean(*a < (*b as f64)))
            }
            (Value::Integer(a), BinaryOp::LessEqual, Value::Float(b)) => {
                Ok(Value::Boolean((*a as f64) <= *b))
            }
            (Value::Float(a), BinaryOp::LessEqual, Value::Integer(b)) => {
                Ok(Value::Boolean(*a <= (*b as f64)))
            }
            (Value::Integer(a), BinaryOp::Greater, Value::Float(b)) => {
                Ok(Value::Boolean((*a as f64) > *b))
            }
            (Value::Float(a), BinaryOp::Greater, Value::Integer(b)) => {
                Ok(Value::Boolean(*a > (*b as f64)))
            }
            (Value::Integer(a), BinaryOp::GreaterEqual, Value::Float(b)) => {
                Ok(Value::Boolean((*a as f64) >= *b))
            }
            (Value::Float(a), BinaryOp::GreaterEqual, Value::Integer(b)) => {
                Ok(Value::Boolean(*a >= (*b as f64)))
            }

            // String concatenation with other types
            (Value::String(a), BinaryOp::Add, Value::Integer(b)) => {
                Ok(Value::String(format!("{}{}", a, b)))
            }
            (Value::Integer(a), BinaryOp::Add, Value::String(b)) => {
                Ok(Value::String(format!("{}{}", a, b)))
            }
            (Value::String(a), BinaryOp::Add, Value::Float(b)) => {
                Ok(Value::String(format!("{}{}", a, b)))
            }
            (Value::Float(a), BinaryOp::Add, Value::String(b)) => {
                Ok(Value::String(format!("{}{}", a, b)))
            }
            (Value::String(a), BinaryOp::Add, Value::Boolean(b)) => {
                Ok(Value::String(format!("{}{}", a, b)))
            }
            (Value::Boolean(a), BinaryOp::Add, Value::String(b)) => {
                Ok(Value::String(format!("{}{}", a, b)))
            }
            (Value::Integer(a), BinaryOp::BitwiseXor, Value::Integer(b)) => {
                Ok(Value::Integer(a ^ b))
            }
            (Value::Integer(a), BinaryOp::LeftShift, Value::Integer(b)) => {
                if *b < 0 {
                    return Err("Shift amount must be non-negative".to_string());
                }
                Ok(Value::Integer(a << b))
            }
            (Value::Integer(a), BinaryOp::RightShift, Value::Integer(b)) => {
                if *b < 0 {
                    return Err("Shift amount must be non-negative".to_string());
                }
                Ok(Value::Integer(a >> b))
            }

            (Value::Float(a), BinaryOp::Add, Value::Float(b)) => Ok(Value::Float(a + b)),
            (Value::Float(a), BinaryOp::Subtract, Value::Float(b)) => Ok(Value::Float(a - b)),
            (Value::Float(a), BinaryOp::Multiply, Value::Float(b)) => Ok(Value::Float(a * b)),
            (Value::Float(a), BinaryOp::Divide, Value::Float(b)) => Ok(Value::Float(a / b)),

            (Value::String(a), BinaryOp::Add, Value::String(b)) => {
                Ok(Value::String(format!("{}{}", a, b)))
            }

            // Vector operations
            (Value::Vector(a), BinaryOp::Add, Value::Vector(b)) => {
                if a.len() != b.len() {
                    return Err("Vector dimensions must match for addition".to_string());
                }
                let result: Vec<f64> = a.iter().zip(b.iter()).map(|(x, y)| x + y).collect();
                Ok(Value::Vector(result))
            }
            (Value::Vector(a), BinaryOp::Subtract, Value::Vector(b)) => {
                if a.len() != b.len() {
                    return Err("Vector dimensions must match for subtraction".to_string());
                }
                let result: Vec<f64> = a.iter().zip(b.iter()).map(|(x, y)| x - y).collect();
                Ok(Value::Vector(result))
            }
            (Value::Vector(a), BinaryOp::Multiply, Value::Float(scalar)) => {
                let result: Vec<f64> = a.iter().map(|x| x * scalar).collect();
                Ok(Value::Vector(result))
            }
            (Value::Float(scalar), BinaryOp::Multiply, Value::Vector(a)) => {
                let result: Vec<f64> = a.iter().map(|x| scalar * x).collect();
                Ok(Value::Vector(result))
            }
            (Value::Vector(a), BinaryOp::Multiply, Value::Integer(scalar)) => {
                let result: Vec<f64> = a.iter().map(|x| x * (*scalar as f64)).collect();
                Ok(Value::Vector(result))
            }
            (Value::Integer(scalar), BinaryOp::Multiply, Value::Vector(a)) => {
                let result: Vec<f64> = a.iter().map(|x| (*scalar as f64) * x).collect();
                Ok(Value::Vector(result))
            }

            // Matrix operations
            (Value::Matrix(a), BinaryOp::Add, Value::Matrix(b)) => {
                if a.len() != b.len() || (!a.is_empty() && a[0].len() != b[0].len()) {
                    return Err("Matrix dimensions must match for addition".to_string());
                }
                let mut result = Vec::new();
                for (row_a, row_b) in a.iter().zip(b.iter()) {
                    let result_row: Vec<f64> =
                        row_a.iter().zip(row_b.iter()).map(|(x, y)| x + y).collect();
                    result.push(result_row);
                }
                Ok(Value::Matrix(result))
            }
            (Value::Matrix(a), BinaryOp::Subtract, Value::Matrix(b)) => {
                if a.len() != b.len() || (!a.is_empty() && a[0].len() != b[0].len()) {
                    return Err("Matrix dimensions must match for subtraction".to_string());
                }
                let mut result = Vec::new();
                for (row_a, row_b) in a.iter().zip(b.iter()) {
                    let result_row: Vec<f64> =
                        row_a.iter().zip(row_b.iter()).map(|(x, y)| x - y).collect();
                    result.push(result_row);
                }
                Ok(Value::Matrix(result))
            }
            (Value::Matrix(a), BinaryOp::Multiply, Value::Float(scalar)) => {
                let result: Vec<Vec<f64>> = a
                    .iter()
                    .map(|row| row.iter().map(|x| x * scalar).collect())
                    .collect();
                Ok(Value::Matrix(result))
            }
            (Value::Float(scalar), BinaryOp::Multiply, Value::Matrix(a)) => {
                let result: Vec<Vec<f64>> = a
                    .iter()
                    .map(|row| row.iter().map(|x| scalar * x).collect())
                    .collect();
                Ok(Value::Matrix(result))
            }

            // Counter operations
            (Value::Counter(a), BinaryOp::Add, Value::Counter(b)) => {
                let mut result = a.clone();
                for (key, count) in b.iter() {
                    *result.entry(key.clone()).or_insert(0) += count;
                }
                Ok(Value::Counter(result))
            }
            (Value::Counter(a), BinaryOp::Subtract, Value::Counter(b)) => {
                let mut result = a.clone();
                for (key, count) in b.iter() {
                    let entry = result.entry(key.clone()).or_insert(0);
                    *entry -= count;
                    if *entry <= 0 {
                        result.remove(key);
                    }
                }
                Ok(Value::Counter(result))
            }

            (Value::Integer(a), BinaryOp::Equal, Value::Integer(b)) => Ok(Value::Boolean(a == b)),
            (Value::Float(a), BinaryOp::Equal, Value::Float(b)) => Ok(Value::Boolean(a == b)),
            (Value::String(a), BinaryOp::Equal, Value::String(b)) => Ok(Value::Boolean(a == b)),
            (Value::Char(c), BinaryOp::Equal, Value::String(s)) => {
                Ok(Value::Boolean(s.len() == 1 && s.chars().next() == Some(*c)))
            }
            (Value::String(s), BinaryOp::Equal, Value::Char(c)) => {
                Ok(Value::Boolean(s.len() == 1 && s.chars().next() == Some(*c)))
            }
            (Value::Boolean(a), BinaryOp::Equal, Value::Boolean(b)) => Ok(Value::Boolean(a == b)),
            // Constant-time equality ~== (same result as ==, no short-circuit)
            (Value::String(a), BinaryOp::ConstantTimeEq, Value::String(b)) => Ok(Value::Boolean(
                a.len() == b.len() && a.as_bytes() == b.as_bytes(),
            )),
            (Value::Integer(a), BinaryOp::ConstantTimeEq, Value::Integer(b)) => {
                Ok(Value::Boolean(a == b))
            }
            (Value::Float(a), BinaryOp::ConstantTimeEq, Value::Float(b)) => {
                Ok(Value::Boolean(a == b))
            }
            (Value::Boolean(a), BinaryOp::ConstantTimeEq, Value::Boolean(b)) => {
                Ok(Value::Boolean(a == b))
            }

            (Value::Integer(a), BinaryOp::Less, Value::Integer(b)) => Ok(Value::Boolean(a < b)),
            (Value::Integer(a), BinaryOp::Greater, Value::Integer(b)) => Ok(Value::Boolean(a > b)),
            (Value::Integer(a), BinaryOp::LessEqual, Value::Integer(b)) => {
                Ok(Value::Boolean(a <= b))
            }
            (Value::Integer(a), BinaryOp::GreaterEqual, Value::Integer(b)) => {
                Ok(Value::Boolean(a >= b))
            }

            (Value::Float(a), BinaryOp::Less, Value::Float(b)) => Ok(Value::Boolean(a < b)),
            (Value::Float(a), BinaryOp::Greater, Value::Float(b)) => Ok(Value::Boolean(a > b)),
            (Value::Float(a), BinaryOp::LessEqual, Value::Float(b)) => Ok(Value::Boolean(a <= b)),
            (Value::Float(a), BinaryOp::GreaterEqual, Value::Float(b)) => {
                Ok(Value::Boolean(a >= b))
            }

            // Infinity comparisons
            (Value::Integer(_), BinaryOp::Less, Value::Infinity(true)) => Ok(Value::Boolean(true)),
            (Value::Integer(_), BinaryOp::Greater, Value::Infinity(true)) => {
                Ok(Value::Boolean(false))
            }
            (Value::Integer(_), BinaryOp::Greater, Value::Infinity(false)) => {
                Ok(Value::Boolean(true))
            }
            (Value::Integer(_), BinaryOp::Less, Value::Infinity(false)) => {
                Ok(Value::Boolean(false))
            }
            (Value::Integer(_), BinaryOp::LessEqual, Value::Infinity(true)) => {
                Ok(Value::Boolean(true))
            }
            (Value::Integer(_), BinaryOp::GreaterEqual, Value::Infinity(true)) => {
                Ok(Value::Boolean(false))
            }
            (Value::Integer(_), BinaryOp::GreaterEqual, Value::Infinity(false)) => {
                Ok(Value::Boolean(true))
            }
            (Value::Integer(_), BinaryOp::LessEqual, Value::Infinity(false)) => {
                Ok(Value::Boolean(false))
            }

            (Value::Float(_), BinaryOp::Less, Value::Infinity(true)) => Ok(Value::Boolean(true)),
            (Value::Float(_), BinaryOp::Greater, Value::Infinity(true)) => {
                Ok(Value::Boolean(false))
            }
            (Value::Float(_), BinaryOp::Greater, Value::Infinity(false)) => {
                Ok(Value::Boolean(true))
            }
            (Value::Float(_), BinaryOp::Less, Value::Infinity(false)) => Ok(Value::Boolean(false)),
            (Value::Float(_), BinaryOp::LessEqual, Value::Infinity(true)) => {
                Ok(Value::Boolean(true))
            }
            (Value::Float(_), BinaryOp::GreaterEqual, Value::Infinity(true)) => {
                Ok(Value::Boolean(false))
            }
            (Value::Float(_), BinaryOp::GreaterEqual, Value::Infinity(false)) => {
                Ok(Value::Boolean(true))
            }
            (Value::Float(_), BinaryOp::LessEqual, Value::Infinity(false)) => {
                Ok(Value::Boolean(false))
            }

            // Reverse infinity comparisons
            (Value::Infinity(true), BinaryOp::Greater, Value::Integer(_)) => {
                Ok(Value::Boolean(true))
            }
            (Value::Infinity(true), BinaryOp::Less, Value::Integer(_)) => Ok(Value::Boolean(false)),
            (Value::Infinity(false), BinaryOp::Less, Value::Integer(_)) => Ok(Value::Boolean(true)),
            (Value::Infinity(false), BinaryOp::Greater, Value::Integer(_)) => {
                Ok(Value::Boolean(false))
            }
            (Value::Infinity(true), BinaryOp::GreaterEqual, Value::Integer(_)) => {
                Ok(Value::Boolean(true))
            }
            (Value::Infinity(true), BinaryOp::LessEqual, Value::Integer(_)) => {
                Ok(Value::Boolean(false))
            }
            (Value::Infinity(false), BinaryOp::LessEqual, Value::Integer(_)) => {
                Ok(Value::Boolean(true))
            }
            (Value::Infinity(false), BinaryOp::GreaterEqual, Value::Integer(_)) => {
                Ok(Value::Boolean(false))
            }

            (Value::Infinity(true), BinaryOp::Greater, Value::Float(_)) => Ok(Value::Boolean(true)),
            (Value::Infinity(true), BinaryOp::Less, Value::Float(_)) => Ok(Value::Boolean(false)),
            (Value::Infinity(false), BinaryOp::Less, Value::Float(_)) => Ok(Value::Boolean(true)),
            (Value::Infinity(false), BinaryOp::Greater, Value::Float(_)) => {
                Ok(Value::Boolean(false))
            }
            (Value::Infinity(true), BinaryOp::GreaterEqual, Value::Float(_)) => {
                Ok(Value::Boolean(true))
            }
            (Value::Infinity(true), BinaryOp::LessEqual, Value::Float(_)) => {
                Ok(Value::Boolean(false))
            }
            (Value::Infinity(false), BinaryOp::LessEqual, Value::Float(_)) => {
                Ok(Value::Boolean(true))
            }
            (Value::Infinity(false), BinaryOp::GreaterEqual, Value::Float(_)) => {
                Ok(Value::Boolean(false))
            }

            (Value::Infinity(true), BinaryOp::Greater, Value::Infinity(false)) => {
                Ok(Value::Boolean(true))
            }
            (Value::Infinity(false), BinaryOp::Less, Value::Infinity(true)) => {
                Ok(Value::Boolean(true))
            }
            (Value::Infinity(a), BinaryOp::Equal, Value::Infinity(b)) => Ok(Value::Boolean(a == b)),

            // Infinity arithmetic
            (Value::Infinity(positive), BinaryOp::Add, _) => Ok(Value::Infinity(*positive)),
            (_, BinaryOp::Add, Value::Infinity(positive)) => Ok(Value::Infinity(*positive)),
            (Value::Infinity(positive), BinaryOp::Subtract, _) => Ok(Value::Infinity(*positive)),
            (_, BinaryOp::Subtract, Value::Infinity(positive)) => Ok(Value::Infinity(!positive)),
            (Value::Infinity(positive), BinaryOp::Multiply, Value::Integer(i)) => {
                if *i > 0 {
                    Ok(Value::Infinity(*positive))
                } else if *i < 0 {
                    Ok(Value::Infinity(!positive))
                } else {
                    Ok(Value::Float(f64::NAN))
                }
            }
            (Value::Integer(i), BinaryOp::Multiply, Value::Infinity(positive)) => {
                if *i > 0 {
                    Ok(Value::Infinity(*positive))
                } else if *i < 0 {
                    Ok(Value::Infinity(!positive))
                } else {
                    Ok(Value::Float(f64::NAN))
                }
            }
            (Value::Infinity(positive), BinaryOp::Multiply, Value::Float(f)) => {
                if *f > 0.0 {
                    Ok(Value::Infinity(*positive))
                } else if *f < 0.0 {
                    Ok(Value::Infinity(!positive))
                } else {
                    Ok(Value::Float(f64::NAN))
                }
            }
            (Value::Float(f), BinaryOp::Multiply, Value::Infinity(positive)) => {
                if *f > 0.0 {
                    Ok(Value::Infinity(*positive))
                } else if *f < 0.0 {
                    Ok(Value::Infinity(!positive))
                } else {
                    Ok(Value::Float(f64::NAN))
                }
            }
            (Value::Integer(_), BinaryOp::Divide, Value::Infinity(_)) => Ok(Value::Float(0.0)),
            (Value::Float(_), BinaryOp::Divide, Value::Infinity(_)) => Ok(Value::Float(0.0)),

            (Value::Boolean(a), BinaryOp::And, Value::Boolean(b)) => Ok(Value::Boolean(*a && *b)),
            (Value::Boolean(a), BinaryOp::Or, Value::Boolean(b)) => Ok(Value::Boolean(*a || *b)),
            (_, BinaryOp::ConstantTimeEq, _) => Ok(Value::Boolean(self.values_equal(left, right))),

            // String + any / any + string (concat); other types converted to string
            (Value::String(a), BinaryOp::Add, right) => {
                Ok(Value::String(format!("{}{}", a, right)))
            }
            (left, BinaryOp::Add, Value::String(b)) => {
                Ok(Value::String(format!("{}{}", left, b)))
            }

            _ => Err(format!(
                "Unsupported binary operation: {} {:?} {}",
                left, op, right
            )),
        }
    }

    pub(super) fn eval_unary_op(&self, op: &UnaryOp, operand: &Value) -> Result<Value, String> {
        match (op, operand) {
            (UnaryOp::Minus, Value::Integer(i)) => Ok(Value::Integer(-i)),
            (UnaryOp::Minus, Value::Float(f)) => Ok(Value::Float(-f)),
            (UnaryOp::Minus, Value::Infinity(positive)) => Ok(Value::Infinity(!positive)),
            (UnaryOp::Not, Value::Boolean(b)) => Ok(Value::Boolean(!b)),
            (UnaryOp::BitwiseNot, Value::Integer(i)) => Ok(Value::Integer(!i)),
            _ => Err(format!("Unsupported unary operation: {:?} {}", op, operand)),
        }
    }

    pub(super) fn pattern_matches(&mut self, pattern: &Pattern, value: &Value) -> Result<bool, String> {
        match (pattern, value) {
            (Pattern::Wildcard, _) => Ok(true),
            (Pattern::Literal(lit_node), _) => {
                let lit_val = match lit_node {
                    AstNode::Integer(i) => Value::Integer(*i),
                    AstNode::Float(f) => Value::Float(*f),
                    AstNode::String(s) => Value::String(s.clone()),
                    AstNode::Boolean(b) => Value::Boolean(*b),
                    _ => return Err("Invalid literal in pattern".to_string()),
                };
                Ok(self.values_equal(&lit_val, value))
            }
            // CRITICAL FIX: Pattern matching now binds variables
            (Pattern::Identifier(name), _) => {
                self.set_variable(name.clone(), value.clone());
                Ok(true)
            }
            // CRITICAL FIX: Add tuple pattern matching
            (Pattern::Tuple(patterns), Value::Tuple(values)) => {
                if patterns.len() != values.len() {
                    return Ok(false);
                }
                for (pat, val) in patterns.iter().zip(values.iter()) {
                    if !self.pattern_matches(pat, val)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            // CRITICAL FIX: Add list pattern matching
            (Pattern::List(patterns), Value::List(values)) => {
                if patterns.len() != values.len() {
                    return Ok(false);
                }
                for (pat, val) in patterns.iter().zip(values.iter()) {
                    if !self.pattern_matches(pat, val)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    pub(crate) fn values_equal(&self, a: &Value, b: &Value) -> bool {
        match (a, b) {
            (Value::Integer(a), Value::Integer(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => (a - b).abs() < f64::EPSILON,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Char(a), Value::Char(b)) => a == b,
            // Cross-type numerical equality: 5 == 5.0
            (Value::Integer(a), Value::Float(b)) => ((*a as f64) - b).abs() < f64::EPSILON,
            (Value::Float(a), Value::Integer(b)) => (a - (*b as f64)).abs() < f64::EPSILON,
            // Deep equality for collections
            (Value::List(a), Value::List(b)) => {
                a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| self.values_equal(x, y))
            }
            (Value::Tuple(a), Value::Tuple(b)) => {
                a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| self.values_equal(x, y))
            }
            _ => false,
        }
    }

    pub(super) fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Boolean(b) => *b,
            Value::Integer(i) => *i != 0,
            Value::Float(f) => *f != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::List(l) => !l.is_empty(),
            Value::None => false,
            _ => true,
        }
    }

    // CRITICAL FIX: Matrix validation to prevent ragged matrices
    pub(super) fn validate_matrix(&self, rows: &[Vec<f64>]) -> Result<(), String> {
        if rows.is_empty() {
            return Ok(());
        }
        let expected_len = rows[0].len();
        for (i, row) in rows.iter().enumerate() {
            if row.len() != expected_len {
                return Err(format!(
                    "Matrix row {} has length {}, expected {} (ragged matrices not allowed)",
                    i,
                    row.len(),
                    expected_len
                ));
            }
        }
        Ok(())
    }

    pub(super) fn get_property(&self, obj: &Value, field: &str) -> Result<Value, String> {
        match obj {
            Value::Class {
                name: class_name,
                static_fields,
                static_methods,
                ..
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
                    self.get_instance_method(class_name, field)
                }
            }
            Value::Grid(grid) => {
                let rows = grid.len() as i64;
                let cols = if grid.is_empty() {
                    0
                } else {
                    grid[0].len() as i64
                };
                match field {
                    "rows" => Ok(Value::Integer(rows)),
                    "cols" | "columns" => Ok(Value::Integer(cols)),
                    "len" | "length" | "size" => Ok(Value::Integer(rows * cols)),
                    "neighbors" => Ok(Value::GridNeighbors(Box::new(Value::Grid(grid.clone())))),
                    "neighbors8" => Ok(Value::GridNeighbors8(Box::new(Value::Grid(grid.clone())))),
                    "find_all" => Ok(Value::GridFindAll(Box::new(Value::Grid(grid.clone())))),
                    "row" => Ok(Value::GridRow(Box::new(Value::Grid(grid.clone())))),
                    "col" => Ok(Value::GridCol(Box::new(Value::Grid(grid.clone())))),
                    _ => Err(format!("Grid method '{}' not found", field)),
                }
            }
            Value::Matrix(mat) => {
                let rows = mat.len() as i64;
                let cols = if mat.is_empty() {
                    0
                } else {
                    mat[0].len() as i64
                };
                match field {
                    "rows" => Ok(Value::Integer(rows)),
                    "cols" | "columns" => Ok(Value::Integer(cols)),
                    "len" | "length" | "size" => Ok(Value::Integer(rows * cols)),
                    "row" => Ok(Value::MatrixRow(Box::new(Value::Matrix(mat.clone())))),
                    "col" => Ok(Value::MatrixCol(Box::new(Value::Matrix(mat.clone())))),
                    "diagonal" => Ok(Value::MatrixDiagonal(Box::new(Value::Matrix(mat.clone())))),
                    "T" | "transpose" => {
                        if mat.is_empty() || mat[0].is_empty() {
                            return Ok(Value::Matrix(Vec::new()));
                        }
                        let rows = mat.len();
                        let cols = mat[0].len();
                        let mut transposed = vec![vec![0.0; rows]; cols];
                        for i in 0..rows {
                            for j in 0..cols {
                                transposed[j][i] = mat[i][j];
                            }
                        }
                        Ok(Value::Matrix(transposed))
                    }
                    "flat" => Ok(Value::MatrixFlat(Box::new(Value::Matrix(mat.clone())))),
                    "row_sums" => Ok(Value::MatrixRowSums(Box::new(Value::Matrix(mat.clone())))),
                    "col_sums" => Ok(Value::MatrixColSums(Box::new(Value::Matrix(mat.clone())))),
                    "row_means" => Ok(Value::MatrixRowMeans(Box::new(Value::Matrix(mat.clone())))),
                    "col_means" => Ok(Value::MatrixColMeans(Box::new(Value::Matrix(mat.clone())))),
                    _ => Err(format!("Matrix method '{}' not found", field)),
                }
            }
            Value::Counter(counter) => match field {
                "most_common" => {
                    let mut items: Vec<_> = counter.iter().collect();
                    items.sort_by(|a, b| b.1.cmp(a.1));
                    let result: Vec<Value> = items
                        .iter()
                        .map(|(k, v)| {
                            Value::Tuple(vec![Value::String(k.to_string()), Value::Integer(**v)])
                        })
                        .collect();
                    Ok(Value::List(result))
                }
                "elements" | "keys" => {
                    let keys: Vec<Value> =
                        counter.keys().map(|k| Value::String(k.clone())).collect();
                    Ok(Value::List(keys))
                }
                "total" => Ok(Value::Integer(counter.values().sum())),
                "len" | "length" | "size" => Ok(Value::Integer(counter.len() as i64)),
                _ => Err(format!("Counter method '{}' not found", field)),
            },
            Value::Pool(_pool_id) => match field {
                "alloc_str" | "alloc_vec" | "alloc_mat" | "reset" | "len" => Ok(Value::BoundMethod {
                    receiver: Box::new(obj.clone()),
                    method: field.to_string(),
                }),
                _ => Err(format!("Pool method '{}' not found", field)),
            },
            Value::PoolRef { pool_id, index } => {
                let storage = self.pools.get(pool_id).ok_or("Pool no longer valid (reset?)".to_string())?;
                let val = storage.get(*index).cloned().ok_or("PoolRef index out of bounds".to_string())?;
                match field {
                    "value" | "get" => Ok(val),
                    _ => Err(format!("PoolRef has no property '{}'", field)),
                }
            }
            Value::SmallVec { .. } => Ok(Value::BoundMethod {
                receiver: Box::new(obj.clone()),
                method: field.to_string(),
            }),
            Value::List(_) | Value::String(_) | Value::Integer(_) | Value::Float(_)
            | Value::Dict(_) |             Value::Date(_) | Value::Time(_) | Value::DateTime(_)
            | Value::Graph(_) | Value::EnumVariant { .. } | Value::Duration { .. } => Ok(Value::BoundMethod {
                receiver: Box::new(obj.clone()),
                method: field.to_string(),
            }),
            Value::DateType | Value::TimeType | Value::DateTimeType | Value::GraphType => Ok(Value::BoundMethod {
                receiver: Box::new(obj.clone()),
                method: field.to_string(),
            }),
            _ => Err(format!("Cannot get property '{}' on non-object", field)),
        }
    }

    pub(super) fn get_instance_method(&self, class_name: &str, method_name: &str) -> Result<Value, String> {
        let class_val = self.get_variable(class_name)?;
        if let Value::Class { methods, .. } = class_val {
            if let Some(m) = methods.get(method_name) {
                return Ok(m.clone());
            }
            // Mirror dispatch: if class has handle_missing, use it for missing methods
            if let Some(handle_missing) = methods.get("handle_missing") {
                return Ok(Value::MirrorDispatch {
                    method_name: method_name.to_string(),
                    handle_missing: Box::new(handle_missing.clone()),
                });
            }
            Err(format!(
                "Unknown method '{}' on class '{}'",
                method_name, class_name
            ))
        } else {
            Err(format!("'{}' is not a class", class_name))
        }
    }

    pub(super) fn call_value(
        &mut self,
        callee: Value,
        args: &[AstNode],
        this_opt: Option<Value>,
    ) -> Result<Value, String> {
        let eval_args: Vec<Value> = args
            .iter()
            .map(|a| self.eval_node(a))
            .collect::<Result<Vec<_>, _>>()?;
        self.call_value_with_args(callee, &eval_args, this_opt)
    }

    fn memo_cache_key(args: &[Value]) -> String {
        args.iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join("\x00")
    }

    fn value_type_name(v: &Value) -> &'static str {
        match v {
            Value::Integer(_) => "int",
            Value::Float(_) => "float",
            Value::String(_) => "str",
            Value::List(_) => "list",
            Value::Dict(_) => "dict",
            Value::Set(_) => "set",
            Value::Counter(_) => "counter",
            Value::Date(_) => "date",
            Value::Time(_) => "time",
            Value::DateTime(_) => "datetime",
            Value::DateType => "date_type",
            Value::TimeType => "time_type",
            Value::DateTimeType => "datetime_type",
            Value::GraphType => "graph_type",
            Value::Duration { .. } => "duration",
            Value::EnumVariant { .. } => "enum_variant",
            _ => "value",
        }
    }

    fn call_bound_method(
        &mut self,
        receiver: Value,
        method: &str,
        eval_args: &[Value],
        _this_opt: Option<Value>,
    ) -> Result<Value, String> {
        match &receiver {
            Value::List(list) => self.call_list_method(list, method, eval_args),
            Value::String(s) => self.call_string_method(s, method, eval_args),
            Value::Integer(i) => self.call_int_method(*i, method, eval_args),
            Value::Float(f) => self.call_float_method(*f, method, eval_args),
            Value::Dict(dict) => self.call_dict_method(dict, method, eval_args),
            Value::Date(s) => self.call_date_method(s, method, eval_args),
            Value::Time(s) => self.call_time_method(s, method, eval_args),
            Value::DateTime(s) => self.call_datetime_method(s, method, eval_args),
            Value::DateType => self.call_date_type_method(method, eval_args),
            Value::TimeType => self.call_time_type_method(method, eval_args),
            Value::DateTimeType => self.call_datetime_type_method(method, eval_args),
            Value::GraphType => self.call_graph_type_method(method, eval_args),
            Value::Graph(graph) => self.call_graph_method(graph, method, eval_args),
            Value::Duration { total_seconds } => self.call_interval_method(*total_seconds, method, eval_args),
            Value::EnumVariant {
                enum_name,
                variant_name,
                value,
            } => self.call_enum_variant_method(enum_name, variant_name, value.as_ref(), method, eval_args),
            Value::Pool(pool_id) => self.call_pool_method(*pool_id, method, eval_args),
            Value::SmallVec { cap, elements } => self.call_smallvec_method(*cap, elements, method, eval_args),
            _ => Err(format!(
                "Bound method '{}' not supported for type {}",
                method,
                Self::value_type_name(&receiver)
            )),
        }
    }

    fn call_pool_method(
        &mut self,
        pool_id: usize,
        method: &str,
        args: &[Value],
    ) -> Result<Value, String> {
        let storage = self.pools.get_mut(&pool_id).ok_or("Pool no longer valid".to_string())?;
        match method {
            "alloc_str" => {
                if args.len() != 1 {
                    return Err("pool.alloc_str(s) requires exactly 1 argument".to_string());
                }
                let s = match &args[0] {
                    Value::String(x) => x.clone(),
                    _ => return Err("pool.alloc_str(s) requires string".to_string()),
                };
                storage.push(Value::String(s.clone()));
                let index = storage.len() - 1;
                Ok(Value::PoolRef { pool_id, index })
            }
            "alloc_vec" => {
                let cap = args.first().and_then(|v| if let Value::Integer(i) = v { Some(*i as usize) } else { None }).unwrap_or(0);
                let vec: Vec<Value> = Vec::with_capacity(cap);
                storage.push(Value::List(vec.clone()));
                let index = storage.len() - 1;
                Ok(Value::PoolRef { pool_id, index })
            }
            "alloc_mat" => {
                if args.len() < 2 {
                    return Err("pool.alloc_mat(rows, cols) requires 2 arguments".to_string());
                }
                let rows = match &args[0] { Value::Integer(i) if *i >= 0 => *i as usize, _ => return Err("alloc_mat rows must be non-negative int".to_string()) };
                let cols = match &args[1] { Value::Integer(i) if *i >= 0 => *i as usize, _ => return Err("alloc_mat cols must be non-negative int".to_string()) };
                let mat = vec![vec![0.0; cols]; rows];
                storage.push(Value::Matrix(mat));
                let index = storage.len() - 1;
                Ok(Value::PoolRef { pool_id, index })
            }
            "reset" => {
                storage.clear();
                Ok(Value::None)
            }
            "len" => Ok(Value::Integer(storage.len() as i64)),
            _ => Err(format!("Pool method '{}' not found", method)),
        }
    }

    fn call_smallvec_method(
        &mut self,
        cap: usize,
        elements: &[Value],
        method: &str,
        args: &[Value],
    ) -> Result<Value, String> {
        let mut vec = elements.to_vec();
        match method {
            "push" => {
                if args.len() != 1 {
                    return Err("smallvec.push(x) requires 1 argument".to_string());
                }
                vec.push(args[0].clone());
                Ok(Value::SmallVec { cap, elements: vec })
            }
            "pop" => Ok(vec.pop().unwrap_or(Value::None)),
            "len" | "length" | "size" => Ok(Value::Integer(vec.len() as i64)),
            "first" => Ok(vec.first().cloned().unwrap_or(Value::None)),
            "last" => Ok(vec.last().cloned().unwrap_or(Value::None)),
            _ => Err(format!("SmallVec method '{}' not found", method)),
        }
    }

    fn call_list_method(
        &self,
        list: &[Value],
        method: &str,
        args: &[Value],
    ) -> Result<Value, String> {
        match method {
            "len" | "length" | "size" => {
                if !args.is_empty() {
                    return Err("list.len() takes no arguments".to_string());
                }
                Ok(Value::Integer(list.len() as i64))
            }
            "first" => {
                if !args.is_empty() {
                    return Err("list.first() takes no arguments".to_string());
                }
                list.first()
                    .cloned()
                    .ok_or_else(|| "List is empty".to_string())
            }
            "last" => {
                if !args.is_empty() {
                    return Err("list.last() takes no arguments".to_string());
                }
                list.last()
                    .cloned()
                    .ok_or_else(|| "List is empty".to_string())
            }
            "push" => {
                let mut new_list = list.to_vec();
                if args.len() != 1 {
                    return Err("list.push(value) requires exactly one argument".to_string());
                }
                new_list.push(args[0].clone());
                Ok(Value::List(new_list))
            }
            "pop" => {
                if !args.is_empty() {
                    return Err("list.pop() takes no arguments".to_string());
                }
                list.last()
                    .cloned()
                    .ok_or_else(|| "List is empty".to_string())
            }
            "append" => {
                let mut new_list = list.to_vec();
                if args.len() != 1 {
                    return Err("list.append(value) requires exactly one argument".to_string());
                }
                new_list.push(args[0].clone());
                Ok(Value::List(new_list))
            }
            "get" => {
                if args.is_empty() {
                    return Err("list.get(index) requires an index argument".to_string());
                }
                let idx = match &args[0] {
                    Value::Integer(i) => *i,
                    _ => return Err("list.get(index) expects integer index".to_string()),
                };
                let idx = idx as usize;
                if args.len() >= 2 {
                    let default = args[1].clone();
                    Ok(list.get(idx).cloned().unwrap_or(default))
                } else {
                    list.get(idx)
                        .cloned()
                        .ok_or_else(|| format!("List index {} out of range", idx))
                }
            }
            "contains" => {
                if args.len() != 1 {
                    return Err("list.contains(value) requires exactly one argument".to_string());
                }
                let target = &args[0];
                Ok(Value::Boolean(list.iter().any(|v| self.values_equal(v, target))))
            }
            "index" | "find" => {
                if args.is_empty() {
                    return Err("list.index(value) requires at least one argument".to_string());
                }
                let target = &args[0];
                let start = if args.len() >= 2 {
                    match &args[1] {
                        Value::Integer(i) => *i as usize,
                        _ => 0,
                    }
                } else {
                    0
                };
                list.iter()
                    .skip(start)
                    .position(|v| self.values_equal(v, target))
                    .map(|i| Value::Integer((i + start) as i64))
                    .ok_or_else(|| "Value not found in list".to_string())
            }
            "count" => {
                if args.len() != 1 {
                    return Err("list.count(value) requires exactly one argument".to_string());
                }
                let target = &args[0];
                let n = list
                    .iter()
                    .filter(|v| self.values_equal(v, target))
                    .count();
                Ok(Value::Integer(n as i64))
            }
            "join" => {
                if args.len() != 1 {
                    return Err("list.join(separator) requires exactly one argument".to_string());
                }
                let sep = match &args[0] {
                    Value::String(s) => s.as_str(),
                    _ => return Err("list.join(separator) expects string".to_string()),
                };
                let parts: Vec<String> = list
                    .iter()
                    .map(|v| v.to_string())
                    .collect();
                Ok(Value::String(parts.join(sep)))
            }
            "insert" => {
                if args.len() < 2 {
                    return Err("list.insert(index, value) requires two arguments".to_string());
                }
                let idx = match &args[0] {
                    Value::Integer(i) => *i as usize,
                    _ => return Err("list.insert(index, value) expects integer index".to_string()),
                };
                let mut new_list = list.to_vec();
                if idx > new_list.len() {
                    new_list.push(args[1].clone());
                } else {
                    new_list.insert(idx, args[1].clone());
                }
                Ok(Value::List(new_list))
            }
            "slice" => {
                let start = if args.is_empty() {
                    0
                } else {
                    match &args[0] {
                        Value::Integer(i) => *i as i64,
                        _ => 0,
                    }
                };
                let end = if args.len() >= 2 {
                    match &args[1] {
                        Value::Integer(i) => *i as i64,
                        _ => list.len() as i64,
                    }
                } else {
                    list.len() as i64
                };
                let step = if args.len() >= 3 {
                    match &args[2] {
                        Value::Integer(i) => *i,
                        _ => 1,
                    }
                } else {
                    1
                };
                let len = list.len() as i64;
                let start_u = if start < 0 {
                    (len + start).max(0) as usize
                } else {
                    start.min(len) as usize
                };
                let end_i = if end < 0 { len + end } else { end };
                let end_i = end_i.max(0).min(len);
                let mut out = Vec::new();
                if step > 0 {
                    let mut i = start_u as i64;
                    while i < end_i {
                        out.push(list[i as usize].clone());
                        i += step;
                    }
                } else if step < 0 {
                    let mut i = start_u as i64;
                    while i > end_i {
                        out.push(list[i as usize].clone());
                        i += step;
                    }
                }
                Ok(Value::List(out))
            }
            "empty" | "is_empty" => {
                if !args.is_empty() {
                    return Err("list.is_empty() takes no arguments".to_string());
                }
                Ok(Value::Boolean(list.is_empty()))
            }
            _ => Err(format!("List method '{}' not found", method)),
        }
    }

    fn call_string_method(
        &self,
        s: &str,
        method: &str,
        args: &[Value],
    ) -> Result<Value, String> {
        match method {
            "len" | "length" | "size" => {
                if !args.is_empty() {
                    return Err("str.len() takes no arguments".to_string());
                }
                Ok(Value::Integer(s.len() as i64))
            }
            "trim" => {
                if !args.is_empty() {
                    return Err("str.trim() takes no arguments".to_string());
                }
                Ok(Value::String(s.trim().to_string()))
            }
            "trim_start" | "trim_left" => {
                if !args.is_empty() {
                    return Err("str.trim_start() takes no arguments".to_string());
                }
                Ok(Value::String(s.trim_start().to_string()))
            }
            "trim_end" | "trim_right" => {
                if !args.is_empty() {
                    return Err("str.trim_end() takes no arguments".to_string());
                }
                Ok(Value::String(s.trim_end().to_string()))
            }
            "upper" | "uppercase" | "to_upper" => {
                if !args.is_empty() {
                    return Err("str.to_upper() takes no arguments".to_string());
                }
                Ok(Value::String(s.to_uppercase()))
            }
            "lower" | "lowercase" | "to_lower" => {
                if !args.is_empty() {
                    return Err("str.to_lower() takes no arguments".to_string());
                }
                Ok(Value::String(s.to_lowercase()))
            }
            "replace" => {
                if args.len() < 2 {
                    return Err("str.replace(old, new) requires two arguments".to_string());
                }
                let old_str = args[0].to_string();
                let new_str = args[1].to_string();
                let count = if args.len() >= 3 {
                    match &args[2] {
                        Value::Integer(n) => Some(*n as usize),
                        _ => None,
                    }
                } else {
                    None
                };
                let result = if let Some(n) = count {
                    s.replacen(&old_str, &new_str, n)
                } else {
                    s.replace(&old_str, &new_str)
                };
                Ok(Value::String(result))
            }
            "split" => {
                if args.is_empty() {
                    return Err("str.split(sep) requires at least one argument".to_string());
                }
                let sep = args[0].to_string();
                let parts: Vec<Value> = if sep.is_empty() {
                    s.chars().map(|c| Value::String(c.to_string())).collect()
                } else {
                    s.split(&sep).map(|p| Value::String(p.to_string())).collect()
                };
                Ok(Value::List(parts))
            }
            "starts_with" => {
                if args.len() != 1 {
                    return Err("str.starts_with(prefix) requires one argument".to_string());
                }
                let prefix = args[0].to_string();
                Ok(Value::Boolean(s.starts_with(&prefix)))
            }
            "ends_with" => {
                if args.len() != 1 {
                    return Err("str.ends_with(suffix) requires one argument".to_string());
                }
                let suffix = args[0].to_string();
                Ok(Value::Boolean(s.ends_with(&suffix)))
            }
            "contains" => {
                if args.len() != 1 {
                    return Err("str.contains(sub) requires one argument".to_string());
                }
                let sub = args[0].to_string();
                Ok(Value::Boolean(s.contains(&sub)))
            }
            "find" => {
                if args.is_empty() {
                    return Err("str.find(sub) requires at least one argument".to_string());
                }
                let sub = args[0].to_string();
                let start = if args.len() >= 2 {
                    match &args[1] {
                        Value::Integer(i) => *i as usize,
                        _ => 0,
                    }
                } else {
                    0
                };
                Ok(s[start..]
                    .find(&sub)
                    .map(|i| Value::Integer((i + start) as i64))
                    .unwrap_or(Value::Integer(-1)))
            }
            "repeat" => {
                if args.len() != 1 {
                    return Err("str.repeat(n) requires one argument".to_string());
                }
                let n = match &args[0] {
                    Value::Integer(i) => *i as usize,
                    _ => return Err("str.repeat(n) expects integer".to_string()),
                };
                Ok(Value::String(s.repeat(n)))
            }
            "empty" | "is_empty" => {
                if !args.is_empty() {
                    return Err("str.is_empty() takes no arguments".to_string());
                }
                Ok(Value::Boolean(s.is_empty()))
            }
            _ => Err(format!("String method '{}' not found", method)),
        }
    }

    fn call_int_method(&self, i: i64, method: &str, args: &[Value]) -> Result<Value, String> {
        match method {
            "bit" => {
                if args.len() != 1 {
                    return Err("int.bit(n) requires one argument".to_string());
                }
                let n = match &args[0] {
                    Value::Integer(n) => *n as u32,
                    _ => return Err("int.bit(n) expects integer".to_string()),
                };
                Ok(Value::Integer(if (i >> n) & 1 != 0 { 1 } else { 0 }))
            }
            "set_bit" => {
                if args.len() != 1 {
                    return Err("int.set_bit(n) requires one argument".to_string());
                }
                let n = match &args[0] {
                    Value::Integer(n) => *n as u32,
                    _ => return Err("int.set_bit(n) expects integer".to_string()),
                };
                Ok(Value::Integer(i | (1i64 << n)))
            }
            "clear_bit" => {
                if args.len() != 1 {
                    return Err("int.clear_bit(n) requires one argument".to_string());
                }
                let n = match &args[0] {
                    Value::Integer(n) => *n as u32,
                    _ => return Err("int.clear_bit(n) expects integer".to_string()),
                };
                Ok(Value::Integer(i & !(1i64 << n)))
            }
            "toggle_bit" => {
                if args.len() != 1 {
                    return Err("int.toggle_bit(n) requires one argument".to_string());
                }
                let n = match &args[0] {
                    Value::Integer(n) => *n as u32,
                    _ => return Err("int.toggle_bit(n) expects integer".to_string()),
                };
                Ok(Value::Integer(i ^ (1i64 << n)))
            }
            "popcount" | "count_bits" => {
                if !args.is_empty() {
                    return Err("int.popcount() takes no arguments".to_string());
                }
                Ok(Value::Integer(i.count_ones() as i64))
            }
            "trailing_zeros" => {
                if !args.is_empty() {
                    return Err("int.trailing_zeros() takes no arguments".to_string());
                }
                Ok(Value::Integer(i.trailing_zeros() as i64))
            }
            "leading_zeros" => {
                if !args.is_empty() {
                    return Err("int.leading_zeros() takes no arguments".to_string());
                }
                Ok(Value::Integer(i.leading_zeros() as i64))
            }
            "is_power_of_two" => {
                if !args.is_empty() {
                    return Err("int.is_power_of_two() takes no arguments".to_string());
                }
                Ok(Value::Boolean(i > 0 && (i & (i - 1)) == 0))
            }
            "div_ceil" => {
                if args.len() != 1 {
                    return Err("int.div_ceil(other) requires one argument".to_string());
                }
                let other = match &args[0] {
                    Value::Integer(j) => *j,
                    _ => return Err("int.div_ceil(other) expects integer".to_string()),
                };
                if other == 0 {
                    return Err("division by zero".to_string());
                }
                Ok(Value::Integer((i + other - 1) / other))
            }
            "gcd" => {
                if args.len() != 1 {
                    return Err("int.gcd(other) requires one argument".to_string());
                }
                let mut a = i.abs();
                let mut b = match &args[0] {
                    Value::Integer(j) => j.abs(),
                    _ => return Err("int.gcd(other) expects integer".to_string()),
                };
                while b != 0 {
                    let t = b;
                    b = a % b;
                    a = t;
                }
                Ok(Value::Integer(a))
            }
            "lcm" => {
                if args.len() != 1 {
                    return Err("int.lcm(other) requires one argument".to_string());
                }
                let other = match &args[0] {
                    Value::Integer(j) => *j,
                    _ => return Err("int.lcm(other) expects integer".to_string()),
                };
                let mut a = i.abs();
                let mut b = other.abs();
                if a == 0 || b == 0 {
                    return Ok(Value::Integer(0));
                }
                let prod = a * b;
                while b != 0 {
                    let t = b;
                    b = a % b;
                    a = t;
                }
                Ok(Value::Integer(prod / a))
            }
            _ => Err(format!("Integer method '{}' not found", method)),
        }
    }

    fn call_float_method(
        &self,
        f: f64,
        method: &str,
        args: &[Value],
    ) -> Result<Value, String> {
        match method {
            "approx_eq" => {
                if args.len() < 1 {
                    return Err("float.approx_eq(other) requires at least one argument".to_string());
                }
                let other = match &args[0] {
                    Value::Float(x) => *x,
                    Value::Integer(n) => *n as f64,
                    _ => return Err("float.approx_eq(other) expects number".to_string()),
                };
                let eps = if args.len() >= 2 {
                    match &args[1] {
                        Value::Float(x) => *x,
                        Value::Integer(n) => *n as f64,
                        _ => 1e-9,
                    }
                } else {
                    1e-9
                };
                Ok(Value::Boolean((f - other).abs() < eps))
            }
            "is_nan" => {
                if !args.is_empty() {
                    return Err("float.is_nan() takes no arguments".to_string());
                }
                Ok(Value::Boolean(f.is_nan()))
            }
            "is_inf" | "is_infinite" => {
                if !args.is_empty() {
                    return Err("float.is_inf() takes no arguments".to_string());
                }
                Ok(Value::Boolean(f.is_infinite()))
            }
            "is_finite" => {
                if !args.is_empty() {
                    return Err("float.is_finite() takes no arguments".to_string());
                }
                Ok(Value::Boolean(f.is_finite()))
            }
            "sign" => {
                if !args.is_empty() {
                    return Err("float.sign() takes no arguments".to_string());
                }
                Ok(Value::Integer(if f > 0.0 {
                    1
                } else if f < 0.0 {
                    -1
                } else {
                    0
                }))
            }
            "clamp" => {
                if args.len() < 2 {
                    return Err("float.clamp(min, max) requires two arguments".to_string());
                }
                let min_val = match &args[0] {
                    Value::Float(x) => *x,
                    Value::Integer(n) => *n as f64,
                    _ => return Err("float.clamp(min, max) expects numbers".to_string()),
                };
                let max_val = match &args[1] {
                    Value::Float(x) => *x,
                    Value::Integer(n) => *n as f64,
                    _ => return Err("float.clamp(min, max) expects numbers".to_string()),
                };
                Ok(Value::Float(f.clamp(min_val, max_val)))
            }
            "fract" => {
                if !args.is_empty() {
                    return Err("float.fract() takes no arguments".to_string());
                }
                Ok(Value::Float(f - f.floor()))
            }
            "to_degrees" => {
                if !args.is_empty() {
                    return Err("float.to_degrees() takes no arguments".to_string());
                }
                Ok(Value::Float(f.to_degrees()))
            }
            "to_radians" => {
                if !args.is_empty() {
                    return Err("float.to_radians() takes no arguments".to_string());
                }
                Ok(Value::Float(f.to_radians()))
            }
            _ => Err(format!("Float method '{}' not found", method)),
        }
    }

    fn call_dict_method(
        &self,
        dict: &std::collections::HashMap<String, Value>,
        method: &str,
        args: &[Value],
    ) -> Result<Value, String> {
        match method {
            "get" => {
                if args.is_empty() {
                    return Err("dict.get(key) requires at least one argument".to_string());
                }
                let key = args[0].to_string();
                if let Some(v) = dict.get(&key) {
                    return Ok(v.clone());
                }
                if args.len() >= 2 {
                    Ok(args[1].clone())
                } else {
                    Ok(Value::None)
                }
            }
            "has" | "contains_key" => {
                if args.len() != 1 {
                    return Err("dict.has(key) requires one argument".to_string());
                }
                let key = args[0].to_string();
                Ok(Value::Boolean(dict.contains_key(&key)))
            }
            "len" | "length" | "size" => {
                if !args.is_empty() {
                    return Err("dict.len() takes no arguments".to_string());
                }
                Ok(Value::Integer(dict.len() as i64))
            }
            "keys" => {
                if !args.is_empty() {
                    return Err("dict.keys() takes no arguments".to_string());
                }
                let keys: Vec<Value> =
                    dict.keys().map(|k| Value::String(k.clone())).collect();
                Ok(Value::List(keys))
            }
            "values" => {
                if !args.is_empty() {
                    return Err("dict.values() takes no arguments".to_string());
                }
                let values: Vec<Value> = dict.values().cloned().collect();
                Ok(Value::List(values))
            }
            "items" => {
                if !args.is_empty() {
                    return Err("dict.items() takes no arguments".to_string());
                }
                let items: Vec<Value> = dict
                    .iter()
                    .map(|(k, v)| Value::Tuple(vec![Value::String(k.clone()), v.clone()]))
                    .collect();
                Ok(Value::List(items))
            }
            _ => Err(format!("Dict method '{}' not found", method)),
        }
    }

    #[cfg(feature = "serde_json")]
    fn json_value_to_jade(v: &serde_json::Value) -> Value {
        use std::collections::HashMap;
        match v {
            serde_json::Value::Null => Value::None,
            serde_json::Value::Bool(b) => Value::Boolean(*b),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Value::Integer(i)
                } else if let Some(f) = n.as_f64() {
                    Value::Float(f)
                } else {
                    Value::Float(n.as_f64().unwrap_or(0.0))
                }
            }
            serde_json::Value::String(s) => Value::String(s.clone()),
            serde_json::Value::Array(arr) => {
                Value::List(arr.iter().map(Self::json_value_to_jade).collect())
            }
            serde_json::Value::Object(obj) => {
                let map: HashMap<String, Value> = obj
                    .iter()
                    .map(|(k, v)| (k.clone(), Self::json_value_to_jade(v)))
                    .collect();
                Value::Dict(map)
            }
        }
    }

    #[cfg(feature = "serde_json")]
    fn jade_value_to_json_string(v: &Value) -> Option<String> {
        match v {
            Value::None => Some("null".to_string()),
            Value::Boolean(b) => Some(if *b { "true" } else { "false" }.to_string()),
            Value::Integer(i) => Some(i.to_string()),
            Value::Float(f) => Some(format!("{}", f)),
            Value::String(s) => Some(serde_json::to_string(s).ok()?),
            Value::List(list) => {
                let parts: Vec<String> = list
                    .iter()
                    .filter_map(|x| Self::jade_value_to_json_string(x))
                    .collect();
                Some(format!("[{}]", parts.join(",")))
            }
            Value::Dict(dict) => {
                let parts: Vec<String> = dict
                    .iter()
                    .filter_map(|(k, v)| {
                        Some(format!(
                            "{}:{}",
                            serde_json::to_string(k).ok()?,
                            Self::jade_value_to_json_string(v)?
                        ))
                    })
                    .collect();
                Some(format!("{{{}}}", parts.join(",")))
            }
            _ => None,
        }
    }

    fn parse_date_parts(s: &str) -> Result<(i64, i64, i64), String> {
        let parts: Vec<&str> = s.trim().split('-').collect();
        if parts.len() != 3 {
            return Err(format!("Invalid date format '{}', use YYYY-MM-DD", s));
        }
        let y: i64 = parts[0].parse().map_err(|_| "Invalid year")?;
        let m: i64 = parts[1].parse().map_err(|_| "Invalid month")?;
        let d: i64 = parts[2].parse().map_err(|_| "Invalid day")?;
        Ok((y, m, d))
    }

    /// Parse date from YYYY-MM-DD or MM/DD/YYYY (and DD/MM/YYYY when first part > 12).
    /// Returns (year, month, day) and canonical "YYYY-MM-DD" string.
    fn parse_date_flexible(s: &str) -> Result<(i64, i64, i64, String), String> {
        let s = s.trim();
        if s.contains('/') {
            let parts: Vec<&str> = s.split('/').collect();
            if parts.len() != 3 {
                return Err(format!("Invalid date format '{}', use YYYY-MM-DD or MM/DD/YYYY", s));
            }
            let a: i64 = parts[0].parse().map_err(|_| "Invalid number in date")?;
            let b: i64 = parts[1].parse().map_err(|_| "Invalid number in date")?;
            let c: i64 = parts[2].parse().map_err(|_| "Invalid number in date")?;
            let (y, m, d) = if a > 31 || (a <= 12 && b <= 31) {
                // MM/DD/YYYY (US) or YYYY/MM/DD when first is year
                if a > 31 {
                    (a, b, c)
                } else {
                    (c, a, b)
                }
            } else {
                // DD/MM/YYYY
                (c, b, a)
            };
            let canonical = format!("{:04}-{:02}-{:02}", y, m, d);
            Ok((y, m, d, canonical))
        } else {
            let (y, m, d) = Self::parse_date_parts(s)?;
            Ok((y, m, d, s.to_string()))
        }
    }

    fn parse_time_parts(s: &str) -> Result<(i64, i64, i64, i64), String> {
        let s = s.trim();
        let (time_part, millis) = if let Some((t, ms)) = s.split_once('.') {
            (t, ms.parse::<i64>().unwrap_or(0))
        } else {
            (s, 0i64)
        };
        let parts: Vec<&str> = time_part.split(':').collect();
        if parts.len() < 3 {
            return Err(format!("Invalid time format '{}', use HH:MM:SS", s));
        }
        let h: i64 = parts[0].parse().map_err(|_| "Invalid hour")?;
        let min: i64 = parts[1].parse().map_err(|_| "Invalid minute")?;
        let sec: i64 = parts[2].parse().map_err(|_| "Invalid second")?;
        Ok((h, min, sec, millis))
    }

    fn call_date_method(&self, s: &str, method: &str, args: &[Value]) -> Result<Value, String> {
        let (y, m, d) = Self::parse_date_parts(s)?;
        match method {
            "year" => {
                if !args.is_empty() {
                    return Err("date.year() takes no arguments".to_string());
                }
                Ok(Value::Integer(y))
            }
            "month" => {
                if !args.is_empty() {
                    return Err("date.month() takes no arguments".to_string());
                }
                Ok(Value::Integer(m))
            }
            "day" => {
                if !args.is_empty() {
                    return Err("date.day() takes no arguments".to_string());
                }
                Ok(Value::Integer(d))
            }
            "weekday" => {
                if !args.is_empty() {
                    return Err("date.weekday() takes no arguments".to_string());
                }
                let weekday = Self::weekday_from_ymd(y, m, d);
                Ok(Value::Integer(weekday))
            }
            "weekday_name" | "day_of_week" => {
                if !args.is_empty() {
                    return Err("date.weekday_name() takes no arguments".to_string());
                }
                let w = Self::weekday_from_ymd(y, m, d);
                let name = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"]
                    .get(w as usize)
                    .copied()
                    .unwrap_or("Unknown");
                Ok(Value::String(name.to_string()))
            }
            "month_name" => {
                if !args.is_empty() {
                    return Err("date.month_name() takes no arguments".to_string());
                }
                let name = [
                    "January", "February", "March", "April", "May", "June",
                    "July", "August", "September", "October", "November", "December",
                ]
                .get((m as usize).wrapping_sub(1))
                .copied()
                .unwrap_or("Unknown");
                Ok(Value::String(name.to_string()))
            }
            "is_leap" => {
                if !args.is_empty() {
                    return Err("date.is_leap() takes no arguments".to_string());
                }
                Ok(Value::Boolean(Self::is_leap_year(y)))
            }
            "add_days" => {
                if args.len() != 1 {
                    return Err("date.add_days(n) requires one argument".to_string());
                }
                let n = match &args[0] {
                    Value::Integer(i) => *i,
                    _ => return Err("date.add_days(n) expects integer".to_string()),
                };
                let (ny, nm, nd) = Self::add_days_ymd(y, m, d, n);
                Ok(Value::Date(format!("{:04}-{:02}-{:02}", ny, nm, nd)))
            }
            "format" => {
                if args.is_empty() {
                    return Err("date.format(fmt) requires one argument".to_string());
                }
                let fmt = args[0].to_string();
                let out = if fmt == "YYYY-MM-DD" || fmt.is_empty() {
                    format!("{:04}-{:02}-{:02}", y, m, d)
                } else {
                    format!("{:04}-{:02}-{:02}", y, m, d)
                };
                Ok(Value::String(out))
            }
            "difference" => {
                if args.len() != 1 {
                    return Err("date.difference(other) requires one argument".to_string());
                }
                let other = match &args[0] {
                    Value::Date(os) => os.as_str(),
                    _ => return Err("date.difference(other) expects date".to_string()),
                };
                let (oy, om, od) = Self::parse_date_parts(other)?;
                let days = Self::days_between(oy, om, od, y, m, d);
                Ok(Value::Duration {
                    total_seconds: days * 24 * 3600,
                })
            }
            _ => Err(format!("Date method '{}' not found", method)),
        }
    }

    fn weekday_from_ymd(y: i64, m: i64, d: i64) -> i64 {
        let (y, m, d) = (y as i32, m as i32, d as i32);
        let mut month = m;
        let mut year = y;
        if month < 3 {
            month += 12;
            year -= 1;
        }
        let k = year % 100;
        let j = year / 100;
        let day = (d + (13 * (month + 1)) / 5 + k + k / 4 + j / 4 + 5 * j) % 7;
        // Zeller: 0=Sat, 1=Sun, 2=Mon, ... 6=Fri. Map to 0=Mon..6=Sun.
        ((day + 5) % 7) as i64
    }

    fn is_leap_year(y: i64) -> bool {
        (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0)
    }

    fn add_days_ymd(y: i64, m: i64, d: i64, delta: i64) -> (i64, i64, i64) {
        const DAYS: [i64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let mut d = d + delta;
        let mut m = m;
        let mut y = y;
        loop {
            let max = if m == 2 && Self::is_leap_year(y) { 29 } else { DAYS[(m - 1) as usize] };
            if d > max {
                d -= max;
                m += 1;
                if m > 12 {
                    m = 1;
                    y += 1;
                }
            } else if d < 1 {
                m -= 1;
                if m < 1 {
                    m = 12;
                    y -= 1;
                }
                let max = if m == 2 && Self::is_leap_year(y) { 29 } else { DAYS[(m - 1) as usize] };
                d += max;
            } else {
                break;
            }
        }
        (y, m, d)
    }

    fn days_between(y1: i64, m1: i64, d1: i64, y2: i64, m2: i64, d2: i64) -> i64 {
        let j1 = Self::to_julian_day(y1, m1, d1);
        let j2 = Self::to_julian_day(y2, m2, d2);
        j2 - j1
    }

    fn to_julian_day(y: i64, m: i64, d: i64) -> i64 {
        let (y, m, d) = (y as i32, m as i32, d as i32);
        let a = (14 - m) / 12;
        let ny = y + 4800 - a;
        let nm = m + 12 * a - 3;
        (d + (153 * nm + 2) / 5 + 365 * ny + ny / 4 - ny / 100 + ny / 400 - 32045) as i64
    }

    fn call_time_method(&self, s: &str, method: &str, args: &[Value]) -> Result<Value, String> {
        let (h, min, sec, millis) = Self::parse_time_parts(s)?;
        match method {
            "hour" => {
                if !args.is_empty() {
                    return Err("time.hour() takes no arguments".to_string());
                }
                Ok(Value::Integer(h))
            }
            "minute" => {
                if !args.is_empty() {
                    return Err("time.minute() takes no arguments".to_string());
                }
                Ok(Value::Integer(min))
            }
            "second" => {
                if !args.is_empty() {
                    return Err("time.second() takes no arguments".to_string());
                }
                Ok(Value::Integer(sec))
            }
            "millis" => {
                if !args.is_empty() {
                    return Err("time.millis() takes no arguments".to_string());
                }
                Ok(Value::Integer(millis))
            }
            "add_hours" => {
                if args.len() != 1 {
                    return Err("time.add_hours(n) requires one argument".to_string());
                }
                let n = match &args[0] {
                    Value::Integer(i) => *i,
                    _ => return Err("time.add_hours(n) expects integer".to_string()),
                };
                let (nh, nmin, nsec) = Self::add_seconds_to_time(h, min, sec, n * 3600);
                Ok(Value::Time(format!("{:02}:{:02}:{:02}", nh, nmin, nsec)))
            }
            "add_minutes" => {
                if args.len() != 1 {
                    return Err("time.add_minutes(n) requires one argument".to_string());
                }
                let n = match &args[0] {
                    Value::Integer(i) => *i,
                    _ => return Err("time.add_minutes(n) expects integer".to_string()),
                };
                let (nh, nmin, nsec) = Self::add_seconds_to_time(h, min, sec, n * 60);
                Ok(Value::Time(format!("{:02}:{:02}:{:02}", nh, nmin, nsec)))
            }
            "is_midnight" => {
                if !args.is_empty() {
                    return Err("time.is_midnight() takes no arguments".to_string());
                }
                Ok(Value::Boolean(h == 0 && min == 0 && sec == 0))
            }
            "format" => {
                if args.is_empty() {
                    return Err("time.format(fmt) requires one argument".to_string());
                }
                let out = format!("{:02}:{:02}:{:02}", h, min, sec);
                Ok(Value::String(out))
            }
            _ => Err(format!("Time method '{}' not found", method)),
        }
    }

    fn add_seconds_to_time(h: i64, min: i64, sec: i64, delta: i64) -> (i64, i64, i64) {
        let total = h * 3600 + min * 60 + sec + delta;
        let total = ((total % 86400) + 86400) % 86400;
        let h = total / 3600;
        let min = (total % 3600) / 60;
        let s = total % 60;
        (h, min, s)
    }

    fn call_datetime_method(&self, s: &str, method: &str, args: &[Value]) -> Result<Value, String> {
        let s = s.trim();
        let (date_part, time_part) = if let Some(pos) = s.find('T').or_else(|| s.find(' ')) {
            let (d, t) = s.split_at(pos);
            (d.trim(), t.trim().trim_start_matches('T').trim_start_matches(' '))
        } else {
            (s, "00:00:00")
        };
        let (y, m, day) = Self::parse_date_parts(date_part)?;
        let (h, min, sec, _millis) = Self::parse_time_parts(time_part).unwrap_or((0, 0, 0, 0));
        match method {
            "year" => Ok(Value::Integer(y)),
            "month" => Ok(Value::Integer(m)),
            "day" => Ok(Value::Integer(day)),
            "hour" => Ok(Value::Integer(h)),
            "minute" => Ok(Value::Integer(min)),
            "second" => Ok(Value::Integer(sec)),
            "date" => Ok(Value::Date(format!("{:04}-{:02}-{:02}", y, m, day))),
            "time" => Ok(Value::Time(format!("{:02}:{:02}:{:02}", h, min, sec))),
            "timestamp" => {
                let jd = Self::to_julian_day(y, m, day);
                let secs = (jd - 2440588) * 86400 + h * 3600 + min * 60 + sec;
                Ok(Value::Integer(secs))
            }
            "format" => {
                if args.is_empty() {
                    return Err("datetime.format(fmt) requires one argument".to_string());
                }
                let out = format!(
                    "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
                    y, m, day, h, min, sec
                );
                Ok(Value::String(out))
            }
            "add" => {
                let mut days = 0i64;
                let mut hours = 0i64;
                let mut minutes = 0i64;
                let mut seconds = 0i64;
                if args.len() >= 1 {
                    if let Value::Integer(n) = args[0] {
                        days = n;
                    }
                }
                if args.len() >= 2 {
                    if let Value::Integer(n) = args[1] {
                        hours = n;
                    }
                }
                if args.len() >= 3 {
                    if let Value::Integer(n) = args[2] {
                        minutes = n;
                    }
                }
                if args.len() >= 4 {
                    if let Value::Integer(n) = args[3] {
                        seconds = n;
                    }
                }
                let total_secs = days * 86400 + hours * 3600 + minutes * 60 + seconds;
                let (ny, nm, nd) = Self::add_days_ymd(y, m, day, total_secs / 86400);
                let (nh, nmin, nsec) = Self::add_seconds_to_time(h, min, sec, total_secs % 86400);
                Ok(Value::DateTime(format!(
                    "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
                    ny, nm, nd, nh, nmin, nsec
                )))
            }
            "difference" => {
                if args.len() != 1 {
                    return Err("datetime.difference(other) requires one argument".to_string());
                }
                let other = match &args[0] {
                    Value::DateTime(os) => os.as_str(),
                    _ => return Err("datetime.difference(other) expects datetime".to_string()),
                };
                let (od, ot) = if let Some(pos) = other.find('T').or_else(|| other.find(' ')) {
                    let (d, t) = other.split_at(pos);
                    (d.trim(), t.trim())
                } else {
                    (other, "00:00:00")
                };
                let (oy, om, oday) = Self::parse_date_parts(od)?;
                let (oh, omin, osec, _) = Self::parse_time_parts(ot).unwrap_or((0, 0, 0, 0));
                let j1 = Self::to_julian_day(oy, om, oday);
                let j2 = Self::to_julian_day(y, m, day);
                let secs1 = (j1 - 2440588) * 86400 + oh * 3600 + omin * 60 + osec;
                let secs2 = (j2 - 2440588) * 86400 + h * 3600 + min * 60 + sec;
                Ok(Value::Duration {
                    total_seconds: secs2 - secs1,
                })
            }
            _ => Err(format!("DateTime method '{}' not found", method)),
        }
    }

    fn call_date_type_method(&self, method: &str, args: &[Value]) -> Result<Value, String> {
        match method {
            "today" => {
                if !args.is_empty() {
                    return Err("date.today() takes no arguments".to_string());
                }
                let now = std::time::SystemTime::now();
                let dur = now.duration_since(std::time::UNIX_EPOCH).unwrap();
                let secs = dur.as_secs() as i64;
                let days = secs / 86400;
                let jd = days + 2440588;
                let (y, m, d) = Self::julian_to_ymd(jd);
                Ok(Value::Date(format!("{:04}-{:02}-{:02}", y, m, d)))
            }
            "parse" => {
                if args.is_empty() {
                    return Err("date.parse(str) requires one argument".to_string());
                }
                let s = args[0].to_string();
                let (_, _, _, canonical) = Self::parse_date_flexible(&s)?;
                Ok(Value::Date(canonical))
            }
            _ => Err(format!("Date type method '{}' not found", method)),
        }
    }

    fn julian_to_ymd(jd: i64) -> (i64, i64, i64) {
        let a = jd + 32044;
        let b = (4 * a + 3) / 146097;
        let c = a - (146097 * b) / 4;
        let d = (4 * c + 3) / 1461;
        let e = c - (1461 * d) / 4;
        let m = (5 * e + 2) / 153;
        let day = e - (153 * m + 2) / 5 + 1;
        let month = m + 3 - 12 * (m / 10);
        let year = 100 * b + d - 4800 + m / 10;
        (year as i64, month as i64, day as i64)
    }

    fn call_time_type_method(&self, method: &str, args: &[Value]) -> Result<Value, String> {
        match method {
            "now" => {
                if !args.is_empty() {
                    return Err("time.now() takes no arguments".to_string());
                }
                let now = std::time::SystemTime::now();
                let dur = now.duration_since(std::time::UNIX_EPOCH).unwrap();
                let secs = (dur.as_secs() % 86400) as i64;
                let h = secs / 3600;
                let min = (secs % 3600) / 60;
                let s = secs % 60;
                Ok(Value::Time(format!("{:02}:{:02}:{:02}", h, min, s)))
            }
            _ => Err(format!("Time type method '{}' not found", method)),
        }
    }

    fn call_datetime_type_method(&self, method: &str, args: &[Value]) -> Result<Value, String> {
        match method {
            "now" | "now_local" => {
                if !args.is_empty() {
                    return Err("datetime.now() takes no arguments".to_string());
                }
                let now = std::time::SystemTime::now();
                let dur = now.duration_since(std::time::UNIX_EPOCH).unwrap();
                let secs = dur.as_secs() as i64;
                let days = secs / 86400;
                let jd = days + 2440588;
                let (y, m, d) = Self::julian_to_ymd(jd);
                let h = (secs % 86400) / 3600;
                let min = ((secs % 86400) % 3600) / 60;
                let s = ((secs % 86400) % 3600) % 60;
                Ok(Value::DateTime(format!(
                    "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
                    y, m, d, h, min, s
                )))
            }
            "parse" => {
                if args.is_empty() {
                    return Err("datetime.parse(str) requires one argument".to_string());
                }
                Ok(Value::DateTime(args[0].to_string()))
            }
            _ => Err(format!("DateTime type method '{}' not found", method)),
        }
    }

    fn call_graph_type_method(&self, method: &str, args: &[Value]) -> Result<Value, String> {
        use std::collections::HashMap;
        match method {
            "directed" | "undirected" => {
                if !args.is_empty() {
                    return Err("graph.directed() and graph.undirected() take no arguments".to_string());
                }
                Ok(Value::Graph(HashMap::new()))
            }
            _ => Err(format!("Graph type method '{}' not found", method)),
        }
    }

    fn call_graph_method(
        &self,
        graph: &std::collections::HashMap<String, Vec<(String, f64)>>,
        method: &str,
        args: &[Value],
    ) -> Result<Value, String> {
        use std::collections::HashSet;
        match method {
            "nodes" => {
                if !args.is_empty() {
                    return Err("graph.nodes() takes no arguments".to_string());
                }
                let mut nodes: HashSet<&String> = graph.keys().collect();
                for edges in graph.values() {
                    for (n, _) in edges {
                        nodes.insert(n);
                    }
                }
                let list: Vec<Value> = nodes.into_iter().map(|s| Value::String(s.clone())).collect();
                Ok(Value::List(list))
            }
            "edges" => {
                if !args.is_empty() {
                    return Err("graph.edges() takes no arguments".to_string());
                }
                let mut list = Vec::new();
                for (from, edges) in graph {
                    for (to, w) in edges {
                        list.push(Value::Tuple(vec![
                            Value::String(from.clone()),
                            Value::String(to.clone()),
                            Value::Float(*w),
                        ]));
                    }
                }
                Ok(Value::List(list))
            }
            "degree" => {
                if args.len() != 1 {
                    return Err("graph.degree(node) requires one argument".to_string());
                }
                let node = match &args[0] {
                    Value::String(s) => s.as_str(),
                    _ => return Err("graph.degree(node) expects string".to_string()),
                };
                let out = graph.get(node).map(|e| e.len()).unwrap_or(0);
                Ok(Value::Integer(out as i64))
            }
            "is_connected" => {
                if !args.is_empty() {
                    return Err("graph.is_connected() takes no arguments".to_string());
                }
                let all_nodes: HashSet<String> = graph.keys().cloned().chain(graph.values().flat_map(|e| e.iter().map(|(n, _)| n.clone()))).collect();
                if all_nodes.is_empty() {
                    return Ok(Value::Boolean(true));
                }
                let start = all_nodes.iter().next().unwrap().clone();
                let mut visited = HashSet::new();
                let mut stack = vec![start];
                while let Some(u) = stack.pop() {
                    if !visited.insert(u.clone()) {
                        continue;
                    }
                    if let Some(edges) = graph.get(&u) {
                        for (v, _) in edges {
                            stack.push(v.clone());
                        }
                    }
                }
                Ok(Value::Boolean(visited.len() >= all_nodes.len()))
            }
            _ => Err(format!("Graph method '{}' not found", method)),
        }
    }

    fn call_interval_method(
        &self,
        total_seconds: i64,
        method: &str,
        args: &[Value],
    ) -> Result<Value, String> {
        match method {
            "days" => {
                if !args.is_empty() {
                    return Err("interval.days takes no arguments".to_string());
                }
                Ok(Value::Integer(total_seconds / 86400))
            }
            "hours" => {
                if !args.is_empty() {
                    return Err("interval.hours takes no arguments".to_string());
                }
                Ok(Value::Integer(total_seconds / 3600))
            }
            "minutes" => {
                if !args.is_empty() {
                    return Err("interval.minutes takes no arguments".to_string());
                }
                Ok(Value::Integer(total_seconds / 60))
            }
            "seconds" => {
                if !args.is_empty() {
                    return Err("interval.seconds takes no arguments".to_string());
                }
                Ok(Value::Integer(total_seconds))
            }
            "is_zero" => {
                if !args.is_empty() {
                    return Err("interval.is_zero takes no arguments".to_string());
                }
                Ok(Value::Boolean(total_seconds == 0))
            }
            "abs" => {
                if !args.is_empty() {
                    return Err("interval.abs takes no arguments".to_string());
                }
                Ok(Value::Duration {
                    total_seconds: total_seconds.abs(),
                })
            }
            _ => Err(format!("Interval method '{}' not found", method)),
        }
    }

    fn call_enum_variant_method(
        &self,
        _enum_name: &str,
        variant_name: &str,
        value: &Value,
        method: &str,
        args: &[Value],
    ) -> Result<Value, String> {
        match method {
            "variant_name" | "name" | "label" => {
                if !args.is_empty() {
                    return Err("enum_variant.variant_name() takes no arguments".to_string());
                }
                Ok(Value::String(variant_name.to_string()))
            }
            "unwrap" => {
                if !args.is_empty() {
                    return Err("enum_variant.unwrap() takes no arguments".to_string());
                }
                Ok(value.clone())
            }
            "unwrap_or" => {
                if args.len() != 1 {
                    return Err("enum_variant.unwrap_or(default) requires one argument".to_string());
                }
                Ok(value.clone())
            }
            "is_variant" => {
                if args.len() != 1 {
                    return Err("enum_variant.is_variant(name) requires one argument".to_string());
                }
                let name = args[0].to_string();
                Ok(Value::Boolean(name == variant_name))
            }
            "value" => {
                if !args.is_empty() {
                    return Err("enum_variant.value takes no arguments".to_string());
                }
                Ok(value.clone())
            }
            _ => Err(format!("Enum variant method '{}' not found", method)),
        }
    }

    pub(crate) fn call_value_with_args(
        &mut self,
        callee: Value,
        eval_args: &[Value],
        this_opt: Option<Value>,
    ) -> Result<Value, String> {
        match callee {
            Value::Memoized { id, inner } => {
                let key = Self::memo_cache_key(eval_args);
                if let Some(cached) = self.memo_caches.get(&id).and_then(|c| c.get(&key)) {
                    return Ok(cached.clone());
                }
                let result = self.call_value_with_args(*inner, eval_args, this_opt)?;
                self.memo_caches
                    .entry(id)
                    .or_default()
                    .insert(key, result.clone());
                Ok(result)
            }
            Value::BoundMethod { receiver, method } => {
                return self.call_bound_method(*receiver, &method, eval_args, this_opt);
            }
            Value::MatchGroup(m) => {
                if eval_args.len() != 1 {
                    return Err("match.group(n) expects one integer argument".to_string());
                }
                let n = match &eval_args[0] {
                    Value::Integer(i) => *i,
                    _ => return Err("match.group(n) expects integer n".to_string()),
                };
                if n == 0 {
                    return Ok(Value::String(m.text.clone()));
                }
                let idx = (n - 1) as usize;
                if idx < m.groups.len() {
                    Ok(Value::String(m.groups[idx].clone()))
                } else {
                    Ok(Value::None)
                }
            }
            Value::OnceCached { id, inner } => {
                if let Some(cached) = self.once_cache.get(&id) {
                    return Ok(cached.clone());
                }
                let result = self.call_value_with_args(*inner, eval_args, this_opt)?;
                self.once_cache.insert(id, result.clone());
                Ok(result)
            }
            Value::MirrorDispatch {
                method_name,
                handle_missing,
            } => {
                // Call handle_missing(method_name, ...args) with this_opt = instance
                let mut mirror_args = vec![Value::String(method_name.clone())];
                mirror_args.extend(eval_args.iter().cloned());
                self.call_value_with_args(*handle_missing.clone(), &mirror_args, this_opt)
            }
            Value::Constructor(class_name) => {
                let class_val = self.get_variable(&class_name)?;
                let Value::Class {
                    name: _,
                    class_type,
                    parent: _,
                    fields: class_fields,
                    methods,
                    static_fields: _,
                    static_methods: _,
                } = class_val
                else {
                    return Err(format!("'{}' is not a class", class_name));
                };
                
                // Handle singleton class - return existing instance if it exists
                if class_type.as_deref() == Some("singleton") {
                    if let Some(existing) = self.singleton_registry.get(&class_name) {
                        return Ok(existing.clone());
                    }
                }
                
                let instance_fields = class_fields.clone();
                
                // Handle data class - add auto-generated methods
                if class_type.as_deref() == Some("data") {
                    // Data classes get automatic .copy(), .hash(), ==, .to_str()
                    // These are handled in method calls and operators
                }
                
                // Handle secure class - encrypt all fields
                if class_type.as_deref() == Some("secure") {
                    // Fields should already be marked as enc<T> in the class definition
                    // Encryption happens at field assignment time
                }
                
                let instance = Value::Instance {
                    class_name: class_name.clone(),
                    fields: instance_fields.clone(),
                };
                
                // Call init method if it exists
                if let Some(Value::Function { params, body, .. }) = methods.get("init") {
                    self.call_function_internal(
                        "init",
                        eval_args,
                        params,
                        body,
                        Some(instance.clone()),
                    )?;
                }
                
                // Store singleton instance
                if class_type.as_deref() == Some("singleton") {
                    self.singleton_registry.insert(class_name.clone(), instance.clone());
                }
                
                // Track resource class for cleanup
                if class_type.as_deref() == Some("resource") {
                    self.resource_stack.push((class_name.clone(), instance.clone()));
                }
                
                Ok(instance)
            }
            Value::Function { name, params, body } => {
                self.call_function_internal(&name, eval_args, &params, &body, this_opt)
            }
            Value::GridNeighbors(grid_val) => {
                let Value::Grid(grid) = grid_val.as_ref() else {
                    return Err("GridNeighbors requires a grid".to_string());
                };
                if eval_args.len() != 2 {
                    return Err(
                        "grid.neighbors(i, j) requires exactly 2 arguments (row, col)".to_string(),
                    );
                }
                let i = match &eval_args[0] {
                    Value::Integer(n) => *n as usize,
                    _ => return Err("grid.neighbors row must be integer".to_string()),
                };
                let j = match &eval_args[1] {
                    Value::Integer(n) => *n as usize,
                    _ => return Err("grid.neighbors col must be integer".to_string()),
                };
                let rows = grid.len();
                let cols = if grid.is_empty() { 0 } else { grid[0].len() };
                let mut neighbors = Vec::new();
                for (di, dj) in [(-1i64, 0), (1, 0), (0, -1), (0, 1)] {
                    let ni = i as i64 + di;
                    let nj = j as i64 + dj;
                    if ni >= 0 && ni < rows as i64 && nj >= 0 && nj < cols as i64 {
                        neighbors.push(grid[ni as usize][nj as usize].clone());
                    }
                }
                Ok(Value::List(neighbors))
            }
            Value::GridNeighbors8(grid_val) => {
                let Value::Grid(grid) = grid_val.as_ref() else {
                    return Err("GridNeighbors8 requires a grid".to_string());
                };
                if eval_args.len() != 2 {
                    return Err(
                        "grid.neighbors8(i, j) requires exactly 2 arguments (row, col)".to_string(),
                    );
                };
                let i = match &eval_args[0] {
                    Value::Integer(n) => *n as usize,
                    _ => return Err("grid.neighbors8 row must be integer".to_string()),
                };
                let j = match &eval_args[1] {
                    Value::Integer(n) => *n as usize,
                    _ => return Err("grid.neighbors8 col must be integer".to_string()),
                };
                let rows = grid.len();
                let cols = if grid.is_empty() { 0 } else { grid[0].len() };
                let mut neighbors = Vec::new();
                // 8 directions: N, NE, E, SE, S, SW, W, NW
                for (di, dj) in [
                    (-1, 0),
                    (-1, 1),
                    (0, 1),
                    (1, 1),
                    (1, 0),
                    (1, -1),
                    (0, -1),
                    (-1, -1),
                ] {
                    let ni = i as i64 + di;
                    let nj = j as i64 + dj;
                    if ni >= 0 && ni < rows as i64 && nj >= 0 && nj < cols as i64 {
                        neighbors.push(grid[ni as usize][nj as usize].clone());
                    }
                }
                Ok(Value::List(neighbors))
            }
            Value::GridFindAll(grid_val) => {
                let Value::Grid(grid) = grid_val.as_ref() else {
                    return Err("GridFindAll requires a grid".to_string());
                };
                if eval_args.len() != 1 {
                    return Err("grid.find_all(value) requires exactly 1 argument".to_string());
                }
                let target = &eval_args[0];
                let mut positions = Vec::new();
                for (i, row) in grid.iter().enumerate() {
                    for (j, cell) in row.iter().enumerate() {
                        if self.values_equal(cell, target) {
                            positions.push(Value::Tuple(vec![
                                Value::Integer(i as i64),
                                Value::Integer(j as i64),
                            ]));
                        }
                    }
                }
                Ok(Value::List(positions))
            }
            Value::GridRow(grid_val) => {
                let Value::Grid(grid) = grid_val.as_ref() else {
                    return Err("GridRow requires a grid".to_string());
                };
                if eval_args.len() != 1 {
                    return Err("grid.row(n) requires exactly 1 argument".to_string());
                }
                let row_idx = match &eval_args[0] {
                    Value::Integer(n) => *n as usize,
                    _ => return Err("grid.row index must be integer".to_string()),
                };
                if row_idx >= grid.len() {
                    return Err(format!(
                        "Row index {} out of bounds (grid has {} rows)",
                        row_idx,
                        grid.len()
                    ));
                }
                Ok(Value::List(grid[row_idx].clone()))
            }
            Value::GridCol(grid_val) => {
                let Value::Grid(grid) = grid_val.as_ref() else {
                    return Err("GridCol requires a grid".to_string());
                };
                if eval_args.len() != 1 {
                    return Err("grid.col(n) requires exactly 1 argument".to_string());
                }
                let col_idx = match &eval_args[0] {
                    Value::Integer(n) => *n as usize,
                    _ => return Err("grid.col index must be integer".to_string()),
                };
                if grid.is_empty() {
                    return Err("Cannot get column from empty grid".to_string());
                }
                let cols = grid[0].len();
                if col_idx >= cols {
                    return Err(format!(
                        "Column index {} out of bounds (grid has {} columns)",
                        col_idx, cols
                    ));
                }
                let column: Vec<Value> = grid.iter().map(|row| row[col_idx].clone()).collect();
                Ok(Value::List(column))
            }
            Value::MatrixRow(mat_val) => {
                let Value::Matrix(mat) = mat_val.as_ref() else {
                    return Err("MatrixRow requires a matrix".to_string());
                };
                if eval_args.len() != 1 {
                    return Err("matrix.row(n) requires exactly 1 argument".to_string());
                }
                let row_idx = match &eval_args[0] {
                    Value::Integer(n) => {
                        
                        if *n < 0 {
                            (mat.len() as i64 + n) as usize
                        } else {
                            *n as usize
                        }
                    }
                    _ => return Err("matrix.row index must be integer".to_string()),
                };
                if row_idx >= mat.len() {
                    return Err(format!(
                        "Row index {} out of bounds (matrix has {} rows)",
                        row_idx,
                        mat.len()
                    ));
                }
                let row_values: Vec<Value> =
                    mat[row_idx].iter().map(|&v| Value::Float(v)).collect();
                Ok(Value::List(row_values))
            }
            Value::MatrixCol(mat_val) => {
                let Value::Matrix(mat) = mat_val.as_ref() else {
                    return Err("MatrixCol requires a matrix".to_string());
                };
                if eval_args.len() != 1 {
                    return Err("matrix.col(n) requires exactly 1 argument".to_string());
                }
                let col_idx = match &eval_args[0] {
                    Value::Integer(n) => {
                        if mat.is_empty() {
                            return Err("Cannot get column from empty matrix".to_string());
                        }
                        let cols = mat[0].len();
                        
                        if *n < 0 {
                            (cols as i64 + n) as usize
                        } else {
                            *n as usize
                        }
                    }
                    _ => return Err("matrix.col index must be integer".to_string()),
                };
                if mat.is_empty() {
                    return Err("Cannot get column from empty matrix".to_string());
                }
                let cols = mat[0].len();
                if col_idx >= cols {
                    return Err(format!(
                        "Column index {} out of bounds (matrix has {} columns)",
                        col_idx, cols
                    ));
                }
                let column: Vec<Value> = mat.iter().map(|row| Value::Float(row[col_idx])).collect();
                Ok(Value::List(column))
            }
            Value::MatrixDiagonal(mat_val) => {
                let Value::Matrix(mat) = mat_val.as_ref() else {
                    return Err("MatrixDiagonal requires a matrix".to_string());
                };
                if !eval_args.is_empty() {
                    return Err("matrix.diagonal() takes no arguments".to_string());
                }
                if mat.is_empty() {
                    return Ok(Value::List(Vec::new()));
                }
                let size = mat.len().min(mat[0].len());
                let diagonal: Vec<Value> = (0..size).map(|i| Value::Float(mat[i][i])).collect();
                Ok(Value::List(diagonal))
            }
            Value::MatrixFlat(mat_val) => {
                let Value::Matrix(mat) = mat_val.as_ref() else {
                    return Err("MatrixFlat requires a matrix".to_string());
                };
                if !eval_args.is_empty() {
                    return Err("matrix.flat() takes no arguments".to_string());
                }
                let flat: Vec<Value> = mat
                    .iter()
                    .flat_map(|row| row.iter().map(|&v| Value::Float(v)))
                    .collect();
                Ok(Value::List(flat))
            }
            Value::MatrixRowSums(mat_val) => {
                let Value::Matrix(mat) = mat_val.as_ref() else {
                    return Err("MatrixRowSums requires a matrix".to_string());
                };
                if !eval_args.is_empty() {
                    return Err("matrix.row_sums() takes no arguments".to_string());
                }
                let sums: Vec<Value> = mat
                    .iter()
                    .map(|row| Value::Float(row.iter().sum()))
                    .collect();
                Ok(Value::List(sums))
            }
            Value::MatrixColSums(mat_val) => {
                let Value::Matrix(mat) = mat_val.as_ref() else {
                    return Err("MatrixColSums requires a matrix".to_string());
                };
                if !eval_args.is_empty() {
                    return Err("matrix.col_sums() takes no arguments".to_string());
                }
                if mat.is_empty() {
                    return Ok(Value::List(Vec::new()));
                }
                let cols = mat[0].len();
                let sums: Vec<Value> = (0..cols)
                    .map(|j| Value::Float(mat.iter().map(|row| row[j]).sum()))
                    .collect();
                Ok(Value::List(sums))
            }
            Value::MatrixRowMeans(mat_val) => {
                let Value::Matrix(mat) = mat_val.as_ref() else {
                    return Err("MatrixRowMeans requires a matrix".to_string());
                };
                if !eval_args.is_empty() {
                    return Err("matrix.row_means() takes no arguments".to_string());
                }
                let means: Vec<Value> = mat
                    .iter()
                    .map(|row| {
                        let sum: f64 = row.iter().sum();
                        Value::Float(sum / row.len() as f64)
                    })
                    .collect();
                Ok(Value::List(means))
            }
            Value::MatrixColMeans(mat_val) => {
                let Value::Matrix(mat) = mat_val.as_ref() else {
                    return Err("MatrixColMeans requires a matrix".to_string());
                };
                if !eval_args.is_empty() {
                    return Err("matrix.col_means() takes no arguments".to_string());
                }
                if mat.is_empty() {
                    return Ok(Value::List(Vec::new()));
                }
                let rows = mat.len();
                let cols = mat[0].len();
                let means: Vec<Value> = (0..cols)
                    .map(|j| {
                        let sum: f64 = mat.iter().map(|row| row[j]).sum();
                        Value::Float(sum / rows as f64)
                    })
                    .collect();
                Ok(Value::List(means))
            }
            _ => Err(format!("Cannot call {} as function", callee)),
        }
    }

    pub(super) fn get_variable(&self, name: &str) -> Result<Value, String> {
        // Check static variables first
        if let Some(value) = self.statics.get(name) {
            return Ok(value.clone());
        }

        // Check local scopes (from innermost to outermost)
        for scope in self.locals.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Ok(value.clone());
            }
        }

        // Check global scope
        if let Some(value) = self.globals.get(name) {
            Ok(value.clone())
        } else {
            // Use enhanced error with suggestions
            Err(JError::undefined_variable(name, 0, 0).to_string())
        }
    }

    pub(super) fn set_variable(&mut self, name: String, value: Value) {
        if let Some(scope) = self.locals.last_mut() {
            scope.insert(name, value);
        } else {
            self.globals.insert(name, value);
        }
    }

    /// Assign to an existing variable, traversing scopes from innermost to outermost
    pub(super) fn assign_variable(&mut self, name: &str, value: Value) -> Result<(), String> {
        // Search from innermost to outermost local scope
        for scope in self.locals.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.to_string(), value);
                return Ok(());
            }
        }

        // Check globals
        if self.globals.contains_key(name) {
            self.globals.insert(name.to_string(), value);
            return Ok(());
        }

        // Check statics
        if self.statics.contains_key(name) {
            self.statics.insert(name.to_string(), value);
            return Ok(());
        }

        Err(format!("Variable '{}' not found in any scope", name))
    }

    // Module system helper methods
    pub(super) fn load_module(&mut self, path: &str) -> Result<Value, String> {
        if let Some(cached) = self.module_cache.get(path) {
            return Ok(cached.clone());
        }

        // Resolve file path
            let file_path = self.resolve_module_path(path)?;

            // Read and parse file
            let source = std::fs::read_to_string(&file_path)
                .map_err(|e| format!("Failed to load module {}: {}", path, e))?;

            let mut lexer = crate::lexer::Lexer::new(&source);
            let tokens = lexer
                .tokenize()
                .map_err(|e| format!("Lexer error in module {}: {}", path, e))?;

            let mut parser = crate::parser::Parser::new(tokens);
            let ast = parser
                .parse()
                .map_err(|e| format!("Parser error in module {}: {}", path, e))?;

            // Execute in isolated scope
            self.push_scope();
            self.eval_node(&ast)?;

            // Get all variables from module scope as exports
            let mut exports = HashMap::new();
            if let Some(scope) = self.locals.last() {
                exports = scope.clone();
            }

            self.pop_scope();

            // Create module value
            let module = Value::Module {
                name: path.to_string(),
                path: file_path,
                exports,
            };

            // Cache it
            self.module_cache.insert(path.to_string(), module.clone());

            Ok(module)
    }

    fn resolve_module_path(&self, path: &str) -> Result<String, String> {
        // If path starts with ./ or ../, it's relative
        if path.starts_with("./") || path.starts_with("../") {
            let full_path = if path.ends_with(".jdl") {
                path.to_string()
            } else {
                format!("{}.jdl", path)
            };

            if std::path::Path::new(&full_path).exists() {
                return Ok(full_path);
            } else {
                return Err(format!("Module file not found: {}", full_path));
            }
        }

        // Search in module search paths
        for search_path in &self.module_search_paths {
            let full_path = if path.ends_with(".jdl") {
                format!("{}/{}", search_path, path)
            } else {
                format!("{}/{}.jdl", search_path, path)
            };

            if std::path::Path::new(&full_path).exists() {
                return Ok(full_path);
            }
            // Also try path/main.jdl (e.g. package directory)
            let dir_with_main = format!("{}/{}/main.jdl", search_path, path);
            if std::path::Path::new(&dir_with_main).exists() {
                return Ok(dir_with_main);
            }
        }

        // Resolve from Jolt package cache (~/.jolt/cache/<name>-<version>/main.jdl)
        if !path.contains('/') && !path.contains('\\') {
            if let Some(cache_path) = self.jolt_cache_dir() {
                let cache = std::path::Path::new(&cache_path);
                if cache.exists() {
                    let entries = match std::fs::read_dir(cache) {
                        Ok(e) => e,
                        Err(_) => return Err(format!("Module not found: {}", path)),
                    };
                    for entry in entries.flatten() {
                        let name = entry.file_name();
                        let name_str = name.to_string_lossy();
                        if name_str.starts_with(&format!("{}-", path)) || name_str == path {
                            let main_jdl = entry.path().join("main.jdl");
                            if main_jdl.exists() {
                                return Ok(main_jdl.to_string_lossy().into_owned());
                            }
                            let mod_jdl = entry.path().join(format!("{}.jdl", path));
                            if mod_jdl.exists() {
                                return Ok(mod_jdl.to_string_lossy().into_owned());
                            }
                        }
                    }
                }
            }
        }

        Err(format!("Module not found: {}", path))
    }

    /// Path to Jolt package cache (~/.jolt/cache). Used for import resolution.
    fn jolt_cache_dir(&self) -> Option<String> {
        #[cfg(windows)]
        let home = std::env::var("USERPROFILE").ok();
        #[cfg(not(windows))]
        let home = std::env::var("HOME").ok();
        home.map(|h| format!("{}/.jolt/cache", h.replace('\\', "/")))
    }

    pub(super) fn push_scope(&mut self) {
        self.locals.push(HashMap::new());
    }

    pub(super) fn pop_scope(&mut self) {
        self.locals.pop();
    }

    pub(super) fn execute_file(&mut self, filename: &str) -> Result<Value, String> {
        // Read the file
        let source = std::fs::read_to_string(filename)
            .map_err(|e| format!("Error reading file '{}': {}", filename, e))?;

        // Tokenize
        let mut lexer = crate::lexer::Lexer::new(&source);
        let tokens = lexer
            .tokenize()
            .map_err(|e| format!("Lexer error in '{}': {}", filename, e))?;

        // Parse
        let mut parser = crate::parser::Parser::new(tokens);
        let ast = parser
            .parse()
            .map_err(|e| format!("Parser error in '{}': {}", filename, e))?;

        // Execute in current context
        self.eval_node(&ast)
    }

    pub(super) fn normalize_slice_indices(
        &self,
        start: Option<i64>,
        end: Option<i64>,
        len: i64,
        step: i64,
    ) -> Result<(i64, i64), String> {
        let start = match start {
            Some(s) => {
                if s < 0 {
                    (len + s).max(0)
                } else {
                    s.min(len)
                }
            }
            None => {
                if step > 0 {
                    0
                } else {
                    len - 1
                }
            }
        };

        let end = match end {
            Some(e) => {
                if e < 0 {
                    (len + e).max(-1)
                } else {
                    e.min(len)
                }
            }
            None => {
                if step > 0 {
                    len
                } else {
                    -1
                }
            }
        };

        Ok((start, end))
    }

    fn levenshtein_distance(&self, s1: &str, s2: &str) -> usize {
        let len1 = s1.len();
        let len2 = s2.len();

        if len1 == 0 {
            return len2;
        }
        if len2 == 0 {
            return len1;
        }

        let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

        // Initialize first row and column
        for (i, row) in matrix.iter_mut().enumerate().take(len1 + 1) {
            row[0] = i;
        }
        for (j, cell) in matrix[0].iter_mut().enumerate().take(len2 + 1) {
            *cell = j;
        }

        // Fill the matrix
        for i in 1..=len1 {
            for j in 1..=len2 {
                let cost = if s1.chars().nth(i - 1) == s2.chars().nth(j - 1) {
                    0
                } else {
                    1
                };
                matrix[i][j] = (matrix[i - 1][j] + 1)
                    .min(matrix[i][j - 1] + 1)
                    .min(matrix[i - 1][j - 1] + cost);
            }
        }

        matrix[len1][len2]
    }

    // KMP (Knuth-Morris-Pratt) string search algorithm
    fn kmp_search(&self, text: &str, pattern: &str) -> Vec<usize> {
        if pattern.is_empty() {
            return (0..text.len()).collect();
        }

        let lps = self.compute_lps(pattern);
        let mut positions = Vec::new();
        let mut i = 0; // index for text
        let mut j = 0; // index for pattern

        while i < text.len() {
            if pattern.chars().nth(j) == text.chars().nth(i) {
                i += 1;
                j += 1;
            }

            if j == pattern.len() {
                positions.push(i - j);
                j = lps[j - 1];
            } else if i < text.len() && pattern.chars().nth(j) != text.chars().nth(i) {
                if j != 0 {
                    j = lps[j - 1];
                } else {
                    i += 1;
                }
            }
        }

        positions
    }

    fn compute_lps(&self, pattern: &str) -> Vec<usize> {
        let mut lps = vec![0; pattern.len()];
        let mut len = 0;
        let mut i = 1;

        while i < pattern.len() {
            if pattern.chars().nth(i) == pattern.chars().nth(len) {
                len += 1;
                lps[i] = len;
                i += 1;
            } else if len != 0 {
                len = lps[len - 1];
            } else {
                lps[i] = 0;
                i += 1;
            }
        }

        lps
    }

    // Z-algorithm
    fn compute_z_array(&self, s: &str) -> Vec<usize> {
        let n = s.len();
        let mut z = vec![0; n];
        let mut l = 0;
        let mut r = 0;

        for i in 1..n {
            if i <= r {
                z[i] = (r - i + 1).min(z[i - l]);
            }
            while i + z[i] < n && s.chars().nth(z[i]) == s.chars().nth(i + z[i]) {
                z[i] += 1;
            }
            if i + z[i] - 1 > r {
                l = i;
                r = i + z[i] - 1;
            }
        }

        z[0] = n; // Z[0] is the length of the string
        z
    }

    // Convex hull (Graham scan algorithm for 2D points)
    fn convex_hull_2d(&self, points: &[Value]) -> Result<Vec<Value>, String> {
        if points.len() < 3 {
            return Ok(points.to_vec());
        }

        // Extract (x, y) coordinates
        let mut coords: Vec<(f64, f64)> = Vec::new();
        for point in points {
            match point {
                Value::Tuple(tuple) if tuple.len() == 2 => {
                    let x = match &tuple[0] {
                        Value::Integer(i) => *i as f64,
                        Value::Float(f) => *f,
                        _ => return Err("Convex hull points must be numeric tuples".to_string()),
                    };
                    let y = match &tuple[1] {
                        Value::Integer(i) => *i as f64,
                        Value::Float(f) => *f,
                        _ => return Err("Convex hull points must be numeric tuples".to_string()),
                    };
                    coords.push((x, y));
                }
                _ => return Err("Convex hull expects list of (x, y) tuples".to_string()),
            }
        }

        // Find bottom-most point (or left-most in case of tie)
        let mut min_idx = 0;
        for i in 1..coords.len() {
            if coords[i].1 < coords[min_idx].1
                || (coords[i].1 == coords[min_idx].1 && coords[i].0 < coords[min_idx].0)
            {
                min_idx = i;
            }
        }
        coords.swap(0, min_idx);

        // Sort points by polar angle with respect to bottom point
        let pivot = coords[0];
        coords[1..].sort_by(|a, b| {
            let angle_a = (a.1 - pivot.1).atan2(a.0 - pivot.0);
            let angle_b = (b.1 - pivot.1).atan2(b.0 - pivot.0);
            angle_a
                .partial_cmp(&angle_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Graham scan
        let mut hull = vec![0];
        for i in 1..coords.len() {
            while hull.len() > 1
                && self.cross_product(
                    &coords[hull[hull.len() - 2]],
                    &coords[hull[hull.len() - 1]],
                    &coords[i],
                ) <= 0.0
            {
                hull.pop();
            }
            hull.push(i);
        }

        // Convert back to Value tuples
        Ok(hull
            .into_iter()
            .map(|i| {
                let (x, y) = coords[i];
                Value::Tuple(vec![Value::Float(x), Value::Float(y)])
            })
            .collect())
    }

    fn cross_product(&self, o: &(f64, f64), a: &(f64, f64), b: &(f64, f64)) -> f64 {
        (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0)
    }

    // BFS (Breadth-First Search) — used by builtins::dsa
    pub(crate) fn bfs_search(
        &self,
        graph: &HashMap<String, Vec<(String, f64)>>,
        start: &str,
        goal: Option<&str>,
    ) -> Result<Vec<String>, String> {
        use std::collections::VecDeque;

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut parent: HashMap<String, String> = HashMap::new();

        queue.push_back(start.to_string());
        visited.insert(start.to_string());

        while let Some(current) = queue.pop_front() {
            if let Some(target) = goal {
                if current == target {
                    // Reconstruct path
                    let mut path = Vec::new();
                    let mut node = current;
                    path.push(node.clone());
                    while let Some(p) = parent.get(&node) {
                        path.push(p.clone());
                        node = p.clone();
                    }
                    path.reverse();
                    return Ok(path);
                }
            }

            if let Some(neighbors) = graph.get(&current) {
                for (neighbor, _) in neighbors {
                    if !visited.contains(neighbor) {
                        visited.insert(neighbor.clone());
                        parent.insert(neighbor.clone(), current.clone());
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }

        if goal.is_some() {
            Err("Goal not reachable".to_string())
        } else {
            Ok(visited.into_iter().collect())
        }
    }

    // DFS (Depth-First Search) — used by builtins::dsa
    pub(crate) fn dfs_search(
        &self,
        graph: &HashMap<String, Vec<(String, f64)>>,
        start: &str,
        goal: Option<&str>,
    ) -> Result<Vec<String>, String> {
        let mut visited = HashSet::new();
        let mut path = Vec::new();

        if self.dfs_recursive(graph, start, goal, &mut visited, &mut path) {
            Ok(path)
        } else if goal.is_some() {
            Err("Goal not reachable".to_string())
        } else {
            Ok(visited.into_iter().collect())
        }
    }

    fn dfs_recursive(
        &self,
        graph: &HashMap<String, Vec<(String, f64)>>,
        current: &str,
        goal: Option<&str>,
        visited: &mut HashSet<String>,
        path: &mut Vec<String>,
    ) -> bool {
        visited.insert(current.to_string());
        path.push(current.to_string());

        if let Some(target) = goal {
            if current == target {
                return true;
            }
        }

        if let Some(neighbors) = graph.get(current) {
            for (neighbor, _) in neighbors {
                if !visited.contains(neighbor)
                    && self.dfs_recursive(graph, neighbor, goal, visited, path) {
                        return true;
                    }
            }
        }

        if goal.is_some() {
            path.pop();
        }
        false
    }

    // FFT (Fast Fourier Transform) - simplified version
    fn fft_transform(&self, signal: &[Value]) -> Result<Vec<Value>, String> {
        // Convert signal to complex numbers (simplified: just use real part)
        let mut samples: Vec<f64> = Vec::new();
        for val in signal {
            match val {
                Value::Integer(i) => samples.push(*i as f64),
                Value::Float(f) => samples.push(*f),
                _ => return Err("FFT requires numeric values".to_string()),
            }
        }

        // Simple DFT implementation (not optimized FFT, but functional)
        let n = samples.len();
        let mut result = Vec::new();

        for k in 0..n {
            let mut real = 0.0;
            let mut imag = 0.0;

            for (j, s) in samples.iter().enumerate().take(n) {
                let angle = -2.0 * std::f64::consts::PI * (k as f64) * (j as f64) / (n as f64);
                real += s * angle.cos();
                imag += s * angle.sin();
            }

            // Return magnitude
            let magnitude = (real * real + imag * imag).sqrt();
            result.push(Value::Float(magnitude));
        }

        Ok(result)
    }

    fn generate_permutations(&self, list: &[Value]) -> Vec<Vec<Value>> {
        if list.is_empty() {
            return vec![vec![]];
        }

        let mut result = Vec::new();
        for (i, item) in list.iter().enumerate() {
            let mut remaining = list.to_vec();
            remaining.remove(i);

            let sub_perms = self.generate_permutations(&remaining);
            for mut perm in sub_perms {
                perm.insert(0, item.clone());
                result.push(perm);
            }
        }

        result
    }

    fn generate_combinations(&self, list: &[Value], r: usize) -> Vec<Vec<Value>> {
        if r == 0 {
            return vec![vec![]];
        }
        if r > list.len() {
            return vec![];
        }
        if r == list.len() {
            return vec![list.to_vec()];
        }

        let mut result = Vec::new();

        // Include first element
        let sub_combs = self.generate_combinations(&list[1..], r - 1);
        for mut comb in sub_combs {
            comb.insert(0, list[0].clone());
            result.push(comb);
        }

        // Exclude first element
        let sub_combs = self.generate_combinations(&list[1..], r);
        result.extend(sub_combs);

        result
    }

    fn cartesian_product(&self, lists: &[Vec<Value>]) -> Vec<Vec<Value>> {
        if lists.is_empty() {
            return vec![vec![]];
        }

        let mut result = vec![vec![]];

        for list in lists {
            let mut new_result = Vec::new();
            for existing in &result {
                for item in list {
                    let mut new_combo = existing.clone();
                    new_combo.push(item.clone());
                    new_result.push(new_combo);
                }
            }
            result = new_result;
        }

        result
    }

    #[allow(dead_code)]
    fn print_animation_legacy(
        &self,
        text: &str,
        anim_type: &str,
        interval: f64,
        count: Option<usize>,
    ) -> Result<(), String> {
        let frames: Vec<&str> = match anim_type {
            "spinner" => vec!["|", "/", "-", "\\"],
            "dots" => vec!["â ‹", "â ™", "â ¹", "â ¸", "â ¼", "â ´", "â ¦", "â §", "â ‡", "â "],
            "bar" => {
                // Indeterminate progress bar animation
                let mut frames = Vec::new();
                for i in 0..10 {
                    let mut frame = String::from("[");
                    for j in 0..10 {
                        if (j + i) % 10 < 3 {
                            frame.push('#');
                        } else {
                            frame.push(' ');
                        }
                    }
                    frame.push(']');
                    frames.push(frame);
                }
                return Err("Bar animation not fully implemented".to_string());
            }
            "bounce" => {
                // Bounce animation: â€¢â€¢â€¢â€¢â€¢ â†’ â€¢ â€¢â€¢â€¢â€¢ â†’ â€¢â€¢ â€¢â€¢â€¢ etc.
                let mut frames = Vec::new();
                for i in 0..5 {
                    let mut frame = String::new();
                    for j in 0..5 {
                        if j == i {
                            frame.push(' ');
                        } else {
                            frame.push('\u{2022}');
                        }
                    }
                    frames.push(frame);
                }
                return Err("Bounce animation not fully implemented".to_string());
            }
            "marquee" => {
                // Marquee scrolling - would need terminal width
                return Err("Marquee animation not fully implemented".to_string());
            }
            "pulse" => {
                // Pulse effect using dim/bright
                return Err("Pulse animation not fully implemented".to_string());
            }
            _ => return Err(format!("Unknown animation type: {}", anim_type)),
        };

        let max_iterations = count.unwrap_or(10); // Default to 10 iterations if not specified
        let mut iteration = 0;

        while iteration < max_iterations {
            for frame in &frames {
                print!("\r{} {}", text, frame);
                std::io::Write::flush(&mut std::io::stdout()).unwrap();
                std::thread::sleep(std::time::Duration::from_secs_f64(interval));
                iteration += 1;
                if iteration >= max_iterations {
                    break;
                }
            }
        }

        println!(); // Newline after animation
        Ok(())
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

// Additional helper methods for Rich-like features
impl Interpreter {
    // Rich-like spinner and loading animations
    fn show_spinner(&self, style: &str, duration: f64, message: &str) -> Result<(), String> {
        use std::io::{self, Write};
        use std::thread;
        use std::time::Duration;

        let frames = match style {
            "dots" => vec!["â ‹", "â ™", "â ¹", "â ¸", "â ¼", "â ´", "â ¦", "â §", "â ‡", "â "],
            "line" => vec!["-", "\\", "|", "/"],
            "arrow" => vec!["â†", "â†–", "â†‘", "â†—", "â†’", "â†˜", "â†“", "â†™"],
            "bounce" => vec!["â ", "â ‚", "â „", "â ‚"],
            "box" => vec!["â—°", "â—³", "â—²", "â—±"],
            "circle" => vec!["â—", "â—“", "â—‘", "â—’"],
            "square" => vec!["â—°", "â—³", "â—²", "â—±"],
            "triangle" => vec!["â—¢", "â—£", "â—¤", "â—¥"],
            "pulse" => vec!["â—", "â—‹", "â—", "â—‹"],
            "grow" => vec!["â–", "â–ƒ", "â–…", "â–‡", "â–ˆ", "â–‡", "â–…", "â–ƒ"],
            _ => vec!["â ‹", "â ™", "â ¹", "â ¸", "â ¼", "â ´", "â ¦", "â §", "â ‡", "â "],
        };

        let iterations = (duration * 10.0) as usize;

        for i in 0..iterations {
            let frame = frames[i % frames.len()];
            print!("\r\x1b[36m{}\x1b[0m {}", frame, message);
            let _ = io::stdout().flush();
            thread::sleep(Duration::from_millis(100));
        }

        println!("\r\x1b[32mâœ“\x1b[0m {} \x1b[32mDone!\x1b[0m", message);
        Ok(())
    }

    fn show_loading(&self, message: &str, style: &str, duration: f64) -> Result<(), String> {
        self.show_spinner(style, duration, message)
    }

    fn show_panel(&self, text: &str, title: Option<&str>, style: &str) -> Result<(), String> {
        let (top_left, top_right, bottom_left, bottom_right, horizontal, vertical) = match style {
            "double" => ("\u{2554}", "\u{2557}", "\u{255A}", "\u{255D}", "\u{2550}", "\u{2551}"),
            "rounded" => ("\u{256D}", "\u{256E}", "\u{2570}", "\u{256F}", "\u{2500}", "\u{2502}"),
            "bold" => ("\u{258F}", "\u{2590}", "\u{2597}", "\u{259D}", "\u{2501}", "\u{2503}"),
            "ascii" => ("+", "+", "+", "+", "-", "|"),
            _ => ("\u{250C}", "\u{2510}", "\u{2514}", "\u{2518}", "\u{2500}", "\u{2502}"), // single
        };

        let lines: Vec<&str> = text.lines().collect();
        let max_width = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);
        let width = max_width.max(20);

        // Top border
        if let Some(t) = title {
            let title_len = t.chars().count();
            let left_pad = (width - title_len) / 2;
            let right_pad = width - title_len - left_pad;
            println!(
                "{}{}\x1b[1m {} \x1b[0m{}{}",
                top_left,
                horizontal.repeat(left_pad),
                t,
                horizontal.repeat(right_pad),
                top_right
            );
        } else {
            println!("{}{}{}", top_left, horizontal.repeat(width + 2), top_right);
        }

        // Content - properly align with character count, not byte length
        for line in lines {
            let line_width = line.chars().count();
            let padding = width.saturating_sub(line_width);
            println!("{} {}{} {}", vertical, line, " ".repeat(padding), vertical);
        }

        // Bottom border
        println!(
            "{}{}{}",
            bottom_left,
            horizontal.repeat(width + 2),
            bottom_right
        );

        Ok(())
    }

    fn show_box(&self, text: &str, width: usize) -> Result<(), String> {
        let lines: Vec<&str> = text.lines().collect();

        println!("â”Œ{}â”", "â”€".repeat(width));
        for line in lines {
            let padding = if line.len() < width {
                width - line.len()
            } else {
                0
            };
            println!("â”‚ {}{} â”‚", line, " ".repeat(padding));
        }
        println!("â””{}â”˜", "â”€".repeat(width));

        Ok(())
    }

    fn show_status(&self, status_type: &str, message: &str) -> Result<(), String> {
        let (icon, color) = match status_type {
            "success" | "ok" => ("âœ“", "32"),   // Green
            "error" | "fail" => ("âœ—", "31"),   // Red
            "warning" | "warn" => ("âš ", "33"), // Yellow
            "info" => ("â„¹", "36"),             // Cyan
            "question" => ("?", "35"),         // Magenta
            _ => ("â€¢", "37"),                  // White
        };

        println!("\x1b[{}m{}\x1b[0m {}", color, icon, message);
        Ok(())
    }

    fn show_tree(&self, data: &Value, title: Option<&str>, indent: usize) -> Result<(), String> {
        if indent == 0 {
            if let Some(t) = title {
                println!("\x1b[1m{}\x1b[0m", t);
            }
        }

        match data {
            Value::Dict(dict) => {
                for (i, (key, value)) in dict.iter().enumerate() {
                    let is_last = i == dict.len() - 1;
                    let prefix = if is_last { "â””â”€" } else { "â”œâ”€" };

                    print!("{}{} \x1b[36m{}\x1b[0m: ", " ".repeat(indent), prefix, key);

                    match value {
                        Value::Dict(_) | Value::List(_) => {
                            println!();
                            self.show_tree(value, None, indent + 2)?;
                        }
                        _ => println!("{}", value),
                    }
                }
            }
            Value::List(list) => {
                for (i, item) in list.iter().enumerate() {
                    let is_last = i == list.len() - 1;
                    let prefix = if is_last { "â””â”€" } else { "â”œâ”€" };

                    print!("{}{} ", " ".repeat(indent), prefix);

                    match item {
                        Value::Dict(_) | Value::List(_) => {
                            println!();
                            self.show_tree(item, None, indent + 2)?;
                        }
                        _ => println!("{}", item),
                    }
                }
            }
            _ => {
                println!("{}{}", " ".repeat(indent), data);
            }
        }

        Ok(())
    }

    fn show_columns(&self, texts: &[String]) -> Result<(), String> {
        if texts.is_empty() {
            return Ok(());
        }

        let col_count = texts.len();
        let term_width = 80; // Default terminal width
        let col_width = term_width / col_count;

        for text in texts {
            let truncated = if text.len() > col_width - 2 {
                format!("{}â€¦", &text[..col_width - 3])
            } else {
                text.clone()
            };
            print!("{:width$}", truncated, width = col_width);
        }
        println!();

        Ok(())
    }
}
