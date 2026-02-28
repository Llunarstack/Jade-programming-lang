//! Tree-walk interpreter for the Jade programming language.
//!
//! Evaluates the AST produced by the parser in a single pass, with a global
//! environment and local scopes for functions and blocks.

use crate::error::JError;
use crate::parser::{AstNode, BinaryOp, Pattern, UnaryOp};
use std::collections::{HashMap, HashSet};
use std::io::Write;

mod value;
mod eval;
mod eval_conditionals;
mod eval_control;
mod eval_primitive;
mod eval_out;
mod call;
mod builtins;
pub(crate) mod crypto;
pub use value::*;

pub struct Interpreter {
    globals: HashMap<String, Value>,
    locals: Vec<HashMap<String, Value>>,
    statics: HashMap<String, Value>, // Static variables
    call_depth: usize,               // Track recursion depth
    defer_stack: Vec<Vec<(AstNode, Option<Value>)>>, // (expr, value for _) when block exits (LIFO)
    once_cache: HashMap<usize, Value>, // @once decorator cached results
    once_next_id: usize,
    // Module system
    module_cache: HashMap<String, Value>,
    module_search_paths: Vec<String>,
    // Trait system
    #[allow(dead_code)]
    trait_impls: HashMap<String, HashMap<String, Value>>, // type_name -> trait_name -> impl
    // Async system
    next_future_id: usize,
    // Advanced class types
    singleton_registry: HashMap<String, Value>, // class_name -> singleton instance
    resource_stack: Vec<(String, Value)>,       // (class_name, instance) for RAII cleanup
    /// When set, out() writes here instead of stdout (e.g. when capturing output).
    pub(crate) output_buffer: Option<Vec<u8>>,
    /// Memoization caches: id -> (args_key -> result).
    pub(crate) memo_caches: std::collections::HashMap<usize, std::collections::HashMap<String, Value>>,
    pub(crate) next_memo_id: usize,
    /// Arena pools for pool | name -> pool(); pool_id -> storage vec
    pub(crate) pools: std::collections::HashMap<usize, Vec<Value>>,
    #[allow(dead_code)]
    pub(crate) next_pool_id: usize,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut interpreter = Self {
            globals: HashMap::new(),
            locals: Vec::new(),
            statics: HashMap::new(),
            call_depth: 0,
            defer_stack: Vec::new(),
            once_cache: HashMap::new(),
            once_next_id: 0,
            // Module system
            module_cache: HashMap::new(),
            module_search_paths: vec![".".to_string()],
            // Trait system
            trait_impls: HashMap::new(),
            // Async system
            next_future_id: 0,
            // Advanced class types
            singleton_registry: HashMap::new(),
            resource_stack: Vec::new(),
            output_buffer: None,
            memo_caches: std::collections::HashMap::new(),
            next_memo_id: 0,
            pools: std::collections::HashMap::new(),
            next_pool_id: 0,
        };

