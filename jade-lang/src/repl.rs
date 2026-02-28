//! Interactive REPL for the Jade language.

use crate::error::JError;
use crate::interpreter::{Interpreter, Value};
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::env;
use std::io::{self, Write};
use std::path::Path;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn value_preview(v: &Value, max_len: usize) -> String {
    let s = format!("{}", v);
    if s.len() <= max_len {
        s
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

#[derive(Clone, Copy, PartialEq)]
enum CommandResult {
    Handled,
    Exit,
}

/// ASCII logo for Jade (spells "JADE"; green-friendly in terminals that support ANSI).
fn logo() -> &'static str {
    r#"
     ██╗ █████╗ ██████╗ ███████╗
     ██║██╔══██╗██╔══██╗██╔════╝
     ██║███████║██║  ██║█████╗
██   ██║██╔══██║██║  ██║██╔══╝
╚█████╔╝██║  ██║██████╔╝███████╗
 ╚════╝ ╚═╝  ╚═╝╚═════╝ ╚══════╝
   Jade · v"#
}

pub struct Repl {
    interpreter: Interpreter,
    history: Vec<String>,
    /// Current multi-line buffer (non-empty when waiting for more input).
    buffer: String,
}

impl Repl {
    pub fn new() -> Self {
        Self {
            interpreter: Interpreter::new(),
            history: Vec::new(),
            buffer: String::new(),
        }
    }

    pub fn run(&mut self) {
        self.print_welcome();
        self.run_loop();
    }

    /// Jade IDLE - interactive shell (like Python IDLE). Same as REPL with IDLE-style welcome.
    pub fn run_idle(&mut self) {
        println!("Jade IDLE {}", VERSION);
        println!("  Interactive shell. Type .help for commands, exit to quit.");
        println!();
        self.run_loop();
    }

    fn run_loop(&mut self) {
        loop {
            let prompt = if self.buffer.is_empty() { "Jade> " } else { "  ... " };
            print!("{}", prompt);
            if let Err(e) = io::stdout().flush() {
                eprintln!("Error flushing output: {}", e);
                break;
            }

            let mut line = String::new();
            match io::stdin().read_line(&mut line) {
                Ok(0) => break, // EOF
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Input error: {}", e);
                    break;
                }
            }

            self.buffer.push_str(&line);

            // Multi-line: if buffer ends with backslash or unclosed bracket/brace, wait for more
            let trimmed = self.buffer.trim_end();
            if trimmed.ends_with('\\') {
                self.buffer.pop(); // remove \
                self.buffer = self.buffer.trim_end().to_string();
                self.buffer.push(' ');
                continue;
            }
            if self.unclosed_delimiter(trimmed) {
                self.buffer.push(' ');
                continue;
            }

            let input = self.buffer.trim().to_string();
            self.buffer.clear();

            if input.is_empty() {
                continue;
            }

            // Dot-commands and built-in commands
            if let Some(cmd) = self.handle_command(&input) {
                if cmd == CommandResult::Exit {
                    break;
                }
                continue;
            }

            self.history.push(input.clone());

            match self.evaluate(&input) {
                Ok(result) => {
                    if !result.is_empty() {
                        println!("{}", result);
                    }
                }
                Err(e) => eprintln!("{}", JError::from_interpreter_message(&e)),
            }
        }
    }

    fn print_welcome(&self) {
        println!("{} {}", logo(), VERSION);
        println!();
        println!("  Type 'help' for commands, 'exit' to quit.");
        println!();
    }

    fn unclosed_delimiter(&self, s: &str) -> bool {
        let mut stack = Vec::new();
        let mut i = 0;
        let bytes = s.as_bytes();
        while i < bytes.len() {
            let c = bytes[i] as char;
            match c {
                '"' | '\'' => {
                    let end = if c == '"' { b'"' } else { b'\'' };
                    i += 1;
                    while i < bytes.len() {
                        if bytes[i] == b'\\' {
                            i += 2;
                            continue;
                        }
                        if bytes[i] == end {
                            i += 1;
                            break;
                        }
                        i += 1;
                    }
                }
                '#' if i + 1 < bytes.len() && bytes[i + 1] != b'#' => {
                    while i < bytes.len() && bytes[i] != b'\n' {
                        i += 1;
                    }
                }
                '(' => {
                    stack.push(')');
                    i += 1;
                }
                '[' => {
                    stack.push(']');
                    i += 1;
                }
                '{' => {
                    stack.push('}');
                    i += 1;
                }
                ')' | ']' | '}' => {
                    if let Some(expect) = stack.pop() {
                        if c != expect {
                            return true;
                        }
                    }
                    i += 1;
                }
                _ => i += 1,
            }
        }
        !stack.is_empty()
    }

    fn load_file(&mut self, path: &str) -> Result<(), String> {
        let source = std::fs::read_to_string(path)
            .map_err(|e| format!("Could not read file: {}", e))?;
        let mut lexer = Lexer::new(&source);
        let tokens = lexer.tokenize().map_err(|e| format!("Lexer: {}", e))?;
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().map_err(|e| format!("Parser: {}", e))?;
        self.interpreter
            .evaluate(&ast)
            .map_err(|e| format!("Runtime: {}", e))?;
        println!("Loaded {}", path);
        Ok(())
    }

    fn evaluate(&mut self, input: &str) -> Result<String, String> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().map_err(|e| format!("{}", e))?;

        let mut parser = Parser::new(tokens);
        let ast = parser.parse().map_err(|e| format!("{}", e))?;

        let result = self.interpreter.evaluate(&ast).map_err(|e| format!("{}", e))?;

        Ok(result)
    }

    fn print_error(&self, _prefix: &str, msg: &str) {
        eprintln!("{}", JError::from_interpreter_message(msg));
    }

    /// Handle built-in and dot-commands. Returns Some if handled (or exit), None to evaluate as Jade.
    fn handle_command(&mut self, input: &str) -> Option<CommandResult> {
        let input = input.trim();
        // Exit
        if matches!(input, "exit" | "quit" | ".exit" | ".quit") {
            println!("Goodbye.");
            return Some(CommandResult::Exit);
        }
        if input == "help" || input == ".help" || input == ".?" {
            self.show_help();
            return Some(CommandResult::Handled);
        }
        if input == "clear" || input == ".clear" || input == "cls" {
            print!("\x1B[2J\x1B[1;1H");
            return Some(CommandResult::Handled);
        }
        if input == "history" || input == ".history" {
            self.show_history();
            return Some(CommandResult::Handled);
        }
        if input == "version" || input == ".version" || input == ".ver" {
            println!("Jade {}", VERSION);
            return Some(CommandResult::Handled);
        }
        if input == "reset" || input == ".reset" {
            self.interpreter = Interpreter::new();
            println!("Interpreter reset.");
            return Some(CommandResult::Handled);
        }
        if input == "vars" || input == ".vars" || input == ".env" {
            self.show_vars();
            return Some(CommandResult::Handled);
        }
        if input == "pwd" || input == ".pwd" {
            if let Ok(cur) = env::current_dir() {
                println!("{}", cur.display());
            } else {
                eprintln!("pwd: cannot get current directory");
            }
            return Some(CommandResult::Handled);
        }
        if let Some(rest) = input.strip_prefix(".load ") {
            let path = rest.trim().trim_matches('"');
            if let Err(e) = self.load_file(path) {
                self.print_error("Load", &e);
            }
            return Some(CommandResult::Handled);
        }
        if let Some(rest) = input.strip_prefix(".run ") {
            let path = rest.trim().trim_matches('"');
            if let Err(e) = self.run_file(path) {
                self.print_error("Run", &e);
            }
            return Some(CommandResult::Handled);
        }
        if let Some(rest) = input.strip_prefix(".save ") {
            let path = rest.trim().trim_matches('"');
            if let Err(e) = self.save_history(path) {
                self.print_error("Save", &e);
            }
            return Some(CommandResult::Handled);
        }
        if let Some(rest) = input.strip_prefix(".cd ") {
            let path = rest.trim().trim_matches('"');
            if let Err(e) = env::set_current_dir(Path::new(path)) {
                self.print_error("cd", &e.to_string());
            }
            return Some(CommandResult::Handled);
        }
        if input == ".ls" || input == ".dir" || input.starts_with(".ls ") || input.starts_with(".dir ") {
            let dir = input
                .strip_prefix(".ls ")
                .or_else(|| input.strip_prefix(".dir "))
                .map(str::trim)
                .and_then(|s| if s.is_empty() { None } else { Some(s) })
                .unwrap_or(".");
            self.list_dir(dir);
            return Some(CommandResult::Handled);
        }
        if let Some(rest) = input.strip_prefix(".cat ") {
            let path = rest.trim().trim_matches('"');
            if let Err(e) = self.cat_file(path) {
                self.print_error("cat", &e);
            }
            return Some(CommandResult::Handled);
        }
        if let Some(rest) = input.strip_prefix(".edit ") {
            let path = rest.trim().trim_matches('"');
            self.edit_file(path);
            return Some(CommandResult::Handled);
        }
        None
    }

    fn show_vars(&self) {
        let names = self.interpreter.global_names();
        if names.is_empty() {
            println!("(no variables)");
            return;
        }
        const PREVIEW_LEN: usize = 52;
        for name in names {
            let preview = match self.interpreter.get_global(&name) {
                Some(v) => value_preview(&v, PREVIEW_LEN),
                None => "?".to_string(),
            };
            println!("  {} = {}", name, preview);
        }
    }

    fn run_file(&self, path: &str) -> Result<(), String> {
        let source = std::fs::read_to_string(path)
            .map_err(|e| format!("Could not read file: {}", e))?;
        let mut lexer = Lexer::new(&source);
        let tokens = lexer.tokenize().map_err(|e| format!("Lexer: {}", e))?;
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().map_err(|e| format!("Parser: {}", e))?;
        let mut interp = Interpreter::new();
        interp.evaluate(&ast).map_err(|e| format!("Runtime: {}", e))?;
        Ok(())
    }

    fn save_history(&self, path: &str) -> Result<(), String> {
        let content = self.history.join("\n");
        std::fs::write(path, content).map_err(|e| format!("Could not write: {}", e))?;
        println!("Saved {} lines to {}", self.history.len(), path);
        Ok(())
    }

    fn list_dir(&self, dir: &str) {
        let path = Path::new(dir);
        let read = match std::fs::read_dir(path) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("ls: {}: {}", dir, e);
                return;
            }
        };
        let mut names: Vec<String> = read
            .flatten()
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .collect();
        names.sort();
        for n in names {
            println!("  {}", n);
        }
    }

    fn cat_file(&self, path: &str) -> Result<(), String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("{}", e))?;
        print!("{}", content);
        Ok(())
    }

    fn edit_file(&self, path: &str) {
        let editor = env::var("VISUAL")
            .ok()
            .or_else(|| env::var("EDITOR").ok())
            .unwrap_or_else(|| {
                if cfg!(windows) {
                    "notepad".to_string()
                } else {
                    "nano".to_string()
                }
            });
        let status = std::process::Command::new(editor.split_whitespace().next().unwrap_or("notepad"))
            .args(editor.split_whitespace().skip(1))
            .arg(path)
            .status();
        if let Err(e) = status {
            eprintln!("edit: {}: {}", editor, e);
        }
    }

    fn show_help(&self) {
        println!("Jade REPL — commands (prefix with . for dot-commands):");
        println!();
        println!("  help, .help     This help");
        println!("  exit, .exit     Quit REPL");
        println!("  clear, cls      Clear screen");
        println!("  version         Jade version");
        println!("  reset           Reset interpreter (clear variables)");
        println!("  vars, .env      List variables and values");
        println!();
        println!("  .load <file>    Load and run a .jdl file in this session");
        println!("  .run <file>     Run a .jdl file in a fresh interpreter");
        println!("  .save <file>   Save command history to file");
        println!("  .pwd            Print working directory");
        println!("  .cd <dir>       Change directory");
        println!("  .ls [dir]       List directory (default: current)");
        println!("  .dir [dir]      Same as .ls");
        println!("  .cat <file>     Print file contents");
        println!("  .edit <file>    Open file in $EDITOR / $VISUAL / notepad");
        println!();
        println!("Jade syntax (quick reference):");
        println!("  type: name = value   e.g. str: name = \"value\"   int: n = 42   list: xs = [1,2,3]");
        println!("  out(x)    fn | add(a,b) > a+b    i in 1..10 : out(i)    import foo");
        println!();
        println!("Examples:");
        println!("  str: msg = \"Hello, Jade!\"   out(msg)   .load script.jdl   .vars");
    }

    fn show_history(&self) {
        if self.history.is_empty() {
            println!("No history yet.");
            return;
        }
        for (i, cmd) in self.history.iter().enumerate() {
            let preview = if cmd.len() > 60 {
                format!("{}...", &cmd[..57])
            } else {
                cmd.clone()
            };
            println!("  {:3}: {}", i + 1, preview);
        }
    }
}

impl Default for Repl {
    fn default() -> Self {
        Self::new()
    }
}
