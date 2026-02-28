//! Jade Programming Language — CLI binary.
//!
//! Delegates to the library crate for REPL, run, build, check, and jolt commands.

use clap::{Arg, Command};
use std::io::Write;
use std::path::PathBuf;

use j_lang::compiler::AotCompiler;
use j_lang::error::JError;
#[cfg(not(feature = "jit"))]
use j_lang::interpreter::Interpreter;
use j_lang::jolt::JoltManager;
use j_lang::lexer::Lexer;
use j_lang::parser::Parser;
use j_lang::repl::Repl;

#[cfg(feature = "jit")]
use j_lang::jit::JitCompiler;

/// Returns the required string argument or exits with an error.
fn require_arg(sub_matches: &clap::ArgMatches, name: &str, what: &str) -> String {
    sub_matches
        .get_one::<String>(name)
        .cloned()
        .unwrap_or_else(|| {
            eprintln!("❌ {}", what);
            std::process::exit(1);
        })
}

/// Reads the file at `path` into a string or exits with an error.
fn read_file_or_exit(path: &str) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("❌ Error reading file: {}", e);
        std::process::exit(1);
    })
}

/// Reads source from stdin (e.g. for `jade -`). Used so editors can pipe the current buffer without saving.
fn read_stdin_or_exit() -> String {
    use std::io::Read;
    let mut s = String::new();
    std::io::stdin()
        .read_to_string(&mut s)
        .unwrap_or_else(|e| {
            eprintln!("❌ Error reading stdin: {}", e);
            std::process::exit(1);
        });
    s
}

