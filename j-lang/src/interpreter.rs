use crate::parser::{AstNode, BinaryOp, UnaryOp, Pattern};
use crate::error::JError;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Char(char),
    List(Vec<Value>),
    Dict(HashMap<String, Value>),
    Set(HashSet<String>), // For simplicity, using String keys
    Counter(HashMap<String, i64>), // Frequency counter
    Deque(Vec<Value>), // Double-ended queue
    PriorityQ(Vec<(i64, Value)>), // Priority queue (min-heap by default)
    Graph(HashMap<String, Vec<(String, f64)>>), // Graph: node -> [(neighbor, weight), ...]
    Tree {
        value: Box<Value>,
        children: Vec<Value>, // Children are also Tree values
    },
    Function {
        name: String,
        params: Vec<String>,
        body: Box<AstNode>,
    },
    Infinity(bool), // true for +inf, false for -inf
    Emoji(String),
    Money(String, f64), // (currency_symbol, amount)
    Hex(String),
    Date(String),
    Time(String),
    DateTime(String),
    Tuple(Vec<Value>),
    Range(i64, i64, i64), // start, end, step
    Task(u64), // task ID
    Channel(u64), // channel ID
    Vector(Vec<f64>), // 1D vector
    Matrix(Vec<Vec<f64>>), // 2D matrix
    Grid(Vec<Vec<Value>>), // 2D grid with neighbor logic
    GridNeighbors(Box<Value>), // callable: grid.neighbors(i, j) -> list of adjacent cell values
    Enum {
        name: String,
        variants: HashMap<String, Value>,
    },
    EnumVariant {
        enum_name: String,
        variant_name: String,
        value: Box<Value>,
    },
    Class {
        name: String,
        parent: Option<String>,
        fields: HashMap<String, Value>,
        methods: HashMap<String, Value>,
        static_fields: HashMap<String, Value>,
        static_methods: HashMap<String, Value>,
    },
    Instance {
        class_name: String,
        fields: HashMap<String, Value>,
    },
    /// Constructor function for Class.new() - creates instances
    Constructor(String), // class name
    None,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Integer(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Char(c) => write!(f, "{}", c),
            Value::List(list) => {
                write!(f, "[")?;
                for (i, item) in list.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            Value::Dict(dict) => {
                write!(f, "{{")?;
                for (i, (key, value)) in dict.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}: {}", key, value)?;
                }
                write!(f, "}}")
            }
            Value::Set(set) => {
                write!(f, "{{")?;
                for (i, item) in set.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", item)?;
                }
                write!(f, "}}")
            }
            Value::Counter(counter) => {
                write!(f, "Counter{{")?;
                for (i, (key, count)) in counter.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}: {}", key, count)?;
                }
                write!(f, "}}")
            }
            Value::Function { name, .. } => write!(f, "<function {}>", name),
            Value::Enum { name, .. } => write!(f, "<enum {}>", name),
            Value::EnumVariant { variant_name, .. } => write!(f, "{}", variant_name),
            Value::Class { name, .. } => write!(f, "<class {}>", name),
            Value::Instance { class_name, .. } => write!(f, "<{} instance>", class_name),
            Value::Constructor(class_name) => write!(f, "<constructor {}>", class_name),
            Value::GridNeighbors(_) => write!(f, "<grid.neighbors>"),
            Value::Infinity(positive) => {
                if *positive {
                    write!(f, "inf")
                } else {
                    write!(f, "-inf")
                }
            }
            Value::Emoji(e) => write!(f, "{}", e),
            Value::Money(symbol, amount) => write!(f, "{}{}", symbol, amount),
            Value::Hex(hex) => write!(f, "{}", hex),
            Value::Date(date) => write!(f, "{}", date),
            Value::Time(time) => write!(f, "{}", time),
            Value::DateTime(datetime) => write!(f, "{}", datetime),
            Value::Tuple(tuple) => {
                write!(f, "(")?;
                for (i, item) in tuple.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", item)?;
                }
                write!(f, ")")
            }
            Value::Range(start, end, step) => write!(f, "{}..{} by {}", start, end, step),
            Value::Task(id) => write!(f, "<task {}>", id),
            Value::Channel(id) => write!(f, "<channel {}>", id),
            Value::Vector(vec) => {
                write!(f, "vec[")?;
                for (i, item) in vec.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            Value::Matrix(mat) => {
                write!(f, "mat[")?;
                for (i, row) in mat.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "[")?;
                    for (j, item) in row.iter().enumerate() {
                        if j > 0 { write!(f, ", ")?; }
                        write!(f, "{}", item)?;
                    }
                    write!(f, "]")?;
                }
                write!(f, "]")
            }
            Value::Grid(grid) => {
                write!(f, "grid[")?;
                for (i, row) in grid.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "[")?;
                    for (j, item) in row.iter().enumerate() {
                        if j > 0 { write!(f, ", ")?; }
                        write!(f, "{}", item)?;
                    }
                    write!(f, "]")?;
                }
                write!(f, "]")
            }
            Value::Deque(deque) => {
                write!(f, "deque[")?;
                for (i, item) in deque.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            Value::PriorityQ(pq) => {
                write!(f, "priorityq[")?;
                for (i, (priority, value)) in pq.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "({}, {})", priority, value)?;
                }
                write!(f, "]")
            }
            Value::Graph(graph) => {
                write!(f, "graph{{")?;
                for (i, (node, edges)) in graph.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}: [", node)?;
                    for (j, (neighbor, weight)) in edges.iter().enumerate() {
                        if j > 0 { write!(f, ", ")?; }
                        write!(f, "({}, {})", neighbor, weight)?;
                    }
                    write!(f, "]")?;
                }
                write!(f, "}}")
            }
            Value::Tree { value, children } => {
                write!(f, "tree{{value: {}, children: [", value)?;
                for (i, child) in children.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", child)?;
                }
                write!(f, "]}}")
            }
            Value::None => write!(f, "none"),
        }
    }
}

pub struct Interpreter {
    globals: HashMap<String, Value>,
    locals: Vec<HashMap<String, Value>>,
    statics: HashMap<String, Value>, // Static variables
    call_depth: usize, // Track recursion depth
    defer_stack: Vec<Vec<AstNode>>, // Defer statements to run when block exits (LIFO)
}

impl Interpreter {
    pub fn new() -> Self {
        let mut interpreter = Self {
            globals: HashMap::new(),
            locals: Vec::new(),
            statics: HashMap::new(),
            call_depth: 0,
            defer_stack: Vec::new(),
        };
        
        // Add built-in functions
        interpreter.add_builtins();
        interpreter
    }
    
    fn add_builtins(&mut self) {
        // Built-in functions will be added here
        // For now, we'll handle them specially in function calls
    }
    
    fn apply_decorator(&mut self, decorator_name: &str, decorator_args: &[crate::parser::AstNode], func: Value) -> Result<Value, String> {
        // Evaluate decorator arguments
        let mut args = Vec::new();
        for arg in decorator_args {
            args.push(self.eval_node(&Box::new(arg.clone()))?);
        }
        
        // Check if decorator is a built-in or user-defined function
        let decorator_func = self.globals.get(decorator_name).cloned();
        if let Some(Value::Function { params, body, .. }) = decorator_func {
            // Call decorator with function as argument
            let mut call_args = vec![func];
            call_args.extend(args);
            return self.call_function_internal(decorator_name, &call_args, &params, &*body, None);
        }
        
        // Built-in decorators
        match decorator_name {
            "memo" | "cache" => {
                // Memoization decorator - wrap function with caching
                Ok(self.create_memoized_function(func)?)
            }
            "timer" => {
                // Timer decorator - measure execution time
                Ok(self.create_timed_function(func)?)
            }
            "log_call" => {
                // Log calls decorator
                Ok(self.create_logged_function(func)?)
            }
            _ => {
                // Try to find user-defined decorator
                Err(format!("Decorator '{}' not found", decorator_name))
            }
        }
    }
    
    fn create_memoized_function(&mut self, func: Value) -> Result<Value, String> {
        // For now, return the function as-is
        // Full memoization would require a cache map
        Ok(func)
    }
    
    fn create_timed_function(&mut self, func: Value) -> Result<Value, String> {
        // For now, return the function as-is
        // Full timing would require wrapping the call
        Ok(func)
    }
    
    fn create_logged_function(&mut self, func: Value) -> Result<Value, String> {
        // For now, return the function as-is
        // Full logging would require wrapping the call
        Ok(func)
    }
    
    fn call_function_internal(&mut self, _func_name: &str, args: &[Value], params: &[String], body: &AstNode, this_value: Option<Value>) -> Result<Value, String> {
        // Check depth BEFORE incrementing to prevent stack overflow
        if self.call_depth >= 50 {
            return Err(JError::stack_overflow(self.call_depth).to_string());
        }
        
        self.call_depth += 1;
        
        // Push new scope
        self.locals.push(HashMap::new());
        
        // Bind this/self if method call
        if let Some(ref this_val) = this_value {
            self.locals.last_mut().unwrap().insert("this".to_string(), this_val.clone());
            self.locals.last_mut().unwrap().insert("self".to_string(), this_val.clone());
        }
        
        // Bind parameters
        for (i, param) in params.iter().enumerate() {
            if i < args.len() {
                self.locals.last_mut().unwrap().insert(param.clone(), args[i].clone());
            }
        }
        
        // Execute body
        let result = self.eval_node(body);
        
        // Pop scope and decrement call depth
        self.locals.pop();
        self.call_depth -= 1;
        
        result
    }
    
    pub fn run(&mut self, source: &str) -> Result<(), String> {
        // Tokenize
        let mut lexer = crate::lexer::Lexer::new(source);
        let tokens = lexer.tokenize()
            .map_err(|e| format!("Lexer error: {}", e))?;
        
        // Parse
        let mut parser = crate::parser::Parser::new(tokens);
        let ast = parser.parse()
            .map_err(|e| format!("Parser error: {}", e))?;
        
        // Interpret
        self.eval_node(&ast)
            .map_err(|e| format!("Runtime error: {}", e))?;
        
        Ok(())
    }
    
    pub fn evaluate(&mut self, node: &AstNode) -> Result<String, String> {
        let value = self.eval_node(node)?;
        match value {
            Value::None => Ok(String::new()),
            _ => Ok(value.to_string()),
        }
    }
    
