//! JIT execution and hot-reload for J (optional `jit` feature).

#[cfg(feature = "jit")]
use notify::{EventKind, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
#[cfg(feature = "jit")]
use std::sync::mpsc;
use std::time::SystemTime;

use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;

#[allow(dead_code)]
pub struct JitCompiler {
    interpreter: Interpreter,
    file_cache: HashMap<PathBuf, (SystemTime, String)>,
    hot_reload: bool,
}

#[allow(dead_code)]
impl JitCompiler {
    pub fn new() -> Self {
        Self {
            interpreter: Interpreter::new(),
            file_cache: HashMap::new(),
            hot_reload: false,
        }
    }

    /// Add a directory to the interpreter's module search path (for `import`).
    pub fn add_module_search_path(&mut self, path: impl Into<String>) {
        self.interpreter.add_module_search_path(path);
    }

    pub fn run_file(&mut self, file_path: PathBuf, hot_reload: bool) -> Result<(), String> {
        self.hot_reload = hot_reload;

        // Initial execution
        self.execute_file(&file_path)?;

        if hot_reload {
            #[cfg(feature = "jit")]
            self.start_hot_reload(file_path)?;
            #[cfg(not(feature = "jit"))]
            return Err("Hot reload requires the 'jit' feature to be enabled".to_string());
        }

        Ok(())
    }

    fn execute_file(&mut self, file_path: &PathBuf) -> Result<(), String> {
        let content =
            fs::read_to_string(file_path).map_err(|e| format!("Failed to read file: {}", e))?;

        // Check if file has changed
        let metadata =
            fs::metadata(file_path).map_err(|e| format!("Failed to get file metadata: {}", e))?;

        let modified = metadata
            .modified()
            .map_err(|e| format!("Failed to get modification time: {}", e))?;

        // Check cache
        if let Some((cached_time, cached_content)) = self.file_cache.get(file_path) {
            if *cached_time >= modified && *cached_content == content {
                // File hasn't changed, skip recompilation
                return Ok(());
            }
        }

        // Tokenize
        let mut lexer = Lexer::new(&content);
        let tokens = lexer
            .tokenize()
            .map_err(|e| format!("Lexer error in {}: {}", file_path.display(), e))?;

        // Parse
        let mut parser = Parser::new(tokens);
        let ast = parser
            .parse()
            .map_err(|e| format!("Parser error in {}: {}", file_path.display(), e))?;

        // JIT compile and execute
        match self.interpreter.evaluate(&ast) {
            Ok(result) => {
                if !result.is_empty() {
                    println!("{}", result);
                }

                // Update cache
                self.file_cache
                    .insert(file_path.clone(), (modified, content));
            }
            Err(e) => {
                eprintln!("Runtime error: {}", e);
                return Err(e);
            }
        }

        Ok(())
    }

    #[cfg(feature = "jit")]
    fn start_hot_reload(&mut self, file_path: PathBuf) -> Result<(), String> {
        let (tx, rx) = mpsc::channel();

        let mut watcher = notify::recommended_watcher(tx)
            .map_err(|e| format!("Failed to create file watcher: {}", e))?;

        let watch_dir = file_path
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."));
        watcher
            .watch(watch_dir, RecursiveMode::Recursive)
            .map_err(|e| format!("Failed to watch directory: {}", e))?;

        println!("Watching for changes... Press Ctrl+C to stop");

        loop {
            match rx.recv() {
                Ok(Ok(event)) => {
                    let is_write_or_create = matches!(
                        event.kind,
                        EventKind::Create(_) | EventKind::Modify(_)
                    );
                    if !is_write_or_create {
                        continue;
                    }
                    for path in &event.paths {
                        if path.extension().map(|s| s == "jdl").unwrap_or(false)
                            || path.extension().map(|s| s == "j").unwrap_or(false)
                        {
                            println!("\nFile changed: {}", path.display());
                            if let Err(e) = self.execute_file(path) {
                                eprintln!("Hot reload failed: {}", e);
                            }
                            break;
                        }
                    }
                }
                Ok(Err(e)) => {
                    eprintln!("Watch error: {:?}", e);
                    break;
                }
                Err(_) => break,
            }
        }

        Ok(())
    }

    pub fn compile_to_bytecode(&mut self, source: &str) -> Result<Vec<u8>, String> {
        // This would compile to bytecode for faster execution
        // For now, we'll just return a placeholder

        let mut lexer = Lexer::new(source);
        let tokens = lexer
            .tokenize()
            .map_err(|e| format!("Lexer error: {}", e))?;

        let mut parser = Parser::new(tokens);
        let _ast = parser.parse().map_err(|e| format!("Parser error: {}", e))?;

        // TODO: Implement bytecode generation
        Ok(vec![0x4A, 0x49, 0x54]) // "JIT" in bytes as placeholder
    }

    pub fn optimize_hot_path(&mut self, function_name: &str) -> Result<(), String> {
        // This would optimize frequently called functions
        println!("🚀 Optimizing hot path: {}", function_name);

        // TODO: Implement JIT optimization
        // - Profile function calls
        // - Inline small functions
        // - Optimize loops
        // - Specialize for common types

        Ok(())
    }
}

impl Default for JitCompiler {
    fn default() -> Self {
        Self::new()
    }
}