/// Run a single .jdl file (or stdin when file is "-") on the main thread.
fn run_single_file(file: &str) {
    let (source, parent) = if file == "-" {
        (read_stdin_or_exit(), std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
    } else {
        let file_path = PathBuf::from(file);
        let parent = file_path.parent().map(|p| p.to_path_buf()).unwrap_or_default();
        (read_file_or_exit(file), parent)
    };
    let parent_str = parent.to_string_lossy().to_string();
    let packages_path = parent.join(".jade").join("packages");
    let packages_str = packages_path.to_string_lossy().to_string();
    let has_packages = packages_path.exists();

    #[cfg(feature = "jit")]
    let ok = if file == "-" {
        let mut interp = Interpreter::new();
        if !parent_str.is_empty() {
            interp.add_module_search_path(&parent_str);
            if has_packages {
                interp.add_module_search_path(&packages_str);
            }
        }
        interp.run(&source)
    } else {
        let mut jit = JitCompiler::new();
        if !parent_str.is_empty() {
            jit.add_module_search_path(&parent_str);
            if has_packages {
                jit.add_module_search_path(&packages_str);
            }
        }
        jit.run_file(PathBuf::from(file), false)
    };

    #[cfg(not(feature = "jit"))]
    let ok = {
        let mut interpreter = Interpreter::new();
        if !parent_str.is_empty() {
            interpreter.add_module_search_path(&parent_str);
            if has_packages {
                interpreter.add_module_search_path(&packages_str);
            }
        }
        interpreter.run(&source)
    };

    if let Err(e) = ok {
        eprintln!("{}", JError::from_interpreter_message(&e));
        std::process::exit(1);
    }
    let _ = std::io::stdout().flush();
}

fn main() {
    let matches = clap_matches();
    // Run "jade file.jdl" on the main thread so output appears in Cursor/VS Code terminal.
    if matches.get_one::<String>("file").is_some() && matches.subcommand().is_none() {
        let file = matches.get_one::<String>("file").unwrap();
        run_single_file(file);
        return;
    }
    // REPL and other commands use a larger stack (avoid overflow on Windows).
    const STACK_SIZE: usize = 8 * 1024 * 1024; // 8 MiB
    let result = std::thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(run_cli)
        .expect("failed to spawn CLI thread")
        .join();
    if let Err(e) = result {
        std::panic::resume_unwind(e);
    }
}

fn clap_matches() -> clap::ArgMatches {
    Command::new("jade")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Jade Programming Language - Interpreter / JIT / AOT Compiler")
        .subcommand(Command::new("repl").about("Start interactive REPL"))
        .subcommand(Command::new("idle").about("Jade IDLE - interactive shell (like Python IDLE)"))
        .subcommand(
            Command::new("run")
                .about("Run J file with interpreter")
                .arg(
                    Arg::new("file")
                        .help("J source file to run")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            Command::new("build")
                .about("Compile J file to native binary (AOT)")
                .arg(
                    Arg::new("file")
                        .help("J source file to compile")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("release")
                        .long("release")
                        .help("Build optimized release binary")
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .help("Output binary name")
                        .value_name("FILE"),
                ),
        )
        .subcommand(
            Command::new("check")
                .about("Check Jade file for syntax errors")
                .arg(
                    Arg::new("file")
                        .help("Jade source file to check")
                        .required(true)
                        .index(1),
                ),
        )
        .arg(
            Arg::new("file")
                .index(1)
                .required(false)
                .help("Run this .jdl file (interpreted). Use - to read source from stdin (no save needed)"),
        )
        .subcommand(
            Command::new("jolt")
                .about("Jolt package manager")
                .subcommand(
                    Command::new("init")
                        .about("Initialize a new Jade project")
                        .arg(Arg::new("name").help("Project name").index(1))
                        .arg(
                            Arg::new("path")
                                .help("Project path")
                                .long("path")
                                .value_name("PATH"),
                        ),
                )
                .subcommand(
                    Command::new("add")
                        .about("Add a dependency")
                        .arg(
                            Arg::new("package")
                                .help("Package name")
                                .required(true)
                                .index(1),
                        )
                        .arg(
                            Arg::new("version")
                                .help("Package version")
                                .long("version")
                                .short('v')
                                .value_name("VERSION"),
                        ),
                )
                .subcommand(
                    Command::new("remove").about("Remove a dependency").arg(
                        Arg::new("package")
                            .help("Package name")
                            .required(true)
                            .index(1),
                    ),
                )
                .subcommand(Command::new("install").about("Install dependencies"))
                .subcommand(Command::new("list").about("List dependencies"))
                .subcommand(
                    Command::new("run").about("Run a script").arg(
                        Arg::new("script")
                            .help("Script name")
                            .required(true)
                            .index(1),
                    ),
                )
                .subcommand(Command::new("publish").about("Publish package to registry"))
                .subcommand(
                    Command::new("search").about("Search packages").arg(
                        Arg::new("query")
                            .help("Search query")
                            .required(true)
                            .index(1),
                    ),
                )
                .subcommand(
                    Command::new("info").about("Show package information").arg(
                        Arg::new("package")
                            .help("Package name")
                            .required(true)
                            .index(1),
                    ),
                ),
        )
        .get_matches()
}

fn run_cli() {
    let matches = clap_matches();
    match matches.subcommand() {
        Some(("repl", _)) => {
            let mut repl = Repl::new();
            repl.run();
        }
        Some(("run", sub_matches)) => {
            let file = require_arg(sub_matches, "file", "No file specified");
            let file_path = std::path::PathBuf::from(&file);
            let parent = file_path.parent().map(|p| p.to_path_buf()).unwrap_or_default();
            let parent_str = parent.to_string_lossy().to_string();
            let packages_path = parent.join(".jade").join("packages");
            let packages_str = packages_path.to_string_lossy().to_string();
            let has_packages = packages_path.exists();

            #[cfg(feature = "jit")]
            let ok = {
                let mut jit = JitCompiler::new();
                if !parent_str.is_empty() {
                    jit.add_module_search_path(&parent_str);
                    if has_packages {
                        jit.add_module_search_path(&packages_str);
                    }
                }
                jit.run_file(file_path, false)
            };

            #[cfg(not(feature = "jit"))]
            let ok = {
                let source = read_file_or_exit(&file);
                let mut interpreter = Interpreter::new();
                if !parent_str.is_empty() {
                    interpreter.add_module_search_path(&parent_str);
                    if has_packages {
                        interpreter.add_module_search_path(&packages_str);
                    }
                }
                interpreter.run(&source)
            };

            if let Err(e) = ok {
                eprintln!("{}", JError::from_interpreter_message(&e));
                std::process::exit(1);
            }
            let _ = std::io::stdout().flush();
        }
        Some(("idle", _)) => {
            let mut repl = Repl::new();
            repl.run_idle();
        }
        Some(("build", sub_matches)) => {
            let file = require_arg(sub_matches, "file", "No file specified");
            let release = sub_matches.get_flag("release");
            let output = sub_matches.get_one::<String>("output");
            let mode = if release { "release" } else { "debug" };
            let use_llvm = cfg!(feature = "llvm");

            if use_llvm {
                println!("🔨 Compiling {} in {} mode with LLVM", file, mode);
            } else {
                println!(
                    "🔨 Compiling {} in {} mode (fallback - no LLVM)",
                    file, mode
                );
            }

            let mut compiler = AotCompiler::new();
            if let Err(e) =
                compiler.compile_file(PathBuf::from(&file), release, output.map(String::as_str))
            {
                eprintln!("❌ Compilation failed: {}", e);
                std::process::exit(1);
            }
            println!("✅ Compilation successful!");
            if !use_llvm {
                println!("💡 For LLVM support, install Visual Studio Build Tools and rebuild with: cargo build --features llvm");
            }
        }
        Some(("check", sub_matches)) => {
            let file = require_arg(sub_matches, "file", "No file specified");
            println!("🔍 Checking {}", file);
            let source = read_file_or_exit(&file);

            let mut lexer = Lexer::new(&source);
            match lexer.tokenize() {
                Ok(tokens) => {
                    let mut parser = Parser::new(tokens);
                    match parser.parse() {
                        Ok(_) => println!("✅ No syntax errors found"),
                        Err(e) => {
                            eprintln!("{}", JError::from_interpreter_message(&e));
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("{}", JError::from_interpreter_message(&format!("Lexer error: {}", e)));
                    std::process::exit(1);
                }
            }
        }
        Some(("jolt", sub_matches)) => {
            let jolt = JoltManager::new();
            let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

            match sub_matches.subcommand() {
                Some(("init", init_matches)) => {
                    let name = init_matches.get_one::<String>("name").cloned();
                    let path = init_matches
                        .get_one::<String>("path")
                        .map(PathBuf::from)
                        .unwrap_or_else(|| {
                            if let Some(ref project_name) = name {
                                current_dir.join(project_name)
                            } else {
                                current_dir
                            }
                        });

                    if let Err(e) = jolt.init_project(&path, name) {
                        eprintln!("❌ Failed to initialize project: {}", e);
                        std::process::exit(1);
                    }
                }
                Some(("add", add_matches)) => {
                    let package = require_arg(add_matches, "package", "No package specified");
                    let version = add_matches.get_one::<String>("version").map(String::as_str);

                    if let Err(e) = jolt.add_dependency(&current_dir, &package, version) {
                        eprintln!("❌ Failed to add dependency: {}", e);
                        std::process::exit(1);
                    }
                }
                Some(("remove", remove_matches)) => {
                    let package = require_arg(remove_matches, "package", "No package specified");

                    if let Err(e) = jolt.remove_dependency(&current_dir, &package) {
                        eprintln!("❌ Failed to remove dependency: {}", e);
                        std::process::exit(1);
                    }
                }
                Some(("install", _)) => {
                    if let Err(e) = jolt.install_dependencies(&current_dir) {
                        eprintln!("❌ Failed to install dependencies: {}", e);
                        std::process::exit(1);
                    }
                    println!("✅ All dependencies installed!");
                }
                Some(("list", _)) => {
                    if let Err(e) = jolt.list_dependencies(&current_dir) {
                        eprintln!("❌ Failed to list dependencies: {}", e);
                        std::process::exit(1);
                    }
                }
                Some(("run", run_matches)) => {
                    let script = require_arg(run_matches, "script", "No script specified");

                    if let Err(e) = jolt.run_script(&current_dir, &script) {
                        eprintln!("❌ Failed to run script: {}", e);
                        std::process::exit(1);
                    }
                }
                Some(("publish", _)) => {
                    if let Err(e) = jolt.publish(&current_dir) {
                        eprintln!("❌ Failed to publish: {}", e);
                        std::process::exit(1);
                    }
                }
                Some(("search", search_matches)) => {
                    let query = require_arg(search_matches, "query", "No search query specified");

                    if let Err(e) = jolt.search(&query) {
                        eprintln!("❌ Search failed: {}", e);
                        std::process::exit(1);
                    }
                }
                Some(("info", info_matches)) => {
                    let package = require_arg(info_matches, "package", "No package specified");

                    if let Err(e) = jolt.info(&package) {
                        eprintln!("❌ Failed to get package info: {}", e);
                        std::process::exit(1);
                    }
                }
                _ => {
                    println!("Jolt — Jade package manager");
                    println!("  jade jolt --help");
                }
            }
        }
        _ => {
            let mut repl = Repl::new();
            repl.run();
        }
    }
}