        // Add built-in functions
        interpreter.add_builtins();
        interpreter
    }

    fn add_builtins(&mut self) {
        use std::f64::consts::PI;
        self.globals.insert("PI".to_string(), Value::Float(PI));
        self.globals.insert("TAU".to_string(), Value::Float(PI * 2.0));
        self.globals.insert("E".to_string(), Value::Float(std::f64::consts::E));
        self.globals.insert("PHI".to_string(), Value::Float((1.0 + 5.0_f64.sqrt()) / 2.0));
        self.globals.insert("SQRT2".to_string(), Value::Float(2.0_f64.sqrt()));
        self.globals.insert("date".to_string(), Value::DateType);
        self.globals.insert("time".to_string(), Value::TimeType);
        self.globals.insert("datetime".to_string(), Value::DateTimeType);
        self.globals.insert("graph".to_string(), Value::GraphType);
    }

    fn apply_decorator(
        &mut self,
        decorator_name: &str,
        decorator_args: &[crate::parser::AstNode],
        func: Value,
    ) -> Result<Value, String> {
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
            return self.call_function_internal(decorator_name, &call_args, &params, &body, None);
        }

        // Built-in decorators (from j.txt and jnew_features.txt)
        match decorator_name {
            "memo" | "cache" => Ok(self.create_memoized_function(func)?),
            "timer" => Ok(self.create_timed_function(func)?),
            "log_call" => Ok(self.create_logged_function(func)?),
            "tco" => {
                // Tail-call optimization hint - pass through (interpreter may optimize later)
                Ok(func)
            }
            "once" => Ok(self.create_once_function(func)?),
            "throttle" => {
                let interval_secs = args
                    .first()
                    .and_then(|a| {
                        if let Value::Float(f) = a {
                            Some(*f)
                        } else {
                            None
                        }
                    })
                    .unwrap_or(0.5);
                Ok(self.create_throttled_function(func, interval_secs)?)
            }
            "debounce" => {
                let delay_secs = args
                    .first()
                    .and_then(|a| {
                        if let Value::Float(f) = a {
                            Some(*f)
                        } else {
                            None
                        }
                    })
                    .unwrap_or(0.3);
                Ok(self.create_debounced_function(func, delay_secs)?)
            }
            "profile" => Ok(self.create_profiled_function(func)?),
            "trace" => Ok(self.create_trace_function(func)?),
            "validate_args" | "validate" => {
                // Schema-based validation - pass through for now; user can define validator fn
                Ok(func)
            }
            "deprecated" => Ok(self.create_deprecated_function(func, &args)?),
            _ => Err(format!("Decorator '{}' not found", decorator_name)),
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
        // For now, return the function as-is; full logging would wrap the call
        Ok(func)
    }

    fn create_once_function(&mut self, func: Value) -> Result<Value, String> {
        self.once_next_id += 1;
        let id = self.once_next_id;
        Ok(Value::OnceCached {
            id,
            inner: Box::new(func),
        })
    }

    fn create_throttled_function(
        &mut self,
        func: Value,
        _interval_secs: f64,
    ) -> Result<Value, String> {
        // @throttle(sec): max one call per interval - stub
        Ok(func)
    }

    fn create_debounced_function(
        &mut self,
        func: Value,
        _delay_secs: f64,
    ) -> Result<Value, String> {
        // @debounce(sec): delay until silence - stub
        Ok(func)
    }

    fn create_profiled_function(&mut self, func: Value) -> Result<Value, String> {
        // @profile: time + call count - stub
        Ok(func)
    }

    fn create_trace_function(&mut self, func: Value) -> Result<Value, String> {
        // @trace: verbose entry/exit - stub
        Ok(func)
    }

    fn create_deprecated_function(&mut self, func: Value, args: &[Value]) -> Result<Value, String> {
        // @deprecated("message") - stub; could warn on call
        let _msg = args
            .first()
            .map(|v| v.to_string())
            .unwrap_or_else(|| "deprecated".to_string());
        Ok(func)
    }

    fn call_function_internal(
        &mut self,
        _func_name: &str,
        args: &[Value],
        params: &[String],
        body: &AstNode,
        this_value: Option<Value>,
    ) -> Result<Value, String> {
        // Check depth BEFORE incrementing to prevent stack overflow
        if self.call_depth >= 50 {
            return Err(JError::stack_overflow(self.call_depth).to_string());
        }

        self.call_depth += 1;

        // Push new scope
        self.locals.push(HashMap::new());

        // Bind this/self if method call
        if let Some(ref this_val) = this_value {
            if let Some(scope) = self.locals.last_mut() {
                scope.insert("this".to_string(), this_val.clone());
                scope.insert("self".to_string(), this_val.clone());
            }
        }

        // Bind parameters
        for (i, param) in params.iter().enumerate() {
            if i < args.len() {
                if let Some(scope) = self.locals.last_mut() {
                    scope.insert(param.clone(), args[i].clone());
                }
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
        let tokens = lexer
            .tokenize()
            .map_err(|e| format!("Lexer error: {}", e))?;

        // Parse
        let mut parser = crate::parser::Parser::new(tokens);
        let ast = parser.parse().map_err(|e| format!("Parser error: {}", e))?;

        // Interpret
        self.eval_node(&ast)
            .map_err(|e| format!("Runtime error: {}", e))?;

        Ok(())
    }

    /// Capture out() to an internal buffer instead of stdout.
    pub fn set_output_capture(&mut self, capture: bool) {
        self.output_buffer = if capture {
            Some(Vec::new())
        } else {
            None
        };
    }

    /// Take captured output (after run with set_output_capture(true)). Returns None if not capturing.
    pub fn take_captured_output(&mut self) -> Option<String> {
        self.output_buffer
            .take()
            .and_then(|b| String::from_utf8(b).ok())
    }

    /// Register a module in the cache so that `import` can resolve it without file I/O.
    pub fn register_module(&mut self, path: impl Into<String>, module: Value) {
        self.module_cache.insert(path.into(), module);
    }

    /// Add a directory to the module search path (for `import`). Used by the CLI when running from a project.
    pub fn add_module_search_path(&mut self, path: impl Into<String>) {
        self.module_search_paths.push(path.into());
    }

    /// List global variable names (for REPL .vars). Sorted.
    pub fn global_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.globals.keys().cloned().collect();
        names.sort();
        names
    }

    /// Get a global value by name (for REPL inspection).
    pub fn get_global(&self, name: &str) -> Option<Value> {
        self.globals.get(name).cloned()
    }

    #[inline]
    pub(crate) fn write_out(&mut self, s: &str) {
        if let Some(ref mut b) = self.output_buffer {
            b.extend_from_slice(s.as_bytes());
        } else {
            let _ = std::io::stdout().write_all(s.as_bytes()).and_then(|_| std::io::stdout().flush());
        }
    }

    #[inline]
    pub(crate) fn write_out_ln(&mut self, s: &str) {
        if let Some(ref mut b) = self.output_buffer {
            b.extend_from_slice(s.as_bytes());
            b.push(b'\n');
        } else {
            let _ = std::io::stdout().write_all(s.as_bytes()).and_then(|_| std::io::stdout().write_all(b"\n")).and_then(|_| std::io::stdout().flush());
        }
    }

    pub fn evaluate(&mut self, node: &AstNode) -> Result<String, String> {
        let value = self.eval_node(node)?;
        match value {
            Value::None => Ok(String::new()),
            _ => Ok(value.to_string()),
        }
    }
}