    fn eval_node(&mut self, node: &AstNode) -> Result<Value, String> {
        match node {
            AstNode::Integer(i) => Ok(Value::Integer(*i)),
            AstNode::Float(f) => Ok(Value::Float(*f)),
            AstNode::String(s) => Ok(Value::String(s.clone())),
            AstNode::StringInterpolation { parts } => {
                let mut result = String::new();
                for part in parts {
                    let val = self.eval_node(part)?;
                    result.push_str(&val.to_string());
                }
                Ok(Value::String(result))
            }
            AstNode::Boolean(b) => Ok(Value::Boolean(*b)),
            AstNode::Char(c) => Ok(Value::Char(*c)),
            
            AstNode::Infinity(positive) => Ok(Value::Infinity(*positive)),
            AstNode::Emoji(e) => Ok(Value::Emoji(e.clone())),
            AstNode::Money(symbol, amount) => Ok(Value::Money(symbol.clone(), *amount)),
            AstNode::Hex(hex) => Ok(Value::Hex(hex.clone())),
            AstNode::Date(date) => Ok(Value::Date(date.clone())),
            AstNode::Time(time) => Ok(Value::Time(time.clone())),
            AstNode::DateTime(datetime) => Ok(Value::DateTime(datetime.clone())),
            
            AstNode::Tuple(elements) => {
                let mut tuple = Vec::new();
                for element in elements {
                    tuple.push(self.eval_node(element)?);
                }
                Ok(Value::Tuple(tuple))
            }
            
            AstNode::List(elements) => {
                let mut list = Vec::new();
                for element in elements {
                    list.push(self.eval_node(element)?);
                }
                Ok(Value::List(list))
            }
            
            AstNode::Vector(elements) => {
                let mut vector = Vec::new();
                for element in elements {
                    let val = self.eval_node(element)?;
                    match val {
                        Value::Integer(i) => vector.push(i as f64),
                        Value::Float(f) => vector.push(f),
                        _ => return Err("Vector elements must be numeric".to_string()),
                    }
                }
                Ok(Value::Vector(vector))
            }
            
            AstNode::Matrix(rows) => {
                let mut matrix = Vec::new();
                for row in rows {
                    let mut matrix_row = Vec::new();
                    for element in row {
                        let val = self.eval_node(element)?;
                        match val {
                            Value::Integer(i) => matrix_row.push(i as f64),
                            Value::Float(f) => matrix_row.push(f),
                            _ => return Err("Matrix elements must be numeric".to_string()),
                        }
                    }
                    matrix.push(matrix_row);
                }
                Ok(Value::Matrix(matrix))
            }
            
            AstNode::Dict(pairs) => {
                let mut dict = HashMap::new();
                for (key_node, value_node) in pairs {
                    let key = match self.eval_node(key_node)? {
                        Value::String(s) => s,
                        Value::Integer(i) => i.to_string(),
                        _ => return Err("Dictionary keys must be strings or integers".to_string()),
                    };
                    let value = self.eval_node(value_node)?;
                    dict.insert(key, value);
                }
                Ok(Value::Dict(dict))
            }
            
            AstNode::Identifier(name) => {
                self.get_variable(name)
            }
            
            AstNode::VarDeclaration { var_type, name, value, immutable: _, is_static } => {
                let val = self.eval_node(value)?;
                
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
                                        _ => return Err("Vector elements must be numeric".to_string()),
                                    }
                                }
                                Value::Vector(vector)
                            }
                            _ => return Err("Vector must be initialized with a list or vector".to_string()),
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
                                                    _ => return Err("Matrix elements must be numeric".to_string()),
                                                }
                                            }
                                            matrix.push(matrix_row);
                                        }
                                        _ => return Err("Matrix rows must be lists".to_string()),
                                    }
                                }
                                Value::Matrix(matrix)
                            }
                            _ => return Err("Matrix must be initialized with a list of lists or matrix".to_string()),
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
                                        _ => return Err("Set elements must be convertible to strings".to_string()),
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
                                    let key = match item {
                                        Value::String(s) => s,
                                        Value::Integer(i) => i.to_string(),
                                        Value::Float(f) => f.to_string(),
                                        Value::Boolean(b) => b.to_string(),
                                        _ => return Err("Counter elements must be convertible to strings".to_string()),
                                    };
                                    *counter.entry(key).or_insert(0) += 1;
                                }
                                Value::Counter(counter)
                            }
                            _ => return Err("Counter must be initialized with a string or list".to_string()),
                        }
                    }
                    "deque" => {
                        match val {
                            Value::Deque(_) => val,
                            Value::List(list) => Value::Deque(list),
                            _ => return Err("Deque must be initialized with a list".to_string()),
                        }
                    }
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
                            _ => return Err("Priority queue must be initialized with a list of tuples".to_string()),
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
                                        _ => return Err("Graph node edges must be a list".to_string()),
                                    }
                                }
                                Value::Graph(graph)
                            }
                            _ => return Err("Graph must be initialized with a dictionary".to_string()),
                        }
                    }
                    "grid" => {
                        match val {
                            Value::Grid(_) => val,
                            Value::Matrix(rows) => {
                                let grid: Vec<Vec<Value>> = rows
                                    .into_iter()
                                    .map(|row| row.into_iter().map(|f| Value::Float(f)).collect())
                                    .collect();
                                Value::Grid(grid)
                            }
                            Value::List(rows) => {
                                let mut grid = Vec::new();
                                for row in rows {
                                    match row {
                                        Value::List(cells) => grid.push(cells),
                                        _ => return Err("Grid must be a list of lists (2D)".to_string()),
                                    }
                                }
                                Value::Grid(grid)
                            }
                            _ => return Err("Grid must be initialized with a list of lists or matrix literal".to_string()),
                        }
                    }
                    "tree" => {
                        match val {
                            Value::Tree { .. } => val,
                            Value::Dict(dict) => {
                                // Convert dict to tree: {value: ..., children: [...]}
                                let value = dict.get("value")
                                    .ok_or_else(|| "Tree must have 'value' field".to_string())?
                                    .clone();
                                let children = match dict.get("children") {
                                    Some(Value::List(children_list)) => children_list.clone(),
                                    Some(_) => return Err("Tree 'children' must be a list".to_string()),
                                    None => Vec::new(),
                                };
                                Value::Tree {
                                    value: Box::new(value),
                                    children,
                                }
                            }
                            _ => return Err("Tree must be initialized with a dictionary".to_string()),
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
            
            AstNode::EnumDeclaration { name, backing_type: _, variants } => {
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
            },

            AstNode::ClassDeclaration { name, parent, traits: _, fields, methods, static_fields, static_methods } => {
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
                    if let AstNode::FunctionDeclaration { name: method_name, params, body, .. } = method {
                        let param_names: Vec<String> = params.iter().map(|(_, n)| n.clone()).collect();
                        let func = Value::Function {
                            name: method_name.clone(),
                            params: param_names,
                            body: body.clone(),
                        };
                        class_methods.insert(method_name.clone(), func);
                    }
                }
                
                for method in static_methods {
                    if let AstNode::FunctionDeclaration { name: method_name, params, body, .. } = method {
                        let param_names: Vec<String> = params.iter().map(|(_, n)| n.clone()).collect();
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
                    parent: parent.clone(),
                    fields: class_fields,
                    methods: class_methods,
                    static_fields: class_static_fields,
                    static_methods: class_static_methods,
                };
                
                self.set_variable(name.clone(), class_val.clone());
                Ok(class_val)
            }

            AstNode::FunctionDeclaration { name, params, body, decorators, .. } => {
                let param_names: Vec<String> = params.iter().map(|(_, name)| name.clone()).collect();
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
            
            AstNode::FunctionCall { name, args } => {
                self.call_function(name, args)
            }

            AstNode::Call { callee, args } => {
                let (callee_val, this_opt) = if let AstNode::DotAccess { object, field } = callee.as_ref() {
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
            
            AstNode::Binary { left, operator, right } => {
                let left_val = self.eval_node(left)?;
                let right_val = self.eval_node(right)?;
                self.eval_binary_op(&left_val, operator, &right_val)
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
                    Value::Enum { name: enum_name, variants } => {
                        if let Some(val) = variants.get(field) {
                             Ok(Value::EnumVariant {
                                enum_name: enum_name.clone(),
                                variant_name: field.clone(),
                                value: Box::new(val.clone())
                            })
                        } else {
                            match field.as_str() {
                                "count" => Ok(Value::Integer(variants.len() as i64)),
                                "names" => {
                                    let names: Vec<Value> = variants.keys().map(|k| Value::String(k.clone())).collect();
                                    Ok(Value::List(names))
                                }
                                "values" => {
                                    let vals: Vec<Value> = variants.values().cloned().collect();
                                    Ok(Value::List(vals))
                                }
                                _ => Err(format!("Unknown variant or method '{}' on enum '{}'", field, enum_name))
                            }
                        }
                    }
                    Value::EnumVariant { enum_name: _, variant_name, value } => {
                        match field.as_str() {
                            "label" | "name" => Ok(Value::String(variant_name)),
                            "value" => Ok(*value),
                            _ => Err(format!("Unknown property '{}' on enum variant", field))
                        }
                    }
                    Value::Class { name: class_name, parent: _, fields: _, methods: _, static_fields, static_methods } => {
                        if field == "new" {
                            Ok(Value::Constructor(class_name.clone()))
                        } else if let Some(v) = static_fields.get(field) {
                            Ok(v.clone())
                        } else if let Some(v) = static_methods.get(field) {
                            Ok(v.clone())
                        } else {
                            Err(format!("Unknown static field or method '{}' on class '{}'", field, class_name))
                        }
                    }
                    Value::Instance { class_name, fields } => {
                        if let Some(v) = fields.get(field) {
                            Ok(v.clone())
                        } else {
                            self.get_instance_method(class_name.as_str(), field)
                        }
                    }
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
                                    let keys: Vec<Value> = dict.keys().map(|k| Value::String(k.clone())).collect();
                                    Ok(Value::List(keys))
                                }
                                "values" => {
                                    let values: Vec<Value> = dict.values().cloned().collect();
                                    Ok(Value::List(values))
                                }
                                "has" => {
                                    return Err("dict.has() requires a key argument - use dict.has(key)".to_string());
                                }
                                "get" => {
                                    return Err("dict.get() requires key and optional default arguments".to_string());
                                }
                                "remove" => {
                                    return Err("dict.remove() requires a key argument - use dict.remove(key)".to_string());
                                }
                                "merge" => {
                                    return Err("dict.merge() requires another dict argument".to_string());
                                }
                                "update" => {
                                    return Err("dict.update() requires another dict argument".to_string());
                                }
                                "clear" => {
                                    return Err("dict.clear() cannot be called as a method - use clear(dict)".to_string());
                                }
                                "size" | "len" | "length" => {
                                    Ok(Value::Integer(dict.len() as i64))
                                }
                                _ => Err(format!("Dictionary field '{}' not found", field))
                            }
                        }
                    }
                    Value::List(list) => {
                        // List methods
                        match field.as_str() {
                            "len" | "length" | "size" => Ok(Value::Integer(list.len() as i64)),
                            "first" => list.first().cloned().ok_or_else(|| "List is empty".to_string()),
                            "last" => list.last().cloned().ok_or_else(|| "List is empty".to_string()),
                            "sum" => {
                                let mut sum = 0.0;
                                for item in list.iter() {
                                    match item {
                                        Value::Integer(i) => sum += *i as f64,
                                        Value::Float(f) => sum += f,
                                        _ => return Err("sum() requires numeric values".to_string()),
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
                                        _ => return Err("product() requires numeric values".to_string()),
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
                                        (Value::Integer(a), Value::Integer(b)) if b < a => min_val = item.clone(),
                                        (Value::Float(a), Value::Float(b)) if b < a => min_val = item.clone(),
                                        (Value::Integer(a), Value::Float(b)) if (*b as i64) < *a => min_val = item.clone(),
                                        (Value::Float(a), Value::Integer(b)) if (*b as f64) < *a => min_val = item.clone(),
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
                                        (Value::Integer(a), Value::Integer(b)) if b > a => max_val = item.clone(),
                                        (Value::Float(a), Value::Float(b)) if b > a => max_val = item.clone(),
                                        (Value::Integer(a), Value::Float(b)) if (*b as i64) > *a => max_val = item.clone(),
                                        (Value::Float(a), Value::Integer(b)) if (*b as f64) > *a => max_val = item.clone(),
                                        _ => {}
                                    }
                                }
                                Ok(max_val)
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
                                        _ => return Err("mean() requires numeric values".to_string()),
                                    }
                                }
                                Ok(Value::Float(sum / list.len() as f64))
                            }
                            "median" => {
                                if list.is_empty() {
                                    return Err("median() requires non-empty list".to_string());
                                }
                                let mut sorted = list.clone();
                                sorted.sort_by(|a, b| {
                                    match (a, b) {
                                        (Value::Integer(x), Value::Integer(y)) => x.cmp(y),
                                        (Value::Float(x), Value::Float(y)) => {
                                            x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal)
                                        }
                                        (Value::Integer(x), Value::Float(y)) => {
                                            (*x as f64).partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal)
                                        }
                                        (Value::Float(x), Value::Integer(y)) => {
                                            x.partial_cmp(&(*y as f64)).unwrap_or(std::cmp::Ordering::Equal)
                                        }
                                        _ => std::cmp::Ordering::Equal
                                    }
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
                                        _ => Ok(sorted[mid - 1].clone())
                                    }
                                } else {
                                    Ok(sorted[mid].clone())
                                }
                            }
                            "sorted" => {
                                let mut sorted = list.clone();
                                sorted.sort_by(|a, b| {
                                    match (a, b) {
                                        (Value::Integer(x), Value::Integer(y)) => x.cmp(y),
                                        (Value::Float(x), Value::Float(y)) => {
                                            x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal)
                                        }
                                        (Value::Integer(x), Value::Float(y)) => {
                                            (*x as f64).partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal)
                                        }
                                        (Value::Float(x), Value::Integer(y)) => {
                                            x.partial_cmp(&(*y as f64)).unwrap_or(std::cmp::Ordering::Equal)
                                        }
                                        (Value::String(x), Value::String(y)) => x.cmp(y),
                                        _ => std::cmp::Ordering::Equal
                                    }
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
                            "empty" | "is_empty" => {
                                Ok(Value::Boolean(list.is_empty()))
                            }
                            "any" => {
                                return Err("list.any() requires a predicate function - use any(list, fn)".to_string());
                            }
                            "all" => {
                                return Err("list.all() requires a predicate function - use all(list, fn)".to_string());
                            }
                            "count" => {
                                return Err("list.count() requires an argument - use list.count(value)".to_string());
                            }
                            "contains" => {
                                return Err("list.contains() requires an argument - use list.contains(value)".to_string());
                            }
                            "index" | "find" => {
                                return Err("list.index() requires an argument - use list.index(value)".to_string());
                            }
                            "join" => {
                                return Err("list.join() requires a separator argument - use list.join(sep)".to_string());
                            }
                            _ => Err(format!("List method '{}' not found", field))
                        }
                    }
                    Value::String(s) => {
                        // String methods
                        match field.as_str() {
                            "len" | "length" | "size" => Ok(Value::Integer(s.len() as i64)),
                            "first" => s.chars().next().map(|c| Value::Char(c)).ok_or_else(|| "String is empty".to_string()),
                            "last" => s.chars().last().map(|c| Value::Char(c)).ok_or_else(|| "String is empty".to_string()),
                            "empty" | "is_empty" => Ok(Value::Boolean(s.is_empty())),
                            "upper" | "uppercase" => Ok(Value::String(s.to_uppercase())),
                            "lower" | "lowercase" => Ok(Value::String(s.to_lowercase())),
                            "trim" => Ok(Value::String(s.trim().to_string())),
                            "trim_start" | "trim_left" => Ok(Value::String(s.trim_start().to_string())),
                            "trim_end" | "trim_right" => Ok(Value::String(s.trim_end().to_string())),
                            "lines" => {
                                let lines: Vec<Value> = s.lines().map(|l| Value::String(l.to_string())).collect();
                                Ok(Value::List(lines))
                            }
                            "chars" => {
                                let chars: Vec<Value> = s.chars().map(|c| Value::Char(c)).collect();
                                Ok(Value::List(chars))
                            }
                            "split" => {
                                return Err("string.split() requires a separator argument - use string.split(sep)".to_string());
                            }
                            "contains" => {
                                return Err("string.contains() requires a substring argument - use string.contains(substr)".to_string());
                            }
                            "starts_with" | "startswith" => {
                                return Err("string.starts_with() requires a prefix argument".to_string());
                            }
                            "ends_with" | "endswith" => {
                                return Err("string.ends_with() requires a suffix argument".to_string());
                            }
                            "replace" => {
                                return Err("string.replace() requires old and new arguments - use string.replace(old, new)".to_string());
                            }
                            _ => Err(format!("String method '{}' not found", field))
                        }
                    }
                    Value::Counter(counter) => {
                        match field.as_str() {
                            "most_common" => {
                                let mut items: Vec<_> = counter.iter().collect();
                                items.sort_by(|a, b| b.1.cmp(a.1));
                                let result: Vec<Value> = items.iter()
                                    .map(|(k, v)| Value::Tuple(vec![Value::String(k.to_string()), Value::Integer(**v)]))
                                    .collect();
                                Ok(Value::List(result))
                            }
                            "elements" | "keys" => {
                                let keys: Vec<Value> = counter.keys().map(|k| Value::String(k.clone())).collect();
                                Ok(Value::List(keys))
                            }
                            "total" => {
                                let total: i64 = counter.values().sum();
                                Ok(Value::Integer(total))
                            }
                            "len" | "length" | "size" => Ok(Value::Integer(counter.len() as i64)),
                            _ => Err(format!("Counter method '{}' not found", field)),
                        }
                    }
                    Value::Grid(grid) => {
                        let rows = grid.len() as i64;
                        let cols = if grid.is_empty() { 0 } else { grid[0].len() as i64 };
                        match field.as_str() {
                            "rows" => Ok(Value::Integer(rows)),
                            "cols" | "columns" => Ok(Value::Integer(cols)),
                            "len" | "length" | "size" => Ok(Value::Integer(rows * cols)),
                            "neighbors" => Ok(Value::GridNeighbors(Box::new(Value::Grid(grid.clone())))),
                            _ => Err(format!("Grid method '{}' not found", field)),
                        }
                    }
                    _ => Err(format!("Cannot access field '{}' on type {}", field, 
                        match obj_val {
                            Value::Integer(_) => "integer",
                            Value::Float(_) => "float",
                            Value::Boolean(_) => "boolean",
                            Value::Char(_) => "char",
                            Value::Function { .. } => "function",
                            _ => "unknown",
                        }))
                }
            }
            
            AstNode::Index { object, index } => {
                let obj_val = self.eval_node(object)?;
                let idx_val = self.eval_node(index)?;
                
                match (obj_val, idx_val) {
                    (Value::Enum { name: enum_name, variants }, val) => {
                         // Reverse lookup: directions[1]
                         for (key, variant_val) in variants.iter() {
                             if variant_val == &val {
                                  return Ok(Value::EnumVariant {
                                      enum_name: enum_name.clone(),
                                      variant_name: key.clone(),
                                      value: Box::new(variant_val.clone())
                                  });
                             }
                         }
                         Err(format!("Value '{}' not found in enum '{}'", val, enum_name))
                    }
                    (Value::List(list), Value::Integer(i)) => {
                        let idx = if i < 0 {
                            // Negative indexing from the end
                            (list.len() as i64 + i) as usize
                        } else {
                            i as usize
                        };
                        
                        if idx < list.len() {
                            Ok(list[idx].clone())
                        } else {
                            Err(JError::index_out_of_bounds(i, list.len(), 0, 0).to_string())
                        }
                    }
                    (Value::Grid(grid), Value::Integer(i)) => {
                        let idx = if i < 0 {
                            (grid.len() as i64 + i) as usize
                        } else {
                            i as usize
                        };
                        if idx < grid.len() {
                            Ok(Value::List(grid[idx].clone()))
                        } else {
                            Err(format!("Grid row index {} out of bounds (rows {})", i, grid.len()))
                        }
                    }
                    (Value::String(s), Value::Integer(i)) => {
                        let chars: Vec<char> = s.chars().collect();
                        let idx = if i < 0 {
                            // Negative indexing from the end
                            (chars.len() as i64 + i) as usize
                        } else {
                            i as usize
                        };
                        
                        if idx < chars.len() {
                            Ok(Value::Char(chars[idx]))
                        } else {
                            Err(format!("String index {} out of bounds (length {})", i, chars.len()))
                        }
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
                        let idx = if i < 0 {
                            // Negative indexing from the end
                            (tuple.len() as i64 + i) as usize
                        } else {
                            i as usize
                        };
                        
                        if idx < tuple.len() {
                            Ok(tuple[idx].clone())
                        } else {
                            Err(format!("Tuple index {} out of bounds (length {})", i, tuple.len()))
                        }
                    }
                    (obj, idx) => Err(format!("Cannot index {} with {}", 
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
                    ))
                }
            }
            
            AstNode::Slice { object, start, end, step } => {
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
                        let (start, end) = self.normalize_slice_indices(start_idx, end_idx, len, step_val)?;
                        
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
                        let (start, end) = self.normalize_slice_indices(start_idx, end_idx, len, step_val)?;
                        
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
                        let (start, end) = self.normalize_slice_indices(start_idx, end_idx, len, step_val)?;
                        
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
            
            AstNode::If { condition, then_branch, else_branch } => {
                let cond_val = self.eval_node(condition)?;
                if self.is_truthy(&cond_val) {
                    self.eval_node(then_branch)
                } else if let Some(else_node) = else_branch {
                    self.eval_node(else_node)
                } else {
                    Ok(Value::None)
                }
            }
            
            AstNode::Match { expr, arms } => {
                let expr_val = self.eval_node(expr)?;
                
                for arm in arms {
                    if self.pattern_matches(&arm.pattern, &expr_val)? {
                        if let Some(guard) = &arm.guard {
                            let guard_val = self.eval_node(guard)?;
                            if !self.is_truthy(&guard_val) {
                                continue;
                            }
                        }
                        return self.eval_node(&arm.body);
                    }
                }
                
                Err("No matching pattern found".to_string())
            }
            
            AstNode::While { condition, body } => {
                let mut last_val = Value::None;
                
                loop {
                    let cond_result = self.eval_node(condition)?;
                    if !self.is_truthy(&cond_result) {
                        break;
                    }
                    last_val = self.eval_node(body)?;
                }
                
                Ok(last_val)
            }
            
            AstNode::TryCatch { try_block, catch_var, catch_block, finally_block } => {
                // Execute try block
                let result = self.eval_node(try_block);
                
                // If error occurred, execute catch block
                let final_result = match result {
                    Err(error_msg) => {
                        // Set error variable if specified
                        if let Some(var_name) = catch_var {
                            self.set_variable(var_name.clone(), Value::String(error_msg));
                        }
                        
                        // Execute catch block
                        self.eval_node(catch_block)?
                    }
                    Ok(val) => val,
                };
                
                // Execute finally block if present
                if let Some(finally) = finally_block {
                    self.eval_node(finally)?;
                }
                
                Ok(final_result)
            }
            
            AstNode::Throw(expr) => {
                let error_val = self.eval_node(expr)?;
                let error_msg = match error_val {
                    Value::String(s) => s,
                    other => format!("{}", other),
                };
                Err(error_msg)
            }
            
            AstNode::For { var, iterable, body } => {
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
                    Value::Dict(dict) => {
                        for (key, _value) in dict {
                            // For simple iteration, iterate over keys
                            self.set_variable(var.clone(), Value::String(key));
                            last_val = self.eval_node(body)?;
                        }
                    }
                    Value::Tuple(tuple) => {
                        for item in tuple {
                            self.set_variable(var.clone(), item);
                            last_val = self.eval_node(body)?;
                        }
                    }
                    Value::Vector(vec) => {
                        for item in vec {
                            self.set_variable(var.clone(), Value::Float(item));
                            last_val = self.eval_node(body)?;
                        }
                    }
                    _ => return Err("Can only iterate over lists, strings, dictionaries, tuples, and vectors".to_string()),
                }
                
                Ok(last_val)
            }
            
            AstNode::ForReverse { var, iterable, body } => {
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
                    _ => return Err("Can only reverse iterate over lists, strings, and vectors".to_string()),
                }
                
                Ok(last_val)
            }
            
            AstNode::ForStep { var, start, step, condition, body } => {
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
            
            AstNode::ForIndexed { index_var, value_var, iterable, body } => {
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
                    _ => return Err("Can only iterate over lists, strings, dictionaries, tuples, and vectors".to_string()),
                }
                
                Ok(last_val)
            }
            
            AstNode::ForZip { vars, iterables, body } => {
                if vars.len() != iterables.len() {
                    return Err("Number of variables must match number of iterables in zip".to_string());
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
                            let chars: Vec<Value> = s.chars().map(|c| Value::Char(c)).collect();
                            lists.push(chars);
                        }
                        Value::Vector(vec) => {
                            let floats: Vec<Value> = vec.iter().map(|&f| Value::Float(f)).collect();
                            lists.push(floats);
                        }
                        Value::Tuple(tuple) => lists.push(tuple),
                        _ => return Err("Can only zip over lists, strings, vectors, and tuples".to_string()),
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
            
            AstNode::ForParallel { var, iterable, body, workers: _, ordered: _ } => {
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
                    _ => return Err("Can only parallel iterate over lists, strings, and vectors".to_string()),
                }
                
                Ok(last_val)
            }
            
            // Enhanced for loop variants
            AstNode::ForChunked { var, iterable, chunk_size, body } => {
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
                            let chunk_chars: Vec<Value> = chunk.iter().map(|&c| Value::Char(c)).collect();
                            self.set_variable(var.clone(), Value::List(chunk_chars));
                            last_val = self.eval_node(body)?;
                        }
                    }
                    _ => return Err("ForChunked can only iterate over lists and strings".to_string()),
                }
                
                Ok(last_val)
            }
            
            AstNode::ForFiltered { var, iterable, filter, body } => {
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
                    _ => return Err("ForFiltered can only iterate over lists and strings".to_string()),
                }
                
                Ok(last_val)
            }
            
            AstNode::ForWindowed { var, iterable, window_size, body } => {
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
                                let window: Vec<Value> = chars[i..i + window_size].iter().map(|&c| Value::Char(c)).collect();
                                self.set_variable(var.clone(), Value::List(window));
                                last_val = self.eval_node(body)?;
                            }
                        }
                    }
                    _ => return Err("ForWindowed can only iterate over lists and strings".to_string()),
                }
                
                Ok(last_val)
            }
            
            AstNode::Range { start, end, inclusive, step } => {
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
            
            AstNode::Block(statements) => {
                self.defer_stack.push(Vec::new());
                let mut last_val = Value::None;

                for stmt in statements {
                    if let AstNode::Defer(expr) = stmt {
                        self.defer_stack.last_mut().unwrap().push(*expr.clone());
                    } else {
                        last_val = self.eval_node(stmt)?;
                    }
                }

                let defers = self.defer_stack.pop().unwrap();
                for expr in defers.into_iter().rev() {
                    self.eval_node(&expr)?;
                }
                Ok(last_val)
            }

            AstNode::Defer(_) => {
                Ok(Value::None)
            }

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
            
            AstNode::Expression(expr) => {
                self.eval_node(expr)
            }
            
            AstNode::Assignment { name, value } => {
                let val = self.eval_node(value)?;
                
                // Check if it's a static variable first
                if self.statics.contains_key(name) {
                    self.statics.insert(name.clone(), val.clone());
                } else {
                    self.set_variable(name.clone(), val.clone());
                }
                
                Ok(val)
            }
            
            AstNode::DestructuringAssignment { targets, value } => {
                let val = self.eval_node(value)?;
                
                match val {
                    Value::Tuple(tuple) => {
                        if targets.len() != tuple.len() {
                            return Err(format!("Cannot destructure tuple of length {} into {} variables", 
                                tuple.len(), targets.len()));
                        }
                        
                        for (target, value) in targets.iter().zip(tuple.iter()) {
                            self.set_variable(target.clone(), value.clone());
                        }
                        
                        Ok(Value::Tuple(tuple))
                    }
                    Value::List(list) => {
                        if targets.len() > list.len() {
                            return Err(format!("Cannot destructure list of length {} into {} variables", 
                                list.len(), targets.len()));
                        }
                        
                        for (i, target) in targets.iter().enumerate() {
                            if i < list.len() {
                                self.set_variable(target.clone(), list[i].clone());
                            } else {
                                self.set_variable(target.clone(), Value::None);
                            }
                        }
                        
                        Ok(Value::List(list))
                    }
                    _ => Err("Can only destructure tuples and lists".to_string()),
                }
            }
            
            AstNode::Return(expr) => {
                if let Some(expr) = expr {
                    self.eval_node(expr)
                } else {
                    Ok(Value::None)
                }
            }
            
            AstNode::TryExpression(expr) => {
                // For now, just evaluate the expression
                // In a full implementation, this would handle Result types
                self.eval_node(expr)
            }
            
            AstNode::ExecuteFile { filename } => {
                // Execute another J file
                self.execute_file(filename)
            }
            
            AstNode::Lambda { params, body } => {
                Ok(Value::Function {
                    name: "<lambda>".to_string(),
                    params: params.clone(),
                    body: body.clone(),
                })
            }
            
            AstNode::ListComprehension { expr, var, iterable, condition } => {
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
                    _ => return Err("List comprehension can only iterate over lists, strings, and vectors".to_string()),
                }
                
                Ok(Value::List(result))
            }
            
            AstNode::DictComprehension { key_expr, value_expr, var, iterable, condition } => {
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
                            
                            let key = match key_result {
                                Value::String(s) => s,
                                Value::Integer(i) => i.to_string(),
                                _ => return Err("Dictionary comprehension keys must be strings or integers".to_string()),
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
                            
                            let key = match key_result {
                                Value::String(s) => s,
                                Value::Integer(i) => i.to_string(),
                                _ => return Err("Dictionary comprehension keys must be strings or integers".to_string()),
                            };
                            
                            result.insert(key, value_result);
                        }
                    }
                    _ => return Err("Dictionary comprehension can only iterate over lists and strings".to_string()),
                }
                
                Ok(Value::Dict(result))
            }
            
            // Missing AST node patterns
            AstNode::TypeConversion { target_type, name } => {
                // Type conversion: type*variable
                let current_value = self.get_variable(name)?;
                
                let converted_value = match target_type.as_str() {
                    "str" => {
                        match current_value {
                            Value::Integer(i) => Value::String(i.to_string()),
                            Value::Float(f) => Value::String(f.to_string()),
                            Value::Boolean(b) => Value::String(b.to_string()),
                            Value::Char(c) => Value::String(c.to_string()),
                            Value::String(s) => Value::String(s),
                            _ => Value::String(current_value.to_string()),
                        }
                    }
                    "int" => {
                        match current_value {
                            Value::Integer(i) => Value::Integer(i),
                            Value::Float(f) => Value::Integer(f as i64),
                            Value::String(s) => {
                                s.parse::<i64>()
                                    .map(Value::Integer)
                                    .unwrap_or(Value::Integer(0))
                            }
                            Value::Boolean(b) => Value::Integer(if b { 1 } else { 0 }),
                            _ => Value::Integer(0),
                        }
                    }
                    "float" => {
                        match current_value {
                            Value::Integer(i) => Value::Float(i as f64),
                            Value::Float(f) => Value::Float(f),
                            Value::String(s) => {
                                s.parse::<f64>()
                                    .map(Value::Float)
                                    .unwrap_or(Value::Float(0.0))
                            }
                            Value::Boolean(b) => Value::Float(if b { 1.0 } else { 0.0 }),
                            _ => Value::Float(0.0),
                        }
                    }
                    "bool" => {
                        match current_value {
                            Value::Boolean(b) => Value::Boolean(b),
                            Value::Integer(i) => Value::Boolean(i != 0),
                            Value::Float(f) => Value::Boolean(f != 0.0),
                            Value::String(s) => Value::Boolean(!s.is_empty()),
                            Value::List(l) => Value::Boolean(!l.is_empty()),
                            _ => Value::Boolean(true),
                        }
                    }
                    "list" => {
                        match current_value {
                            Value::List(l) => Value::List(l),
                            Value::String(s) => {
                                let chars: Vec<Value> = s.chars().map(|c| Value::Char(c)).collect();
                                Value::List(chars)
                            }
                            Value::Vector(v) => {
                                let floats: Vec<Value> = v.iter().map(|&f| Value::Float(f)).collect();
                                Value::List(floats)
                            }
                            Value::Tuple(t) => Value::List(t),
                            _ => Value::List(vec![current_value]),
                        }
                    }
                    "vec" => {
                        match current_value {
                            Value::Vector(v) => Value::Vector(v),
                            Value::List(l) => {
                                let mut vec = Vec::new();
                                for item in l {
                                    match item {
                                        Value::Integer(i) => vec.push(i as f64),
                                        Value::Float(f) => vec.push(f),
                                        _ => return Err("Cannot convert non-numeric list to vector".to_string()),
                                    }
                                }
                                Value::Vector(vec)
                            }
                            _ => return Err(format!("Cannot convert {} to vector", target_type)),
                        }
                    }
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
            AstNode::AutoFunction { .. } => {
                Err("AutoFunction not yet implemented".to_string())
            }
            
            AstNode::CheatBlock { .. } => {
                Err("CheatBlock not yet implemented".to_string())
            }
            
            AstNode::LiveVariable { .. } => {
                Err("LiveVariable not yet implemented".to_string())
            }
            
            AstNode::WhyExpression { .. } => {
                Err("WhyExpression not yet implemented".to_string())
            }
            
            AstNode::BlendExpression { .. } => {
                Err("BlendExpression not yet implemented".to_string())
            }
            
            AstNode::EchoVariable { .. } => {
                Err("EchoVariable not yet implemented".to_string())
            }
            
            AstNode::TraceBlock { .. } => {
                Err("TraceBlock not yet implemented".to_string())
            }
            
            AstNode::GuardExpression { .. } => {
                Err("GuardExpression not yet implemented".to_string())
            }
            
            AstNode::LensView { .. } => {
                Err("LensView not yet implemented".to_string())
            }
            
            // Memory management
            AstNode::ArenaAllocation { .. } => {
                Err("ArenaAllocation not yet implemented".to_string())
            }
            
            AstNode::StackAllocation { .. } => {
                Err("StackAllocation not yet implemented".to_string())
            }
            
            // Concurrency
            AstNode::TaskSpawn { .. } => {
                Err("TaskSpawn not yet implemented".to_string())
            }
            
            AstNode::ChannelSend { .. } => {
                Err("ChannelSend not yet implemented".to_string())
            }
            
            AstNode::ChannelReceive { .. } => {
                Err("ChannelReceive not yet implemented".to_string())
            }
            
            AstNode::ScopeBlock { .. } => {
                Err("ScopeBlock not yet implemented".to_string())
            }
            
            // Testing
            AstNode::TestCase { .. } => {
                Err("TestCase not yet implemented".to_string())
            }
            
            AstNode::PropertyTest { .. } => {
                Err("PropertyTest not yet implemented".to_string())
            }
            
            AstNode::Assertion { .. } => {
                Err("Assertion not yet implemented".to_string())
            }
            
            // Macros
            AstNode::MacroDefinition { .. } => {
                Err("MacroDefinition not yet implemented".to_string())
            }
            
            AstNode::MacroCall { .. } => {
                Err("MacroCall not yet implemented".to_string())
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
        }
    }
    
    fn call_function(&mut self, name: &str, args: &[AstNode]) -> Result<Value, String> {
        // Handle built-in functions
        match name {
            "out" => {
                if args.is_empty() {
                    println!();
                    return Ok(Value::None);
                }
                
                // Handle multiple arguments with optional formatting options
                if args.len() == 1 {
                    let val = self.eval_node(&args[0])?;
                    // Check if it's a table (list of lists)
                    if let Value::List(rows) = &val {
                        if !rows.is_empty() {
                            if let Value::List(_) = &rows[0] {
                                // It's a table
                                self.print_table(rows)?;
                                return Ok(Value::None);
                            }
                        }
                    }
                    println!("{}", val);
                } else if args.len() == 2 {
                    // Check if second argument is a formatting dict
                    let first_val = self.eval_node(&args[0])?;
                    let second_val = self.eval_node(&args[1])?;
                    
                    match second_val {
                        Value::Dict(options) => {
                            // Check for progress bar
                            if let Some(Value::Float(percent)) = options.get("progress") {
                                let width = if let Some(Value::Integer(w)) = options.get("width") {
                                    *w as usize
                                } else {
                                    40
                                };
                                let color = if let Some(Value::String(c)) = options.get("color") {
                                    c.clone()
                                } else {
                                    "green".to_string()
                                };
                                self.print_progress_bar(*percent, width, &color)?;
                                return Ok(Value::None);
                            }
                            
                            // Check for animation
                            if let Some(Value::String(anim_type)) = options.get("animate") {
                                let interval = if let Some(Value::Float(i)) = options.get("interval") {
                                    *i
                                } else {
                                    0.15
                                };
                                let count = if let Some(Value::Integer(c)) = options.get("count") {
                                    Some(*c as usize)
                                } else {
                                    None
                                };
                                let text = first_val.to_string();
                                self.print_animation(&text, anim_type, interval, count)?;
                                return Ok(Value::None);
                            }
                            
                            // Check for gradient
                            if let Some(Value::List(gradient_colors)) = options.get("gradient") {
                                let text = first_val.to_string();
                                self.print_gradient(&text, gradient_colors)?;
                                return Ok(Value::None);
                            }
                            
                            // Check for table with options
                            if let Value::List(rows) = &first_val {
                                if !rows.is_empty() {
                                    if let Value::List(_) = rows[0] {
                                        self.print_table_with_options(rows, &options)?;
                                        return Ok(Value::None);
                                    }
                                }
                            }
                            
                            // String formatting with {} placeholders
                            let mut output = first_val.to_string();
                            if let Value::String(format_str) = &first_val {
                                if format_str.contains('{') {
                                    output = self.format_string(format_str, &options)?;
                                }
                            }
                            
                            // Apply formatting options
                            let mut formatted_output = String::new();
                            let mut has_color = false;
                            let mut has_style = false;
                            
                            // Process color option
                            if let Some(Value::String(color)) = options.get("color") {
                                match color.as_str() {
                                    "red" => { formatted_output.push_str("\x1b[31m"); has_color = true; }
                                    "green" => { formatted_output.push_str("\x1b[32m"); has_color = true; }
                                    "yellow" => { formatted_output.push_str("\x1b[33m"); has_color = true; }
                                    "blue" => { formatted_output.push_str("\x1b[34m"); has_color = true; }
                                    "magenta" => { formatted_output.push_str("\x1b[35m"); has_color = true; }
                                    "cyan" => { formatted_output.push_str("\x1b[36m"); has_color = true; }
                                    "white" => { formatted_output.push_str("\x1b[37m"); has_color = true; }
                                    _ => {}
                                }
                            }
                            
                            // Process style option
                            if let Some(Value::String(style)) = options.get("style") {
                                match style.as_str() {
                                    "bold" => { formatted_output.push_str("\x1b[1m"); has_style = true; }
                                    "dim" => { formatted_output.push_str("\x1b[2m"); has_style = true; }
                                    "underline" => { formatted_output.push_str("\x1b[4m"); has_style = true; }
                                    "blink" => { formatted_output.push_str("\x1b[5m"); has_style = true; }
                                    _ => {}
                                }
                            }
                            
                            // Add the actual text
                            formatted_output.push_str(&output);
                            
                            // Reset formatting if we applied any
                            if has_color || has_style {
                                formatted_output.push_str("\x1b[0m");
                            }
                            
                            // Process end option (default is newline)
                            let end = if let Some(Value::String(end_val)) = options.get("end") {
                                end_val.clone()
                            } else {
                                "\n".to_string()
                            };
                            
                            print!("{}{}", formatted_output, end);
                        }
                        _ => {
                            // Multiple values, space-separated
                            print!("{} {}\n", first_val, second_val);
                        }
                    }
                } else {
                    // Multiple arguments, space-separated
                    for (i, arg) in args.iter().enumerate() {
                        if i > 0 { print!(" "); }
                        let val = self.eval_node(arg)?;
                        print!("{}", val);
                    }
                    println!();
                }
                Ok(Value::None)
            }
            
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
                    Value::Enum { .. } => "enum",
                    Value::EnumVariant { .. } => "enum_variant",
                            Value::Class { .. } => "class",
                            Value::Instance { .. } => "instance",
                            Value::Constructor(_) => "constructor",
                            Value::None => "none",
                };
                Ok(Value::String(type_name.to_string()))
            }
            
            "len" => {
                if args.len() != 1 {
                    return Err("len() expects exactly 1 argument".to_string());
                }
                let val = self.eval_node(&args[0])?;
                match val {
                    Value::String(s) => Ok(Value::Integer(s.len() as i64)),
                    Value::List(list) => Ok(Value::Integer(list.len() as i64)),
                    Value::Tuple(tuple) => Ok(Value::Integer(tuple.len() as i64)),
                    _ => Err("len() can only be called on strings, lists, and tuples".to_string()),
                }
            }
            
            "range" => {
                match args.len() {
                    1 => {
                        let end_val = self.eval_node(&args[0])?;
                        if let Value::Integer(end) = end_val {
                            let mut range = Vec::new();
                            for i in 0..end {
                                range.push(Value::Integer(i));
                            }
                            Ok(Value::List(range))
                        } else {
                            Err("range() expects integer arguments".to_string())
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
                            (start_val, end_val, step_val) {
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
            
            "map" => {
                // Check if we're in a pipeline context (1 arg) or normal call (2 args)
                let (list_val, func_val) = if args.len() == 1 {
                    // Pipeline context: get list from __pipeline__
                    let pipeline_val = self.get_variable("__pipeline__")
                        .unwrap_or(Value::None);
                    if matches!(pipeline_val, Value::None) {
                        return Err("map() in pipeline requires a value from the left side".to_string());
                    }
                    let func_val = self.eval_node(&args[0])?;
                    (pipeline_val, func_val)
                } else if args.len() == 2 {
                    // Normal call: map(list, func)
                    let list_val = self.eval_node(&args[0])?;
                    let func_val = self.eval_node(&args[1])?;
                    (list_val, func_val)
                } else {
                    return Err("map() expects 1 argument in pipeline or 2 arguments normally".to_string());
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
                                        return Err(format!("map() lambda must have exactly 1 parameter, got {}", params.len()));
                                    }
                                    
                                    // Execute function body
                                    let result = self.eval_node(body)?;
                                    
                                    // Restore scope
                                    self.pop_scope();
                                    
                                    result
                                }
                                _ => return Err("map() expects a function as second argument".to_string()),
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
                    _ => Err("reverse() can only be called on lists, strings, or vectors".to_string()),
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
                        list.sort_by(|a, b| {
                            match (a, b) {
                                (Value::Integer(x), Value::Integer(y)) => x.cmp(y),
                                (Value::Float(x), Value::Float(y)) => x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal),
                                (Value::Integer(x), Value::Float(y)) => (*x as f64).partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal),
                                (Value::Float(x), Value::Integer(y)) => x.partial_cmp(&(*y as f64)).unwrap_or(std::cmp::Ordering::Equal),
                                (Value::String(x), Value::String(y)) => x.cmp(y),
                                _ => std::cmp::Ordering::Equal,
                            }
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
                                (Value::Integer(x), Value::Integer(y)) => if y < x { min_val = item; },
                                (Value::Float(x), Value::Float(y)) => if y < x { min_val = item; },
                                (Value::Integer(x), Value::Float(y)) => if y < &(*x as f64) { min_val = item; },
                                (Value::Float(x), Value::Integer(y)) => if (*y as f64) < *x { min_val = item; },
                                _ => {}
                            }
                        }
                        Ok(min_val.clone())
                    }
                    Value::Vector(vec) => {
                        if vec.is_empty() {
                            return Err("Cannot find min of empty vector".to_string());
                        }
                        let min_val = vec.iter().fold(vec[0], |acc, &x| if x < acc { x } else { acc });
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
                                (Value::Integer(x), Value::Integer(y)) => if y > x { max_val = item; },
                                (Value::Float(x), Value::Float(y)) => if y > x { max_val = item; },
                                (Value::Integer(x), Value::Float(y)) => if y > &(*x as f64) { max_val = item; },
                                (Value::Float(x), Value::Integer(y)) => if (*y as f64) > *x { max_val = item; },
                                _ => {}
                            }
                        }
                        Ok(max_val.clone())
                    }
                    Value::Vector(vec) => {
                        if vec.is_empty() {
                            return Err("Cannot find max of empty vector".to_string());
                        }
                        let max_val = vec.iter().fold(vec[0], |acc, &x| if x > acc { x } else { acc });
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
                        let count = list.iter().filter(|x| self.values_equal(x, &item_val)).count();
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
                            enumerated.push(Value::Tuple(vec![Value::Integer(i as i64), item.clone()]));
                        }
                        Ok(Value::List(enumerated))
                    }
                    _ => Err("enumerate() can only be called on lists".to_string()),
                }
            }
            
            "union" => {
                if args.len() != 2 {
                    return Err("union() expects exactly 2 arguments: union(list1, list2)".to_string());
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
                    return Err("intersect() expects exactly 2 arguments: intersect(list1, list2)".to_string());
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
                    return Err("difference() expects exactly 2 arguments: difference(list1, list2)".to_string());
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
                    let pipeline_val = self.get_variable("__pipeline__")
                        .unwrap_or(Value::None);
                    if matches!(pipeline_val, Value::None) {
                        return Err("filter() in pipeline requires a value from the left side".to_string());
                    }
                    let func_val = self.eval_node(&args[0])?;
                    (pipeline_val, func_val)
                } else if args.len() == 2 {
                    // Normal call: filter(list, func)
                    let list_val = self.eval_node(&args[0])?;
                    let func_val = self.eval_node(&args[1])?;
                    (list_val, func_val)
                } else {
                    return Err("filter() expects 1 argument in pipeline or 2 arguments normally".to_string());
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
                                        return Err(format!("filter() lambda must have exactly 1 parameter, got {}", params.len()));
                                    }
                                    
                                    // Execute function body
                                    let result = self.eval_node(body)?;
                                    
                                    // Restore scope
                                    self.pop_scope();
                                    
                                    result
                                }
                                _ => return Err("filter() expects a function as second argument".to_string()),
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
                                _ => return Err("reduce() expects a function as last argument".to_string()),
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
            
            // Deque operations
            "push_front" => {
                if args.len() != 2 {
                    return Err("push_front() expects exactly 2 arguments: push_front(deque, item)".to_string());
                }
                let deque_val = self.eval_node(&args[0])?;
                let item_val = self.eval_node(&args[1])?;
                
                match deque_val {
                    Value::Deque(mut deque) => {
                        deque.insert(0, item_val);
                        Ok(Value::Deque(deque))
                    }
                    _ => Err("push_front() can only be called on deques".to_string()),
                }
            }
            
            "push_back" => {
                if args.len() != 2 {
                    return Err("push_back() expects exactly 2 arguments: push_back(deque, item)".to_string());
                }
                let deque_val = self.eval_node(&args[0])?;
                let item_val = self.eval_node(&args[1])?;
                
                match deque_val {
                    Value::Deque(mut deque) => {
                        deque.push(item_val);
                        Ok(Value::Deque(deque))
                    }
                    _ => Err("push_back() can only be called on deques".to_string()),
                }
            }
            
            "pop_front" => {
                if args.len() != 1 {
                    return Err("pop_front() expects exactly 1 argument: pop_front(deque)".to_string());
                }
                let deque_val = self.eval_node(&args[0])?;
                
                match deque_val {
                    Value::Deque(mut deque) => {
                        if !deque.is_empty() {
                            let item = deque.remove(0);
                            Ok(item)
                        } else {
                            Err("Cannot pop_front from empty deque".to_string())
                        }
                    }
                    _ => Err("pop_front() can only be called on deques".to_string()),
                }
            }
            
            "pop_back" => {
                if args.len() != 1 {
                    return Err("pop_back() expects exactly 1 argument: pop_back(deque)".to_string());
                }
                let deque_val = self.eval_node(&args[0])?;
                
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
            
            "peek_front" => {
                if args.len() != 1 {
                    return Err("peek_front() expects exactly 1 argument: peek_front(deque)".to_string());
                }
                let deque_val = self.eval_node(&args[0])?;
                
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
            
            "peek_back" => {
                if args.len() != 1 {
                    return Err("peek_back() expects exactly 1 argument: peek_back(deque)".to_string());
                }
                let deque_val = self.eval_node(&args[0])?;
                
                match deque_val {
                    Value::Deque(deque) => {
                        if !deque.is_empty() {
                            Ok(deque[deque.len() - 1].clone())
                        } else {
                            Err("Cannot peek_back on empty deque".to_string())
                        }
                    }
                    _ => Err("peek_back() can only be called on deques".to_string()),
                }
            }
            
            // Priority Queue operations
            "pq_push" => {
                if args.len() != 3 {
                    return Err("pq_push() expects exactly 3 arguments: pq_push(pq, priority, item)".to_string());
                }
                let pq_val = self.eval_node(&args[0])?;
                let priority_val = self.eval_node(&args[1])?;
                let item_val = self.eval_node(&args[2])?;
                
                let priority = match priority_val {
                    Value::Integer(p) => p,
                    _ => return Err("Priority must be an integer".to_string()),
                };
                
                match pq_val {
                    Value::PriorityQ(mut pq) => {
                        pq.push((priority, item_val));
                        // Sort to maintain min-heap property (lowest priority first)
                        pq.sort_by_key(|(p, _)| *p);
                        Ok(Value::PriorityQ(pq))
                    }
                    _ => Err("pq_push() can only be called on priority queues".to_string()),
                }
            }
            
            "pq_pop" => {
                if args.len() != 1 {
                    return Err("pq_pop() expects exactly 1 argument: pq_pop(pq)".to_string());
                }
                let pq_val = self.eval_node(&args[0])?;
                
                match pq_val {
                    Value::PriorityQ(mut pq) => {
                        if !pq.is_empty() {
                            // Remove and return the item with lowest priority
                            let (_, item) = pq.remove(0);
                            Ok(item)
                        } else {
                            Err("Cannot pop from empty priority queue".to_string())
                        }
                    }
                    _ => Err("pq_pop() can only be called on priority queues".to_string()),
                }
            }
            
            "pq_peek" => {
                if args.len() != 1 {
                    return Err("pq_peek() expects exactly 1 argument: pq_peek(pq)".to_string());
                }
                let pq_val = self.eval_node(&args[0])?;
                
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
                        if let Value::String(search) = item_val {
                            Ok(Value::Boolean(s.contains(&search)))
                        } else if let Value::Char(ch) = item_val {
                            Ok(Value::Boolean(s.contains(ch)))
                        } else {
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
                    _ => Err("contains() can only be called on lists, strings, or dicts".to_string()),
                }
            }
            
            "keys" => {
                if args.len() != 1 {
                    return Err("keys() expects exactly 1 argument".to_string());
                }
                let dict_val = self.eval_node(&args[0])?;
                
                match dict_val {
                    Value::Dict(dict) => {
                        let keys: Vec<Value> = dict.keys()
                            .map(|k| Value::String(k.clone()))
                            .collect();
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
                            items.push(Value::Tuple(vec![
                                Value::String(k.clone()),
                                v.clone(),
                            ]));
                        }
                        Ok(Value::List(items))
                    }
                    _ => Err("items() can only be called on dictionaries".to_string()),
                }
            }
            
            "get" => {
                if args.len() < 2 || args.len() > 3 {
                    return Err("get() expects 2 or 3 arguments: get(dict, key, [default])".to_string());
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
                    return Err("remove() expects exactly 2 arguments: remove(dict, key)".to_string());
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
                    return Err("merge() expects exactly 2 arguments: merge(dict1, dict2)".to_string());
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
                    return Err("update() expects exactly 2 arguments: update(dict1, dict2)".to_string());
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
                    Value::Dict(_) => {
                        Ok(Value::Dict(HashMap::new()))
                    }
                    _ => Err("clear() can only be called on dictionaries".to_string()),
                }
            }
            
            // Enum methods
            "enum_name" => {
                if args.len() != 2 {
                    return Err("enum_name() expects exactly 2 arguments: enum_name(enum, value)".to_string());
                }
                let enum_val = self.eval_node(&args[0])?;
                let value_val = self.eval_node(&args[1])?;
                
                match enum_val {
                    Value::Dict(dict) => {
                        // Find the key (variant name) that has this value
                        for (name, value) in dict.iter() {
                            if self.values_equal(value, &value_val) {
                                return Ok(Value::String(name.clone()));
                            }
                        }
                        Ok(Value::None)
                    }
                    _ => Err("enum_name() can only be called on enums".to_string()),
                }
            }
            
            "enum_value" => {
                if args.len() != 2 {
                    return Err("enum_value() expects exactly 2 arguments: enum_value(enum, name)".to_string());
                }
                let enum_val = self.eval_node(&args[0])?;
                let name_val = self.eval_node(&args[1])?;
                
                match (enum_val, name_val) {
                    (Value::Dict(dict), Value::String(name)) => {
                        if let Some(value) = dict.get(&name) {
                            Ok(value.clone())
                        } else {
                            Ok(Value::None)
                        }
                    }
                    _ => Err("enum_value() can only be called on enums with string name".to_string()),
                }
            }
            
            "enum_has" => {
                if args.len() != 2 {
                    return Err("enum_has() expects exactly 2 arguments: enum_has(enum, value)".to_string());
                }
                let enum_val = self.eval_node(&args[0])?;
                let value_val = self.eval_node(&args[1])?;
                
                match enum_val {
                    Value::Dict(dict) => {
                        let has = dict.values().any(|v| self.values_equal(v, &value_val));
                        Ok(Value::Boolean(has))
                    }
                    _ => Err("enum_has() can only be called on enums".to_string()),
                }
            }
            
            // File I/O functions
            "read" => {
                if args.len() != 1 {
                    return Err("read() expects exactly 1 argument: read(filename)".to_string());
                }
                let filename_val = self.eval_node(&args[0])?;
                let filename = match filename_val {
                    Value::String(s) => s,
                    _ => return Err("read() filename must be a string".to_string()),
                };
                
                match std::fs::read_to_string(&filename) {
                    Ok(content) => Ok(Value::String(content)),
                    Err(e) => Err(format!("Failed to read file '{}': {}", filename, e)),
                }
            }
            
            "write" => {
                if args.len() != 2 {
                    return Err("write() expects exactly 2 arguments: write(filename, content)".to_string());
                }
                let filename_val = self.eval_node(&args[0])?;
                let content_val = self.eval_node(&args[1])?;
                
                let filename = match filename_val {
                    Value::String(s) => s,
                    _ => return Err("write() filename must be a string".to_string()),
                };
                
                let content = content_val.to_string();
                
                match std::fs::write(&filename, content) {
                    Ok(_) => Ok(Value::Boolean(true)),
                    Err(e) => Err(format!("Failed to write file '{}': {}", filename, e)),
                }
            }
            
            "read_lines" => {
                if args.len() != 1 {
                    return Err("read_lines() expects exactly 1 argument: read_lines(filename)".to_string());
                }
                let filename_val = self.eval_node(&args[0])?;
                let filename = match filename_val {
                    Value::String(s) => s,
                    _ => return Err("read_lines() filename must be a string".to_string()),
                };
                
                match std::fs::read_to_string(&filename) {
                    Ok(content) => {
                        let lines: Vec<Value> = content.lines()
                            .map(|line| Value::String(line.to_string()))
                            .collect();
                        Ok(Value::List(lines))
                    }
                    Err(e) => Err(format!("Failed to read file '{}': {}", filename, e)),
                }
            }
            
            "write_lines" => {
                if args.len() != 2 {
                    return Err("write_lines() expects exactly 2 arguments: write_lines(filename, lines)".to_string());
                }
                let filename_val = self.eval_node(&args[0])?;
                let lines_val = self.eval_node(&args[1])?;
                
                let filename = match filename_val {
                    Value::String(s) => s,
                    _ => return Err("write_lines() filename must be a string".to_string()),
                };
                
                let lines = match lines_val {
                    Value::List(list) => {
                        list.iter()
                            .map(|v| v.to_string())
                            .collect::<Vec<_>>()
                            .join("\n")
                    }
                    _ => return Err("write_lines() lines must be a list".to_string()),
                };
                
                match std::fs::write(&filename, lines) {
                    Ok(_) => Ok(Value::Boolean(true)),
                    Err(e) => Err(format!("Failed to write file '{}': {}", filename, e)),
                }
            }
            
            "split" => {
                if args.len() != 2 {
                    return Err("split() expects exactly 2 arguments".to_string());
                }
                let string_val = self.eval_node(&args[0])?;
                let delimiter_val = self.eval_node(&args[1])?;
                
                match (string_val, delimiter_val) {
                    (Value::String(s), Value::String(delim)) => {
                        let parts: Vec<Value> = s.split(&delim)
                            .map(|part| Value::String(part.to_string()))
                            .collect();
                        Ok(Value::List(parts))
                    }
                    _ => Err("split() expects string and delimiter arguments".to_string()),
                }
            }
            
            "join" => {
                if args.len() != 2 {
                    return Err("join() expects exactly 2 arguments".to_string());
                }
                let list_val = self.eval_node(&args[0])?;
                let separator_val = self.eval_node(&args[1])?;
                
                match (list_val, separator_val) {
                    (Value::List(list), Value::String(sep)) => {
                        let strings: Result<Vec<String>, String> = list.iter()
                            .map(|v| match v {
                                Value::String(s) => Ok(s.clone()),
                                _ => Ok(v.to_string()),
                            })
                            .collect();
                        
                        match strings {
                            Ok(str_vec) => Ok(Value::String(str_vec.join(&sep))),
                            Err(e) => Err(e),
                        }
                    }
                    _ => Err("join() expects list and separator string arguments".to_string()),
                }
            }
            
            "substring" => {
                match args.len() {
                    2 => {
                        let string_val = self.eval_node(&args[0])?;
                        let start_val = self.eval_node(&args[1])?;
                        
                        match (string_val, start_val) {
                            (Value::String(s), Value::Integer(start)) => {
                                let chars: Vec<char> = s.chars().collect();
                                let start_idx = if start < 0 {
                                    (chars.len() as i64 + start).max(0) as usize
                                } else {
                                    (start as usize).min(chars.len())
                                };
                                
                                let result: String = chars[start_idx..].iter().collect();
                                Ok(Value::String(result))
                            }
                            _ => Err("substring() expects string and integer arguments".to_string()),
                        }
                    }
                    3 => {
                        let string_val = self.eval_node(&args[0])?;
                        let start_val = self.eval_node(&args[1])?;
                        let end_val = self.eval_node(&args[2])?;
                        
                        match (string_val, start_val, end_val) {
                            (Value::String(s), Value::Integer(start), Value::Integer(end)) => {
                                let chars: Vec<char> = s.chars().collect();
                                let start_idx = if start < 0 {
                                    (chars.len() as i64 + start).max(0) as usize
                                } else {
                                    (start as usize).min(chars.len())
                                };
                                let end_idx = if end < 0 {
                                    (chars.len() as i64 + end).max(0) as usize
                                } else {
                                    (end as usize).min(chars.len())
                                };
                                
                                if start_idx <= end_idx {
                                    let result: String = chars[start_idx..end_idx].iter().collect();
                                    Ok(Value::String(result))
                                } else {
                                    Ok(Value::String(String::new()))
                                }
                            }
                            _ => Err("substring() expects string and integer arguments".to_string()),
                        }
                    }
                    _ => Err("substring() expects 2 or 3 arguments".to_string()),
                }
            }
            
            // String manipulation functions
            "upper" => {
                if args.len() != 1 {
                    return Err("upper() expects exactly 1 argument".to_string());
                }
                let val = self.eval_node(&args[0])?;
                match val {
                    Value::String(s) => Ok(Value::String(s.to_uppercase())),
                    _ => Err("upper() can only be called on strings".to_string()),
                }
            }
            
            "lower" => {
                if args.len() != 1 {
                    return Err("lower() expects exactly 1 argument".to_string());
                }
                let val = self.eval_node(&args[0])?;
                match val {
                    Value::String(s) => Ok(Value::String(s.to_lowercase())),
                    _ => Err("lower() can only be called on strings".to_string()),
                }
            }
            
            "trim" => {
                if args.len() != 1 {
                    return Err("trim() expects exactly 1 argument".to_string());
                }
                let val = self.eval_node(&args[0])?;
                match val {
                    Value::String(s) => Ok(Value::String(s.trim().to_string())),
                    _ => Err("trim() can only be called on strings".to_string()),
                }
            }
            
            "replace" => {
                if args.len() != 3 {
                    return Err("replace() expects exactly 3 arguments".to_string());
                }
                let string_val = self.eval_node(&args[0])?;
                let from_val = self.eval_node(&args[1])?;
                let to_val = self.eval_node(&args[2])?;
                
                match (string_val, from_val, to_val) {
                    (Value::String(s), Value::String(from), Value::String(to)) => {
                        Ok(Value::String(s.replace(&from, &to)))
                    }
                    _ => Err("replace() expects string arguments".to_string()),
                }
            }
            
            "starts_with" => {
                if args.len() != 2 {
                    return Err("starts_with() expects exactly 2 arguments".to_string());
                }
                let string_val = self.eval_node(&args[0])?;
                let prefix_val = self.eval_node(&args[1])?;
                
                match (string_val, prefix_val) {
                    (Value::String(s), Value::String(prefix)) => {
                        Ok(Value::Boolean(s.starts_with(&prefix)))
                    }
                    _ => Err("starts_with() expects string arguments".to_string()),
                }
            }
            
            "ends_with" => {
                if args.len() != 2 {
                    return Err("ends_with() expects exactly 2 arguments".to_string());
                }
                let string_val = self.eval_node(&args[0])?;
                let suffix_val = self.eval_node(&args[1])?;
                
                match (string_val, suffix_val) {
                    (Value::String(s), Value::String(suffix)) => {
                        Ok(Value::Boolean(s.ends_with(&suffix)))
                    }
                    _ => Err("ends_with() expects string arguments".to_string()),
                }
            }
            
            "repeat" => {
                if args.len() != 2 {
                    return Err("repeat() expects exactly 2 arguments".to_string());
                }
                let string_val = self.eval_node(&args[0])?;
                let count_val = self.eval_node(&args[1])?;
                
                match (string_val, count_val) {
                    (Value::String(s), Value::Integer(count)) => {
                        if count < 0 {
                            return Err("repeat() count must be non-negative".to_string());
                        }
                        Ok(Value::String(s.repeat(count as usize)))
                    }
                    _ => Err("repeat() expects string and integer arguments".to_string()),
                }
            }
            
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
                                let string_cells: Vec<String> = cells.iter()
                                    .map(|v| format!("{}", v))
                                    .collect();
                                
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
                        let total_width: usize = max_widths.iter().sum::<usize>() + (max_widths.len() * 3) + 1;
                        println!("{}", "─".repeat(total_width));
                        
                        for (row_idx, row) in string_rows.iter().enumerate() {
                            print!("│");
                            for (i, cell) in row.iter().enumerate() {
                                print!(" {:width$} │", cell, width = max_widths[i]);
                            }
                            println!();
                            
                            if row_idx == 0 {
                                // Header separator
                                println!("{}", "─".repeat(total_width));
                            }
                        }
                        
                        println!("{}", "─".repeat(total_width));
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
                    print!("{}", "█".repeat(filled));
                    print!("{}", " ".repeat(empty));
                    print!("] {:.0}%", current_percent);
                    io::stdout().flush().unwrap();
                    
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
                let colors = vec![31, 33, 32, 36, 34, 35]; // Red, Yellow, Green, Cyan, Blue, Magenta
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
                    return Err("gradient() expects 3 arguments: gradient(start_color, end_color, text)".to_string());
                }
                
                let text_val = self.eval_node(&args[2])?;
                let text = match text_val {
                    Value::String(s) => s,
                    other => format!("{}", other),
                };
                
                // Simple gradient using color interpolation
                // For now, just alternate between two colors
                let colors = vec![32, 36]; // Green to Cyan
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
                        let texts: Vec<String> = items.iter()
                            .map(|v| format!("{}", v))
                            .collect();
                        
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
                
                match (string_val, pattern_val) {
                    (Value::String(s), Value::String(pattern)) => {
                        match s.find(&pattern) {
                            Some(index) => Ok(Value::Integer(index as i64)),
                            None => Ok(Value::Integer(-1)),
                        }
                    }
                    _ => Err("find() expects string arguments".to_string()),
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
                            return Err("Matrix dimensions incompatible for multiplication".to_string());
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
                            let dot_product: f64 = row.iter().zip(v.iter()).map(|(a, b)| a * b).sum();
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
                            _ => return Err("Set elements must be convertible to strings".to_string()),
                        };
                        set.insert(key);
                        Ok(Value::Set(set))
                    }
                    _ => Err("add() can only be called on sets".to_string()),
                }
            }
            
            "size" => {
                if args.len() != 1 {
                    return Err("size() expects exactly 1 argument".to_string());
                }
                let val = self.eval_node(&args[0])?;
                
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
            
            // Counter operations
            "most_common" => {
                if args.len() < 1 || args.len() > 2 {
                    return Err("most_common() expects 1 or 2 arguments".to_string());
                }
                let counter_val = self.eval_node(&args[0])?;
                let n = if args.len() == 2 {
                    match self.eval_node(&args[1])? {
                        Value::Integer(i) => i as usize,
                        _ => return Err("most_common() second argument must be an integer".to_string()),
                    }
                } else {
                    usize::MAX
                };
                
                match counter_val {
                    Value::Counter(counter) => {
                        let mut items: Vec<_> = counter.iter().collect();
                        items.sort_by(|a, b| b.1.cmp(a.1)); // Sort by count descending
                        
                        let result: Vec<Value> = items.iter()
                            .take(n)
                            .map(|(key, count)| Value::Tuple(vec![
                                Value::String(key.to_string()),
                                Value::Integer(**count)
                            ]))
                            .collect();
                        
                        Ok(Value::List(result))
                    }
                    _ => Err("most_common() can only be called on counters".to_string()),
                }
            }
            
            "total" => {
                if args.len() != 1 {
                    return Err("total() expects exactly 1 argument".to_string());
                }
                let counter_val = self.eval_node(&args[0])?;
                
                match counter_val {
                    Value::Counter(counter) => {
                        let total: i64 = counter.values().sum();
                        Ok(Value::Integer(total))
                    }
                    _ => Err("total() can only be called on counters".to_string()),
                }
            }
            
            // Algorithm helpers
            "binary_search" => {
                if args.len() != 2 {
                    return Err("binary_search() expects exactly 2 arguments".to_string());
                }
                let list_val = self.eval_node(&args[0])?;
                let target_val = self.eval_node(&args[1])?;
                
                match list_val {
                    Value::List(list) => {
                        // Simple binary search for integers
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
                                    _ => return Err("binary_search() requires a sorted list of integers".to_string()),
                                }
                            }
                            Ok(Value::Integer(-1)) // Not found
                        } else {
                            Err("binary_search() target must be an integer".to_string())
                        }
                    }
                    _ => Err("binary_search() can only be called on lists".to_string()),
                }
            }
            
            "is_sorted" => {
                if args.len() != 1 {
                    return Err("is_sorted() expects exactly 1 argument".to_string());
                }
                let list_val = self.eval_node(&args[0])?;
                
                match list_val {
                    Value::List(list) => {
                        for i in 1..list.len() {
                            match (&list[i-1], &list[i]) {
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
                                _ => return Err("is_sorted() requires comparable elements".to_string()),
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
            
            "group_by" => {
                if args.len() != 2 {
                    return Err("group_by() expects exactly 2 arguments".to_string());
                }
                let list_val = self.eval_node(&args[0])?;
                let key_func = match &args[1] {
                    AstNode::Identifier(name) => name.clone(),
                    _ => return Err("group_by() expects a function as second argument".to_string()),
                };
                
                match list_val {
                    Value::List(list) => {
                        let mut groups: HashMap<String, Vec<Value>> = HashMap::new();
                        
                        for item in list {
                            // Set the item as _ for the key function
                            self.set_variable("_".to_string(), item.clone());
                            let key_result = self.call_function(&key_func, &[])?;
                            
                            let key = match key_result {
                                Value::String(s) => s,
                                Value::Integer(i) => i.to_string(),
                                Value::Float(f) => f.to_string(),
                                Value::Boolean(b) => b.to_string(),
                                _ => return Err("group_by() key function must return a string-convertible value".to_string()),
                            };
                            
                            groups.entry(key).or_insert_with(Vec::new).push(item);
                        }
                        
                        // Convert to dict
                        let mut result = HashMap::new();
                        for (key, values) in groups {
                            result.insert(key, Value::List(values));
                        }
                        
                        Ok(Value::Dict(result))
                    }
                    _ => Err("group_by() can only be called on lists".to_string()),
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
            
            "chunk" => {
                if args.len() != 2 {
                    return Err("chunk() expects exactly 2 arguments".to_string());
                }
                let list_val = self.eval_node(&args[0])?;
                let size_val = self.eval_node(&args[1])?;
                
                match (list_val, size_val) {
                    (Value::List(list), Value::Integer(size)) => {
                        if size <= 0 {
                            return Err("chunk() size must be positive".to_string());
                        }
                        let size = size as usize;
                        let mut result = Vec::new();
                        
                        for chunk in list.chunks(size) {
                            result.push(Value::List(chunk.to_vec()));
                        }
                        
                        Ok(Value::List(result))
                    }
                    _ => Err("chunk() expects list and integer arguments".to_string()),
                }
            }
            
            "partition" => {
                if args.len() != 2 {
                    return Err("partition() expects exactly 2 arguments".to_string());
                }
                let list_val = self.eval_node(&args[0])?;
                let predicate = match &args[1] {
                    AstNode::Identifier(name) => name.clone(),
                    _ => return Err("partition() expects a function as second argument".to_string()),
                };
                
                match list_val {
                    Value::List(list) => {
                        let mut true_items = Vec::new();
                        let mut false_items = Vec::new();
                        
                        for item in list {
                            self.set_variable("_".to_string(), item.clone());
                            let result = self.call_function(&predicate, &[])?;
                            
                            if self.is_truthy(&result) {
                                true_items.push(item);
                            } else {
                                false_items.push(item);
                            }
                        }
                        
                        Ok(Value::Tuple(vec![Value::List(true_items), Value::List(false_items)]))
                    }
                    _ => Err("partition() can only be called on lists".to_string()),
                }
            }
            
            // Mathematical functions
            "abs" => {
                if args.len() != 1 {
                    return Err("abs() expects exactly 1 argument".to_string());
                }
                let val = self.eval_node(&args[0])?;
                
                match val {
                    Value::Integer(i) => Ok(Value::Integer(i.abs())),
                    Value::Float(f) => Ok(Value::Float(f.abs())),
                    _ => Err("abs() can only be called on numbers".to_string()),
                }
            }
            
            "sqrt" => {
                if args.len() != 1 {
                    return Err("sqrt() expects exactly 1 argument".to_string());
                }
                let val = self.eval_node(&args[0])?;
                
                match val {
                    Value::Integer(i) => Ok(Value::Float((i as f64).sqrt())),
                    Value::Float(f) => Ok(Value::Float(f.sqrt())),
                    _ => Err("sqrt() can only be called on numbers".to_string()),
                }
            }
            
            "pow" => {
                if args.len() != 2 {
                    return Err("pow() expects exactly 2 arguments".to_string());
                }
                let base_val = self.eval_node(&args[0])?;
                let exp_val = self.eval_node(&args[1])?;
                
                match (base_val, exp_val) {
                    (Value::Integer(base), Value::Integer(exp)) => {
                        if exp >= 0 {
                            Ok(Value::Integer(base.pow(exp as u32)))
                        } else {
                            Ok(Value::Float((base as f64).powf(exp as f64)))
                        }
                    }
                    (Value::Float(base), Value::Integer(exp)) => {
                        Ok(Value::Float(base.powf(exp as f64)))
                    }
                    (Value::Integer(base), Value::Float(exp)) => {
                        Ok(Value::Float((base as f64).powf(exp)))
                    }
                    (Value::Float(base), Value::Float(exp)) => {
                        Ok(Value::Float(base.powf(exp)))
                    }
                    _ => Err("pow() expects numeric arguments".to_string()),
                }
            }
            
            "ceil" => {
                if args.len() != 1 {
                    return Err("ceil() expects exactly 1 argument".to_string());
                }
                let val = self.eval_node(&args[0])?;
                
                match val {
                    Value::Float(f) => Ok(Value::Integer(f.ceil() as i64)),
                    Value::Integer(i) => Ok(Value::Integer(i)),
                    _ => Err("ceil() can only be called on numbers".to_string()),
                }
            }
            
            "floor" => {
                if args.len() != 1 {
                    return Err("floor() expects exactly 1 argument".to_string());
                }
                let val = self.eval_node(&args[0])?;
                
                match val {
                    Value::Float(f) => Ok(Value::Integer(f.floor() as i64)),
                    Value::Integer(i) => Ok(Value::Integer(i)),
                    _ => Err("floor() can only be called on numbers".to_string()),
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
                            return Err("Vectors must have same length for distance calculation".to_string());
                        }
                        let dist_sq: f64 = a.iter().zip(b.iter()).map(|(x, y)| (x - y).powi(2)).sum();
                        Ok(Value::Float(dist_sq.sqrt()))
                    }
                    (Value::Tuple(a), Value::Tuple(b)) => {
                        if a.len() != b.len() {
                            return Err("Tuples must have same length for distance calculation".to_string());
                        }
                        let mut dist_sq = 0.0;
                        for (av, bv) in a.iter().zip(b.iter()) {
                            let a_num = match av {
                                Value::Integer(i) => *i as f64,
                                Value::Float(f) => *f,
                                _ => return Err("Distance calculation requires numeric values".to_string()),
                            };
                            let b_num = match bv {
                                Value::Integer(i) => *i as f64,
                                Value::Float(f) => *f,
                                _ => return Err("Distance calculation requires numeric values".to_string()),
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
                        
                        let det = match m.len() {
                            1 => m[0][0],
                            2 => m[0][0] * m[1][1] - m[0][1] * m[1][0],
                            3 => {
                                m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1]) -
                                m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0]) +
                                m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
                            }
                            _ => return Err("Determinant only implemented for 1x1, 2x2, and 3x3 matrices".to_string()),
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
                        for i in 0..size {
                            matrix[i][i] = 1.0;
                        }
                        Ok(Value::Matrix(matrix))
                    }
                    _ => Err("identity() expects an integer size".to_string()),
                }
            }
            
            "zeros" => {
                match args.len() {
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
                }
            }
            
            "ones" => {
                match args.len() {
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
                }
            }
            
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
                        Ok(Value::List(positions.into_iter().map(|p| Value::Integer(p as i64)).collect()))
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
                        Ok(Value::List(z_array.into_iter().map(|z| Value::Integer(z as i64)).collect()))
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
            
            "bfs" => {
                if args.len() < 2 || args.len() > 3 {
                    return Err("bfs() expects 2-3 arguments: graph, start, [goal]".to_string());
                }
                let graph_val = self.eval_node(&args[0])?;
                let start_val = self.eval_node(&args[1])?;
                let goal_val = if args.len() == 3 { Some(self.eval_node(&args[2])?) } else { None };
                
                match (graph_val, start_val) {
                    (Value::Graph(graph), Value::String(start)) => {
                        let goal = goal_val.and_then(|v| if let Value::String(s) = v { Some(s) } else { None });
                        let path = self.bfs_search(&graph, &start, goal.as_deref())?;
                        Ok(Value::List(path.into_iter().map(|n| Value::String(n)).collect()))
                    }
                    _ => Err("bfs() expects graph and string start node".to_string()),
                }
            }
            
            "dfs" => {
                if args.len() < 2 || args.len() > 3 {
                    return Err("dfs() expects 2-3 arguments: graph, start, [goal]".to_string());
                }
                let graph_val = self.eval_node(&args[0])?;
                let start_val = self.eval_node(&args[1])?;
                let goal_val = if args.len() == 3 { Some(self.eval_node(&args[2])?) } else { None };
                
                match (graph_val, start_val) {
                    (Value::Graph(graph), Value::String(start)) => {
                        let goal = goal_val.and_then(|v| if let Value::String(s) = v { Some(s) } else { None });
                        let path = self.dfs_search(&graph, &start, goal.as_deref())?;
                        Ok(Value::List(path.into_iter().map(|n| Value::String(n)).collect()))
                    }
                    _ => Err("dfs() expects graph and string start node".to_string()),
                }
            }
            
            // Graph operations
            "add_node" => {
                if args.len() != 2 {
                    return Err("add_node() expects exactly 2 arguments: add_node(graph, node_name)".to_string());
                }
                let graph_val = self.eval_node(&args[0])?;
                let node_val = self.eval_node(&args[1])?;
                
                let node_name = match node_val {
                    Value::String(s) => s,
                    _ => return Err("Node name must be a string".to_string()),
                };
                
                match graph_val {
                    Value::Graph(mut graph) => {
                        if !graph.contains_key(&node_name) {
                            graph.insert(node_name, Vec::new());
                        }
                        Ok(Value::Graph(graph))
                    }
                    _ => Err("add_node() can only be called on graphs".to_string()),
                }
            }
            
            "add_edge" => {
                if args.len() < 3 || args.len() > 4 {
                    return Err("add_edge() expects 3-4 arguments: add_edge(graph, from, to, [weight])".to_string());
                }
                let graph_val = self.eval_node(&args[0])?;
                let from_val = self.eval_node(&args[1])?;
                let to_val = self.eval_node(&args[2])?;
                let weight_val = if args.len() == 4 { 
                    self.eval_node(&args[3])? 
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
                        // Ensure both nodes exist
                        if !graph.contains_key(&from) {
                            graph.insert(from.clone(), Vec::new());
                        }
                        if !graph.contains_key(&to) {
                            graph.insert(to.clone(), Vec::new());
                        }
                        
                        // Add edge
                        if let Some(edges) = graph.get_mut(&from) {
                            edges.push((to, weight));
                        }
                        
                        Ok(Value::Graph(graph))
                    }
                    _ => Err("add_edge() can only be called on graphs".to_string()),
                }
            }
            
            "get_neighbors" => {
                if args.len() != 2 {
                    return Err("get_neighbors() expects exactly 2 arguments: get_neighbors(graph, node)".to_string());
                }
                let graph_val = self.eval_node(&args[0])?;
                let node_val = self.eval_node(&args[1])?;
                
                let node = match node_val {
                    Value::String(s) => s,
                    _ => return Err("Node must be a string".to_string()),
                };
                
                match graph_val {
                    Value::Graph(graph) => {
                        if let Some(neighbors) = graph.get(&node) {
                            let neighbor_list: Vec<Value> = neighbors.iter()
                                .map(|(n, w)| Value::Tuple(vec![Value::String(n.clone()), Value::Float(*w)]))
                                .collect();
                            Ok(Value::List(neighbor_list))
                        } else {
                            Err(format!("Node '{}' not found in graph", node))
                        }
                    }
                    _ => Err("get_neighbors() can only be called on graphs".to_string()),
                }
            }
            
            "dijkstra" => {
                if args.len() < 2 || args.len() > 3 {
                    return Err("dijkstra() expects 2-3 arguments: graph, start, [goal]".to_string());
                }
                let graph_val = self.eval_node(&args[0])?;
                let start_val = self.eval_node(&args[1])?;
                let goal_val = if args.len() == 3 { Some(self.eval_node(&args[2])?) } else { None };
                
                match (graph_val, start_val) {
                    (Value::Graph(graph), Value::String(start)) => {
                        let goal = goal_val.and_then(|v| if let Value::String(s) = v { Some(s) } else { None });
                        let result = self.dijkstra_search(&graph, &start, goal.as_deref())?;
                        match result {
                            (path, distance) => {
                                let mut result_dict = HashMap::new();
                                result_dict.insert("path".to_string(), Value::List(path.into_iter().map(|n| Value::String(n)).collect()));
                                result_dict.insert("distance".to_string(), Value::Float(distance));
                                Ok(Value::Dict(result_dict))
                            }
                        }
                    }
                    _ => Err("dijkstra() expects graph and string start node".to_string()),
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
                        let signal_list: Vec<Value> = signal.into_iter().map(|f| Value::Float(f)).collect();
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
                if args.len() != 0 {
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
                if args.len() != 0 {
                    return Err("today() expects no arguments".to_string());
                }
                // Simple date format - in practice you'd use a proper date library
                Ok(Value::Date("2026-01-31".to_string()))
            }
            
            // Random functions
            "random" => {
                if args.len() == 0 {
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
            
            // Utility functions
            "type_of" => {
                if args.len() != 1 {
                    return Err("type_of() expects exactly 1 argument".to_string());
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
                    Value::Enum { .. } => "enum",
                    Value::EnumVariant { .. } => "enum_variant",
                            Value::Class { .. } => "class",
                            Value::Instance { .. } => "instance",
                            Value::Constructor(_) => "constructor",
                            Value::None => "none",
                };
                Ok(Value::String(type_name.to_string()))
            }
            
            "is_empty" => {
                if args.len() != 1 {
                    return Err("is_empty() expects exactly 1 argument".to_string());
                }
                let val = self.eval_node(&args[0])?;
                
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
            
            "sliding_window" => {
                if args.len() != 2 {
                    return Err("sliding_window() expects exactly 2 arguments".to_string());
                }
                let list_val = self.eval_node(&args[0])?;
                let size_val = self.eval_node(&args[1])?;
                
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
            
            "permutations" => {
                if args.len() != 1 {
                    return Err("permutations() expects exactly 1 argument".to_string());
                }
                let list_val = self.eval_node(&args[0])?;
                
                match list_val {
                    Value::List(list) => {
                        if list.len() > 8 {
                            return Err("permutations() limited to lists of 8 elements or fewer".to_string());
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
                            return Err("combinations() r must be between 0 and list length".to_string());
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
            
            _ => { // If not built-in, look for user-defined function
                let func_val = self.get_variable(name)?;
                if let Value::Function { name: _, params, body } = func_val {
                    let mut eval_args = Vec::new();
                    for arg in args {
                        eval_args.push(self.eval_node(arg)?); // Evaluate arguments
                    }
                    self.call_function_internal(name, &eval_args, &params, &*body, None)
                } else {
                    Err(format!("'{}' is not a function", name))
                }
            }
        }
    }
    
    fn eval_binary_op(&self, left: &Value, op: &BinaryOp, right: &Value) -> Result<Value, String> {
        match (left, op, right) {
            (Value::Integer(a), BinaryOp::Add, Value::Integer(b)) => Ok(Value::Integer(a + b)),
            (Value::Integer(a), BinaryOp::Subtract, Value::Integer(b)) => Ok(Value::Integer(a - b)),
            (Value::Integer(a), BinaryOp::Multiply, Value::Integer(b)) => Ok(Value::Integer(a * b)),
            (Value::Integer(a), BinaryOp::Divide, Value::Integer(b)) => {
                if *b == 0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Value::Integer(a / b))
                }
            }
            (Value::Integer(a), BinaryOp::Modulo, Value::Integer(b)) => Ok(Value::Integer(a % b)),
            (Value::Integer(a), BinaryOp::Power, Value::Integer(b)) => {
                Ok(Value::Integer(a.pow(*b as u32)))
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
                if a.len() != b.len() || (a.len() > 0 && a[0].len() != b[0].len()) {
                    return Err("Matrix dimensions must match for addition".to_string());
                }
                let mut result = Vec::new();
                for (row_a, row_b) in a.iter().zip(b.iter()) {
                    let result_row: Vec<f64> = row_a.iter().zip(row_b.iter()).map(|(x, y)| x + y).collect();
                    result.push(result_row);
                }
                Ok(Value::Matrix(result))
            }
            (Value::Matrix(a), BinaryOp::Subtract, Value::Matrix(b)) => {
                if a.len() != b.len() || (a.len() > 0 && a[0].len() != b[0].len()) {
                    return Err("Matrix dimensions must match for subtraction".to_string());
                }
                let mut result = Vec::new();
                for (row_a, row_b) in a.iter().zip(b.iter()) {
                    let result_row: Vec<f64> = row_a.iter().zip(row_b.iter()).map(|(x, y)| x - y).collect();
                    result.push(result_row);
                }
                Ok(Value::Matrix(result))
            }
            (Value::Matrix(a), BinaryOp::Multiply, Value::Float(scalar)) => {
                let result: Vec<Vec<f64>> = a.iter().map(|row| {
                    row.iter().map(|x| x * scalar).collect()
                }).collect();
                Ok(Value::Matrix(result))
            }
            (Value::Float(scalar), BinaryOp::Multiply, Value::Matrix(a)) => {
                let result: Vec<Vec<f64>> = a.iter().map(|row| {
                    row.iter().map(|x| scalar * x).collect()
                }).collect();
                Ok(Value::Matrix(result))
            }
            
            (Value::Integer(a), BinaryOp::Equal, Value::Integer(b)) => Ok(Value::Boolean(a == b)),
            (Value::Float(a), BinaryOp::Equal, Value::Float(b)) => Ok(Value::Boolean(a == b)),
            (Value::String(a), BinaryOp::Equal, Value::String(b)) => Ok(Value::Boolean(a == b)),
            (Value::Boolean(a), BinaryOp::Equal, Value::Boolean(b)) => Ok(Value::Boolean(a == b)),
            
            (Value::Integer(a), BinaryOp::Less, Value::Integer(b)) => Ok(Value::Boolean(a < b)),
            (Value::Integer(a), BinaryOp::Greater, Value::Integer(b)) => Ok(Value::Boolean(a > b)),
            (Value::Integer(a), BinaryOp::LessEqual, Value::Integer(b)) => Ok(Value::Boolean(a <= b)),
            (Value::Integer(a), BinaryOp::GreaterEqual, Value::Integer(b)) => Ok(Value::Boolean(a >= b)),
            
            (Value::Float(a), BinaryOp::Less, Value::Float(b)) => Ok(Value::Boolean(a < b)),
            (Value::Float(a), BinaryOp::Greater, Value::Float(b)) => Ok(Value::Boolean(a > b)),
            (Value::Float(a), BinaryOp::LessEqual, Value::Float(b)) => Ok(Value::Boolean(a <= b)),
            (Value::Float(a), BinaryOp::GreaterEqual, Value::Float(b)) => Ok(Value::Boolean(a >= b)),
            
            // Infinity comparisons
            (Value::Integer(_), BinaryOp::Less, Value::Infinity(true)) => Ok(Value::Boolean(true)),
            (Value::Integer(_), BinaryOp::Greater, Value::Infinity(true)) => Ok(Value::Boolean(false)),
            (Value::Integer(_), BinaryOp::Greater, Value::Infinity(false)) => Ok(Value::Boolean(true)),
            (Value::Integer(_), BinaryOp::Less, Value::Infinity(false)) => Ok(Value::Boolean(false)),
            (Value::Integer(_), BinaryOp::LessEqual, Value::Infinity(true)) => Ok(Value::Boolean(true)),
            (Value::Integer(_), BinaryOp::GreaterEqual, Value::Infinity(true)) => Ok(Value::Boolean(false)),
            (Value::Integer(_), BinaryOp::GreaterEqual, Value::Infinity(false)) => Ok(Value::Boolean(true)),
            (Value::Integer(_), BinaryOp::LessEqual, Value::Infinity(false)) => Ok(Value::Boolean(false)),
            
            (Value::Float(_), BinaryOp::Less, Value::Infinity(true)) => Ok(Value::Boolean(true)),
            (Value::Float(_), BinaryOp::Greater, Value::Infinity(true)) => Ok(Value::Boolean(false)),
            (Value::Float(_), BinaryOp::Greater, Value::Infinity(false)) => Ok(Value::Boolean(true)),
            (Value::Float(_), BinaryOp::Less, Value::Infinity(false)) => Ok(Value::Boolean(false)),
            (Value::Float(_), BinaryOp::LessEqual, Value::Infinity(true)) => Ok(Value::Boolean(true)),
            (Value::Float(_), BinaryOp::GreaterEqual, Value::Infinity(true)) => Ok(Value::Boolean(false)),
            (Value::Float(_), BinaryOp::GreaterEqual, Value::Infinity(false)) => Ok(Value::Boolean(true)),
            (Value::Float(_), BinaryOp::LessEqual, Value::Infinity(false)) => Ok(Value::Boolean(false)),
            
            // Reverse infinity comparisons
            (Value::Infinity(true), BinaryOp::Greater, Value::Integer(_)) => Ok(Value::Boolean(true)),
            (Value::Infinity(true), BinaryOp::Less, Value::Integer(_)) => Ok(Value::Boolean(false)),
            (Value::Infinity(false), BinaryOp::Less, Value::Integer(_)) => Ok(Value::Boolean(true)),
            (Value::Infinity(false), BinaryOp::Greater, Value::Integer(_)) => Ok(Value::Boolean(false)),
            (Value::Infinity(true), BinaryOp::GreaterEqual, Value::Integer(_)) => Ok(Value::Boolean(true)),
            (Value::Infinity(true), BinaryOp::LessEqual, Value::Integer(_)) => Ok(Value::Boolean(false)),
            (Value::Infinity(false), BinaryOp::LessEqual, Value::Integer(_)) => Ok(Value::Boolean(true)),
            (Value::Infinity(false), BinaryOp::GreaterEqual, Value::Integer(_)) => Ok(Value::Boolean(false)),
            
            (Value::Infinity(true), BinaryOp::Greater, Value::Float(_)) => Ok(Value::Boolean(true)),
            (Value::Infinity(true), BinaryOp::Less, Value::Float(_)) => Ok(Value::Boolean(false)),
            (Value::Infinity(false), BinaryOp::Less, Value::Float(_)) => Ok(Value::Boolean(true)),
            (Value::Infinity(false), BinaryOp::Greater, Value::Float(_)) => Ok(Value::Boolean(false)),
            (Value::Infinity(true), BinaryOp::GreaterEqual, Value::Float(_)) => Ok(Value::Boolean(true)),
            (Value::Infinity(true), BinaryOp::LessEqual, Value::Float(_)) => Ok(Value::Boolean(false)),
            (Value::Infinity(false), BinaryOp::LessEqual, Value::Float(_)) => Ok(Value::Boolean(true)),
            (Value::Infinity(false), BinaryOp::GreaterEqual, Value::Float(_)) => Ok(Value::Boolean(false)),
            
            (Value::Infinity(true), BinaryOp::Greater, Value::Infinity(false)) => Ok(Value::Boolean(true)),
            (Value::Infinity(false), BinaryOp::Less, Value::Infinity(true)) => Ok(Value::Boolean(true)),
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
            
            _ => Err(format!("Unsupported binary operation: {} {:?} {}", left, op, right)),
        }
    }
    
    fn eval_unary_op(&self, op: &UnaryOp, operand: &Value) -> Result<Value, String> {
        match (op, operand) {
            (UnaryOp::Minus, Value::Integer(i)) => Ok(Value::Integer(-i)),
            (UnaryOp::Minus, Value::Float(f)) => Ok(Value::Float(-f)),
            (UnaryOp::Minus, Value::Infinity(positive)) => Ok(Value::Infinity(!positive)),
            (UnaryOp::Not, Value::Boolean(b)) => Ok(Value::Boolean(!b)),
            _ => Err(format!("Unsupported unary operation: {:?} {}", op, operand)),
        }
    }
    
    fn pattern_matches(&self, pattern: &Pattern, value: &Value) -> Result<bool, String> {
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
            (Pattern::Identifier(_), _) => Ok(true), // Identifiers always match and bind
            _ => Ok(false),
        }
    }
    
    fn values_equal(&self, a: &Value, b: &Value) -> bool {
        match (a, b) {
            (Value::Integer(a), Value::Integer(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Char(a), Value::Char(b)) => a == b,
            _ => false,
        }
    }
    
    fn is_truthy(&self, value: &Value) -> bool {
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
    
    fn get_property(&self, obj: &Value, field: &str) -> Result<Value, String> {
        match obj {
            Value::Class { name: class_name, static_fields, static_methods, .. } => {
                if field == "new" {
                    Ok(Value::Constructor(class_name.clone()))
                } else if let Some(v) = static_fields.get(field) {
                    Ok(v.clone())
                } else if let Some(v) = static_methods.get(field) {
                    Ok(v.clone())
                } else {
                    Err(format!("Unknown static field or method '{}' on class '{}'", field, class_name))
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
                let cols = if grid.is_empty() { 0 } else { grid[0].len() as i64 };
                match field {
                    "rows" => Ok(Value::Integer(rows)),
                    "cols" | "columns" => Ok(Value::Integer(cols)),
                    "len" | "length" | "size" => Ok(Value::Integer(rows * cols)),
                    "neighbors" => Ok(Value::GridNeighbors(Box::new(Value::Grid(grid.clone())))),
                    _ => Err(format!("Grid method '{}' not found", field)),
                }
            }
            Value::Counter(counter) => {
                match field {
                    "most_common" => {
                        let mut items: Vec<_> = counter.iter().collect();
                        items.sort_by(|a, b| b.1.cmp(a.1));
                        let result: Vec<Value> = items.iter()
                            .map(|(k, v)| Value::Tuple(vec![Value::String(k.to_string()), Value::Integer(**v)]))
                            .collect();
                        Ok(Value::List(result))
                    }
                    "elements" | "keys" => {
                        let keys: Vec<Value> = counter.keys().map(|k| Value::String(k.clone())).collect();
                        Ok(Value::List(keys))
                    }
                    "total" => Ok(Value::Integer(counter.values().sum())),
                    "len" | "length" | "size" => Ok(Value::Integer(counter.len() as i64)),
                    _ => Err(format!("Counter method '{}' not found", field)),
                }
            }
            _ => Err(format!("Cannot get property '{}' on non-object", field)),
        }
    }

    fn get_instance_method(&self, class_name: &str, method_name: &str) -> Result<Value, String> {
        let class_val = self.get_variable(class_name)?;
        if let Value::Class { methods, .. } = class_val {
            methods.get(method_name).cloned()
                .ok_or_else(|| format!("Unknown method '{}' on class '{}'", method_name, class_name))
        } else {
            Err(format!("'{}' is not a class", class_name))
        }
    }

    fn call_value(&mut self, callee: Value, args: &[AstNode], this_opt: Option<Value>) -> Result<Value, String> {
        let eval_args: Vec<Value> = args.iter().map(|a| self.eval_node(a)).collect::<Result<Vec<_>, _>>()?;
        match callee {
            Value::Constructor(class_name) => {
                let class_val = self.get_variable(&class_name)?;
                let Value::Class { name: _, parent: _, fields: class_fields, methods, static_fields: _, static_methods: _ } = class_val else {
                    return Err(format!("'{}' is not a class", class_name));
                };
                let instance_fields = class_fields.clone();
                let instance = Value::Instance {
                    class_name: class_name.clone(),
                    fields: instance_fields.clone(),
                };
                if let Some(init) = methods.get("init") {
                    if let Value::Function { params, body, .. } = init {
                        self.call_function_internal("init", &eval_args, params, body, Some(instance.clone()))?;
                    }
                }
                Ok(instance)
            }
            Value::Function { name, params, body } => {
                self.call_function_internal(&name, &eval_args, &params, &*body, this_opt)
            }
            Value::GridNeighbors(grid_val) => {
                let Value::Grid(grid) = grid_val.as_ref() else {
                    return Err("GridNeighbors requires a grid".to_string());
                };
                if eval_args.len() != 2 {
                    return Err("grid.neighbors(i, j) requires exactly 2 arguments (row, col)".to_string());
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
            _ => Err(format!("Cannot call {} as function", callee)),
        }
    }

    fn get_variable(&self, name: &str) -> Result<Value, String> {
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
    
    fn set_variable(&mut self, name: String, value: Value) {
        if let Some(scope) = self.locals.last_mut() {
            scope.insert(name, value);
        } else {
            self.globals.insert(name, value);
        }
    }
    
    fn push_scope(&mut self) {
        self.locals.push(HashMap::new());
    }
    
    fn pop_scope(&mut self) {
        self.locals.pop();
    }
    
    fn execute_file(&mut self, filename: &str) -> Result<Value, String> {
        // Read the file
        let source = std::fs::read_to_string(filename)
            .map_err(|e| format!("Error reading file '{}': {}", filename, e))?;
        
        println!("🔥 Executing J file: {}", filename);
        
        // Tokenize
        let mut lexer = crate::lexer::Lexer::new(&source);
        let tokens = lexer.tokenize()
            .map_err(|e| format!("Lexer error in '{}': {}", filename, e))?;
        
        // Parse
        let mut parser = crate::parser::Parser::new(tokens);
        let ast = parser.parse()
            .map_err(|e| format!("Parser error in '{}': {}", filename, e))?;
        
        // Execute in current context
        self.eval_node(&ast)
    }
    
    fn normalize_slice_indices(&self, start: Option<i64>, end: Option<i64>, len: i64, step: i64) -> Result<(i64, i64), String> {
        let start = match start {
            Some(s) => {
                if s < 0 {
                    (len + s).max(0)
                } else {
                    s.min(len)
                }
            }
            None => {
                if step > 0 { 0 } else { len - 1 }
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
                if step > 0 { len } else { -1 }
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
        for i in 0..=len1 {
            matrix[i][0] = i;
        }
        for j in 0..=len2 {
            matrix[0][j] = j;
        }
        
        // Fill the matrix
        for i in 1..=len1 {
            for j in 1..=len2 {
                let cost = if s1.chars().nth(i-1) == s2.chars().nth(j-1) { 0 } else { 1 };
                matrix[i][j] = (matrix[i-1][j] + 1)
                    .min(matrix[i][j-1] + 1)
                    .min(matrix[i-1][j-1] + cost);
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
            } else {
                if len != 0 {
                    len = lps[len - 1];
                } else {
                    lps[i] = 0;
                    i += 1;
                }
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
            if coords[i].1 < coords[min_idx].1 || 
               (coords[i].1 == coords[min_idx].1 && coords[i].0 < coords[min_idx].0) {
                min_idx = i;
            }
        }
        coords.swap(0, min_idx);
        
        // Sort points by polar angle with respect to bottom point
        let pivot = coords[0];
        coords[1..].sort_by(|a, b| {
            let angle_a = (a.1 - pivot.1).atan2(a.0 - pivot.0);
            let angle_b = (b.1 - pivot.1).atan2(b.0 - pivot.0);
            angle_a.partial_cmp(&angle_b).unwrap_or(std::cmp::Ordering::Equal)
        });
        
        // Graham scan
        let mut hull = vec![0];
        for i in 1..coords.len() {
            while hull.len() > 1 && 
                  self.cross_product(&coords[hull[hull.len()-2]], &coords[hull[hull.len()-1]], &coords[i]) <= 0.0 {
                hull.pop();
            }
            hull.push(i);
        }
        
        // Convert back to Value tuples
        Ok(hull.into_iter().map(|i| {
            let (x, y) = coords[i];
            Value::Tuple(vec![Value::Float(x), Value::Float(y)])
        }).collect())
    }
    
    fn cross_product(&self, o: &(f64, f64), a: &(f64, f64), b: &(f64, f64)) -> f64 {
        (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0)
    }
    
    // BFS (Breadth-First Search)
    fn bfs_search(&self, graph: &HashMap<String, Vec<(String, f64)>>, start: &str, goal: Option<&str>) -> Result<Vec<String>, String> {
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
    
    // DFS (Depth-First Search)
    fn dfs_search(&self, graph: &HashMap<String, Vec<(String, f64)>>, start: &str, goal: Option<&str>) -> Result<Vec<String>, String> {
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
    
    fn dfs_recursive(&self, graph: &HashMap<String, Vec<(String, f64)>>, current: &str, goal: Option<&str>, 
                     visited: &mut HashSet<String>, path: &mut Vec<String>) -> bool {
        visited.insert(current.to_string());
        path.push(current.to_string());
        
        if let Some(target) = goal {
            if current == target {
                return true;
            }
        }
        
        if let Some(neighbors) = graph.get(current) {
            for (neighbor, _) in neighbors {
                if !visited.contains(neighbor) {
                    if self.dfs_recursive(graph, neighbor, goal, visited, path) {
                        return true;
                    }
                }
            }
        }
        
        if goal.is_some() {
            path.pop();
        }
        false
    }
    
    // Dijkstra's shortest path algorithm
    fn dijkstra_search(&self, graph: &HashMap<String, Vec<(String, f64)>>, start: &str, goal: Option<&str>) -> Result<(Vec<String>, f64), String> {
        use std::collections::BinaryHeap;
        use std::cmp::Reverse;
        
        // Use a wrapper for f64 that implements Ord
        #[derive(PartialEq, PartialOrd)]
        struct FloatOrd(f64);
        impl Eq for FloatOrd {}
        impl Ord for FloatOrd {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.0.partial_cmp(&other.0).unwrap_or(std::cmp::Ordering::Equal)
            }
        }
        
        let mut dist: HashMap<String, f64> = HashMap::new();
        let mut prev: HashMap<String, String> = HashMap::new();
        let mut heap: BinaryHeap<Reverse<(FloatOrd, String)>> = BinaryHeap::new();
        
        dist.insert(start.to_string(), 0.0);
        heap.push(Reverse((FloatOrd(0.0), start.to_string())));
        
        while let Some(Reverse((FloatOrd(d), u))) = heap.pop() {
            if let Some(target) = goal {
                if u == target {
                    // Reconstruct path
                    let mut path = Vec::new();
                    let mut node = u.clone();
                    while let Some(p) = prev.get(&node) {
                        path.push(node.clone());
                        node = p.clone();
                    }
                    path.push(start.to_string());
                    path.reverse();
                    return Ok((path, d));
                }
            }
            
            if d > *dist.get(&u).unwrap_or(&f64::INFINITY) {
                continue;
            }
            
            if let Some(neighbors) = graph.get(&u) {
                for (v, weight) in neighbors {
                    let alt = d + weight;
                    if alt < *dist.get(v).unwrap_or(&f64::INFINITY) {
                        dist.insert(v.clone(), alt);
                        prev.insert(v.clone(), u.clone());
                        heap.push(Reverse((FloatOrd(alt), v.clone())));
                    }
                }
            }
        }
        
        if goal.is_some() {
            Err("Goal not reachable".to_string())
        } else {
            // Return all reachable nodes with distances
            let mut result: Vec<(String, f64)> = dist.into_iter().collect();
            result.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            let path: Vec<String> = result.into_iter().map(|(n, _)| n).collect();
            Ok((path, 0.0))
        }
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
            
            for j in 0..n {
                let angle = -2.0 * std::f64::consts::PI * (k as f64) * (j as f64) / (n as f64);
                real += samples[j] * angle.cos();
                imag += samples[j] * angle.sin();
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
    
    // Helper functions for enhanced out() features
    
    fn format_string(&self, format_str: &str, vars: &HashMap<String, Value>) -> Result<String, String> {
        let mut result = String::new();
        let mut i = 0;
        let chars: Vec<char> = format_str.chars().collect();
        
        while i < chars.len() {
            if chars[i] == '{' && i + 1 < chars.len() {
                if chars[i + 1] == '}' {
                    // Empty placeholder - skip
                    i += 2;
                    continue;
                }
                
                // Find closing brace
                let mut j = i + 1;
                let mut found = false;
                while j < chars.len() {
                    if chars[j] == '}' {
                        found = true;
                        break;
                    }
                    j += 1;
                }
                
                if found {
                    let var_name: String = chars[i + 1..j].iter().collect();
                    // Check for format specifiers like {name:>8} or {value:.2f}
                    let parts: Vec<&str> = var_name.split(':').collect();
                    let key = parts[0].trim();
                    
                    if let Some(value) = vars.get(key) {
                        let formatted = if parts.len() > 1 {
                            // Has format specifier
                            let spec = parts[1].trim();
                            if spec.starts_with('>') {
                                // Right align
                                let width: usize = spec[1..].parse().unwrap_or(0);
                                format!("{:>width$}", value, width = width)
                            } else if spec.starts_with('<') {
                                // Left align
                                let width: usize = spec[1..].parse().unwrap_or(0);
                                format!("{:<width$}", value, width = width)
                            } else if spec.starts_with(".2f") || spec.starts_with(".1f") {
                                // Float formatting
                                if let Value::Float(f) = value {
                                    format!("{:.2}", f)
                                } else {
                                    value.to_string()
                                }
                            } else {
                                value.to_string()
                            }
                        } else {
                            value.to_string()
                        };
                        result.push_str(&formatted);
                    } else {
                        // Variable not found, try to get from scope
                        match self.get_variable(key) {
                            Ok(val) => result.push_str(&val.to_string()),
                            Err(_) => result.push_str(&format!("{{{}}}", key)), // Keep placeholder if not found
                        }
                    }
                    i = j + 1;
                } else {
                    result.push(chars[i]);
                    i += 1;
                }
            } else {
                result.push(chars[i]);
                i += 1;
            }
        }
        
        Ok(result)
    }
    
    fn print_table(&self, rows: &[Value]) -> Result<(), String> {
        // Convert rows to string matrix
        let mut table: Vec<Vec<String>> = Vec::new();
        for row in rows {
            if let Value::List(cells) = row {
                let mut row_strs = Vec::new();
                for cell in cells {
                    row_strs.push(cell.to_string());
                }
                table.push(row_strs);
            } else {
                return Err("Table rows must be lists".to_string());
            }
        }
        
        if table.is_empty() {
            return Ok(());
        }
        
        // Calculate column widths
        let num_cols = table[0].len();
        let mut col_widths = vec![0; num_cols];
        
        for row in &table {
            for (i, cell) in row.iter().enumerate() {
                if i < col_widths.len() {
                    col_widths[i] = col_widths[i].max(cell.len());
                }
            }
        }
        
        // Print table
        for (row_idx, row) in table.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                if col_idx < col_widths.len() {
                    print!("{:width$}", cell, width = col_widths[col_idx]);
                    if col_idx < row.len() - 1 {
                        print!("  "); // Column separator
                    }
                }
            }
            println!();
            
            // Print header separator after first row
            if row_idx == 0 {
                for &width in &col_widths {
                    print!("{}", "-".repeat(width));
                    if width < col_widths.len() {
                        print!("  ");
                    }
                }
                println!();
            }
        }
        
        Ok(())
    }
    
    fn print_table_with_options(&self, rows: &[Value], options: &HashMap<String, Value>) -> Result<(), String> {
        // Similar to print_table but with alignment and color options
        let mut table: Vec<Vec<String>> = Vec::new();
        for row in rows {
            if let Value::List(cells) = row {
                let mut row_strs = Vec::new();
                for cell in cells {
                    row_strs.push(cell.to_string());
                }
                table.push(row_strs);
            } else {
                return Err("Table rows must be lists".to_string());
            }
        }
        
        if table.is_empty() {
            return Ok(());
        }
        
        let num_cols = table[0].len();
        let mut col_widths = vec![0; num_cols];
        
        for row in &table {
            for (i, cell) in row.iter().enumerate() {
                if i < col_widths.len() {
                    col_widths[i] = col_widths[i].max(cell.len());
                }
            }
        }
        
        // Get alignment option
        let align = if let Some(Value::String(align_str)) = options.get("align") {
            align_str.as_str()
        } else {
            "left"
        };
        
        // Get color option
        let color = if let Some(Value::String(color_str)) = options.get("color") {
            Some(color_str.as_str())
        } else {
            None
        };
        
        let header = if let Some(Value::Boolean(h)) = options.get("header") {
            *h
        } else {
            false
        };
        
        // Apply color if specified
        if let Some(color_name) = color {
            match color_name {
                "red" => print!("\x1b[31m"),
                "green" => print!("\x1b[32m"),
                "yellow" => print!("\x1b[33m"),
                "blue" => print!("\x1b[34m"),
                "magenta" => print!("\x1b[35m"),
                "cyan" => print!("\x1b[36m"),
                "white" => print!("\x1b[37m"),
                _ => {}
            }
        }
        
        for (row_idx, row) in table.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                if col_idx < col_widths.len() {
                    match align {
                        "right" => print!("{:>width$}", cell, width = col_widths[col_idx]),
                        "center" => {
                            let padding = col_widths[col_idx] - cell.len();
                            let left_pad = padding / 2;
                            let right_pad = padding - left_pad;
                            print!("{}{}{}", " ".repeat(left_pad), cell, " ".repeat(right_pad));
                        }
                        _ => print!("{:<width$}", cell, width = col_widths[col_idx]),
                    }
                    if col_idx < row.len() - 1 {
                        print!("  ");
                    }
                }
            }
            println!();
            
            if header && row_idx == 0 {
                for &width in &col_widths {
                    print!("{}", "-".repeat(width));
                    if width < col_widths.len() {
                        print!("  ");
                    }
                }
                println!();
            }
        }
        
        if color.is_some() {
            print!("\x1b[0m"); // Reset color
        }
        
        Ok(())
    }
    
    fn print_progress_bar(&self, percent: f64, width: usize, color: &str) -> Result<(), String> {
        let clamped_percent = percent.max(0.0).min(100.0);
        let filled = ((clamped_percent / 100.0) * width as f64) as usize;
        let empty = width - filled;
        
        // Apply color
        match color {
            "red" => print!("\x1b[31m"),
            "green" => print!("\x1b[32m"),
            "yellow" => print!("\x1b[33m"),
            "blue" => print!("\x1b[34m"),
            "magenta" => print!("\x1b[35m"),
            "cyan" => print!("\x1b[36m"),
            "white" => print!("\x1b[37m"),
            _ => {}
        }
        
        print!("[");
        for _ in 0..filled {
            print!("█");
        }
        if filled < width {
            // Use half block for partial
            if (clamped_percent / 100.0 * width as f64) - filled as f64 > 0.5 {
                print!("▌");
                for _ in 0..(empty - 1) {
                    print!(" ");
                }
            } else {
                for _ in 0..empty {
                    print!(" ");
                }
            }
        }
        print!("] {:.0}%", clamped_percent);
        print!("\x1b[0m"); // Reset color
        println!();
        
        Ok(())
    }
    
    fn print_animation(&self, text: &str, anim_type: &str, interval: f64, count: Option<usize>) -> Result<(), String> {
        let frames: Vec<&str> = match anim_type {
            "spinner" => vec!["|", "/", "-", "\\"],
            "dots" => vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
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
                // Bounce animation: ••••• → • •••• → •• ••• etc.
                let mut frames = Vec::new();
                for i in 0..5 {
                    let mut frame = String::new();
                    for j in 0..5 {
                        if j == i {
                            frame.push(' ');
                        } else {
                            frame.push('•');
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
    
    fn print_gradient(&self, text: &str, gradient_colors: &[Value]) -> Result<(), String> {
        if gradient_colors.len() < 2 {
            return Err("Gradient requires at least 2 colors".to_string());
        }
        
        // Extract hex colors
        let mut colors: Vec<(u8, u8, u8)> = Vec::new();
        for color_val in gradient_colors {
            if let Value::String(hex_str) = color_val {
                let hex = hex_str.trim_start_matches('#');
                if hex.len() == 6 {
                    if let (Ok(r), Ok(g), Ok(b)) = (
                        u8::from_str_radix(&hex[0..2], 16),
                        u8::from_str_radix(&hex[2..4], 16),
                        u8::from_str_radix(&hex[4..6], 16),
                    ) {
                        colors.push((r, g, b));
                    }
                }
            }
        }
        
        if colors.len() < 2 {
            return Err("Gradient colors must be hex strings".to_string());
        }
        
        // Interpolate colors across text
        let chars: Vec<char> = text.chars().collect();
        for (i, ch) in chars.iter().enumerate() {
            let t = if chars.len() > 1 {
                i as f64 / (chars.len() - 1) as f64
            } else {
                0.0
            };
            
            // Find which color segment we're in
            let segment_size = 1.0 / (colors.len() - 1) as f64;
            let segment = (t / segment_size).min((colors.len() - 2) as f64) as usize;
            let local_t = (t - segment as f64 * segment_size) / segment_size;
            
            let (r1, g1, b1) = colors[segment];
            let (r2, g2, b2) = colors[segment + 1];
            
            let r = (r1 as f64 + (r2 as f64 - r1 as f64) * local_t) as u8;
            let g = (g1 as f64 + (g2 as f64 - g1 as f64) * local_t) as u8;
            let b = (b1 as f64 + (b2 as f64 - b1 as f64) * local_t) as u8;
            
            // Print character with RGB color
            print!("\x1b[38;2;{};{};{}m{}", r, g, b, ch);
        }
        
        print!("\x1b[0m"); // Reset
        println!();
        
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
            "dots" => vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
            "line" => vec!["-", "\\", "|", "/"],
            "arrow" => vec!["←", "↖", "↑", "↗", "→", "↘", "↓", "↙"],
            "bounce" => vec!["⠁", "⠂", "⠄", "⠂"],
            "box" => vec!["◰", "◳", "◲", "◱"],
            "circle" => vec!["◐", "◓", "◑", "◒"],
            "square" => vec!["◰", "◳", "◲", "◱"],
            "triangle" => vec!["◢", "◣", "◤", "◥"],
            "pulse" => vec!["●", "○", "●", "○"],
            "grow" => vec!["▁", "▃", "▅", "▇", "█", "▇", "▅", "▃"],
            _ => vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
        };
        
        let iterations = (duration * 10.0) as usize;
        
        for i in 0..iterations {
            let frame = frames[i % frames.len()];
            print!("\r\x1b[36m{}\x1b[0m {}", frame, message);
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(100));
        }
        
        println!("\r\x1b[32m✓\x1b[0m {} \x1b[32mDone!\x1b[0m", message);
        Ok(())
    }
    
    fn show_loading(&self, message: &str, style: &str, duration: f64) -> Result<(), String> {
        self.show_spinner(style, duration, message)
    }
    
    fn show_panel(&self, text: &str, title: Option<&str>, style: &str) -> Result<(), String> {
        let (top_left, top_right, bottom_left, bottom_right, horizontal, vertical) = match style {
            "double" => ("╔", "╗", "╚", "╝", "═", "║"),
            "rounded" => ("╭", "╮", "╰", "╯", "─", "│"),
            "bold" => ("┏", "┓", "┗", "┛", "━", "┃"),
            "ascii" => ("+", "+", "+", "+", "-", "|"),
            _ => ("┌", "┐", "└", "┘", "─", "│"), // single
        };
        
        let lines: Vec<&str> = text.lines().collect();
        let max_width = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);
        let width = max_width.max(20);
        
        // Top border
        if let Some(t) = title {
            let title_len = t.chars().count();
            let left_pad = (width - title_len) / 2;
            let right_pad = width - title_len - left_pad;
            println!("{}{}{} {} {}{}{}", 
                top_left, 
                horizontal.repeat(left_pad),
                "\x1b[1m",
                t,
                "\x1b[0m",
                horizontal.repeat(right_pad),
                top_right
            );
        } else {
            println!("{}{}{}", top_left, horizontal.repeat(width + 2), top_right);
        }
        
        // Content - properly align with character count, not byte length
        for line in lines {
            let line_width = line.chars().count();
            let padding = if width > line_width {
                width - line_width
            } else {
                0
            };
            println!("{} {}{} {}", vertical, line, " ".repeat(padding), vertical);
        }
        
        // Bottom border
        println!("{}{}{}", bottom_left, horizontal.repeat(width + 2), bottom_right);
        
        Ok(())
    }
    
    fn show_box(&self, text: &str, width: usize) -> Result<(), String> {
        let lines: Vec<&str> = text.lines().collect();
        
        println!("┌{}┐", "─".repeat(width));
        for line in lines {
            let padding = if line.len() < width {
                width - line.len()
            } else {
                0
            };
            println!("│ {}{} │", line, " ".repeat(padding));
        }
        println!("└{}┘", "─".repeat(width));
        
        Ok(())
    }
    
    fn show_status(&self, status_type: &str, message: &str) -> Result<(), String> {
        let (icon, color) = match status_type {
            "success" | "ok" => ("✓", "32"), // Green
            "error" | "fail" => ("✗", "31"), // Red
            "warning" | "warn" => ("⚠", "33"), // Yellow
            "info" => ("ℹ", "36"), // Cyan
            "question" => ("?", "35"), // Magenta
            _ => ("•", "37"), // White
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
                    let prefix = if is_last { "└─" } else { "├─" };
                    
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
                    let prefix = if is_last { "└─" } else { "├─" };
                    
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
                format!("{}…", &text[..col_width - 3])
            } else {
                text.clone()
            };
            print!("{:width$}", truncated, width = col_width);
        }
        println!();
        
        Ok(())
    }
}
