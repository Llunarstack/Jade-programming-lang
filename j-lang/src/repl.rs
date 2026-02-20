use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::io::{self, Write};

pub struct Repl {
    interpreter: Interpreter,
    history: Vec<String>,
}

impl Repl {
    pub fn new() -> Self {
        Self {
            interpreter: Interpreter::new(),
            history: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            print!("j> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let input = input.trim();

                    if input.is_empty() {
                        continue;
                    }

                    match input {
                        "exit" | "quit" => {
                            println!("👋 Goodbye!");
                            break;
                        }
                        "help" => {
                            self.show_help();
                            continue;
                        }
                        "clear" => {
                            print!("\x1B[2J\x1B[1;1H");
                            continue;
                        }
                        "history" => {
                            self.show_history();
                            continue;
                        }
                        _ => {}
                    }

                    self.history.push(input.to_string());

                    match self.evaluate(input) {
                        Ok(result) => {
                            if !result.is_empty() {
                                println!("{}", result);
                            }
                        }
                        Err(e) => {
                            println!("❌ Error: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("❌ Input error: {}", e);
                    break;
                }
            }
        }
    }

    fn evaluate(&mut self, input: &str) -> Result<String, String> {
        // Tokenize
        let mut lexer = Lexer::new(input);
        let tokens = lexer
            .tokenize()
            .map_err(|e| format!("Lexer error: {}", e))?;

        // Parse
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().map_err(|e| format!("Parser error: {}", e))?;

        // Interpret
        let result = self
            .interpreter
            .evaluate(&ast)
            .map_err(|e| format!("Runtime error: {}", e))?;

        Ok(result)
    }

    fn show_help(&self) {
        println!("🚀 J Language REPL Commands:");
        println!("  help     - Show this help message");
        println!("  clear    - Clear the screen");
        println!("  history  - Show command history");
        println!("  exit     - Exit the REPL");
        println!();
        println!("📚 Quick J Syntax:");
        println!("  str | name -> \"Ethan\"     # Variable declaration");
        println!("  int | age -> 25           # Integer variable");
        println!("  out(name)                 # Print output");
        println!("  i in 1..10 : out(i)       # Loop");
        println!("  fn | add(a, b) > a + b    # Function");
        println!();
        println!("🔥 Try some examples:");
        println!("  str | msg -> \"Hello J!\"");
        println!("  out(msg)");
        println!("  list | nums -> [1,2,3,4,5]");
        println!("  in nums : out(_ * 2)");
        println!("  j; -> test.j              # Execute another J file");
    }

    fn show_history(&self) {
        if self.history.is_empty() {
            println!("📝 No command history");
            return;
        }

        println!("📝 Command History:");
        for (i, cmd) in self.history.iter().enumerate() {
            println!("  {}: {}", i + 1, cmd);
        }
    }
}

impl Default for Repl {
    fn default() -> Self {
        Self::new()
    }
}
