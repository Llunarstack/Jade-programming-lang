use std::fs;
use std::path::PathBuf;
use std::process::Command;

use crate::lexer::Lexer;
use crate::parser::Parser;

pub struct AotCompiler {
    optimization_level: u8,
    target_triple: String,
}

impl AotCompiler {
    pub fn new() -> Self {
        Self {
            optimization_level: 0,
            target_triple: Self::get_target_triple(),
        }
    }

    pub fn compile_file(
        &mut self,
        file_path: PathBuf,
        release: bool,
        output: Option<&str>,
    ) -> Result<(), String> {
        let source =
            fs::read_to_string(&file_path).map_err(|e| format!("Failed to read file: {}", e))?;

        self.optimization_level = if release { 3 } else { 0 };

        // Determine output filename
        let output_name = output.unwrap_or_else(|| {
            file_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("output")
        });

        println!("📝 Parsing source code...");

        // Parse the source
        let mut lexer = Lexer::new(&source);
        let tokens = lexer
            .tokenize()
            .map_err(|e| format!("Lexer error: {}", e))?;

        let mut parser = Parser::new(tokens);
        let ast = parser.parse().map_err(|e| format!("Parser error: {}", e))?;

        println!("🔧 Generating LLVM IR...");

        // Generate LLVM IR
        let llvm_ir = self.generate_llvm_ir(&ast)?;

        // Write IR to temporary file
        let ir_file = format!("{}.ll", output_name);
        fs::write(&ir_file, llvm_ir).map_err(|e| format!("Failed to write LLVM IR: {}", e))?;

        println!("⚡ Compiling to native code...");

        // Compile with LLVM
        self.compile_llvm_ir(&ir_file, output_name, release)?;

        // Clean up temporary files
        let _ = fs::remove_file(&ir_file);

        println!("🎉 Binary created: {}", output_name);

        Ok(())
    }

    fn generate_llvm_ir(&self, _ast: &crate::parser::AstNode) -> Result<String, String> {
        // This is a simplified LLVM IR generation
        // In a real implementation, this would traverse the AST and generate proper LLVM IR

        let ir = format!(
            r#"
; J Language Compiled Output
target triple = "{}"

@.str = private unnamed_addr constant [14 x i8] c"Hello from J!\00", align 1

declare i32 @printf(i8*, ...)

define i32 @main() {{
entry:
  %call = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([14 x i8], [14 x i8]* @.str, i32 0, i32 0))
  ret i32 0
}}
"#,
            self.target_triple
        );

        Ok(ir)
    }

    fn compile_llvm_ir(
        &self,
        ir_file: &str,
        output_name: &str,
        release: bool,
    ) -> Result<(), String> {
        let opt_level = if release { "3" } else { "0" };

        // Check if LLVM tools are available
        if !self.check_llvm_tools() {
            return self.fallback_compilation(ir_file, output_name);
        }

        // Optimize IR if release build
        if release {
            let optimized_ir = format!("{}.opt.ll", output_name);

            let opt_status = Command::new("opt")
                .args([&format!("-O{}", opt_level), ir_file, "-o", &optimized_ir])
                .status()
                .map_err(|e| format!("Failed to run opt: {}", e))?;

            if !opt_status.success() {
                return Err("LLVM optimization failed".to_string());
            }

            // Use optimized IR for compilation
            self.compile_ir_to_binary(&optimized_ir, output_name)?;
            let _ = fs::remove_file(&optimized_ir);
        } else {
            self.compile_ir_to_binary(ir_file, output_name)?;
        }

        Ok(())
    }

    fn compile_ir_to_binary(&self, ir_file: &str, output_name: &str) -> Result<(), String> {
        let output_file = if cfg!(windows) {
            format!("{}.exe", output_name)
        } else {
            output_name.to_string()
        };

        let llc_status = Command::new("llc")
            .args([ir_file, "-o", &format!("{}.s", output_name)])
            .status()
            .map_err(|e| format!("Failed to run llc: {}", e))?;

        if !llc_status.success() {
            return Err("LLVM code generation failed".to_string());
        }

        // Link with system linker
        let linker = if cfg!(windows) { "link" } else { "gcc" };

        let link_status = Command::new(linker)
            .args([&format!("{}.s", output_name), "-o", &output_file])
            .status()
            .map_err(|e| format!("Failed to run linker: {}", e))?;

        if !link_status.success() {
            return Err("Linking failed".to_string());
        }

        // Clean up assembly file
        let _ = fs::remove_file(format!("{}.s", output_name));

        Ok(())
    }

    fn check_llvm_tools(&self) -> bool {
        Command::new("llc").arg("--version").output().is_ok()
            && Command::new("opt").arg("--version").output().is_ok()
    }

    fn fallback_compilation(&self, _ir_file: &str, output_name: &str) -> Result<(), String> {
        println!("⚠️  LLVM tools not found, using fallback compilation");

        // Create a simple executable that prints a message
        let c_code = r#"
#include <stdio.h>

int main() {
    printf("Hello from J! (fallback compilation)\n");
    return 0;
}
"#;

        let c_file = format!("{}.c", output_name);
        fs::write(&c_file, c_code).map_err(|e| format!("Failed to write C file: {}", e))?;

        let output_file = if cfg!(windows) {
            format!("{}.exe", output_name)
        } else {
            output_name.to_string()
        };

        let compiler = if cfg!(windows) { "cl" } else { "gcc" };

        let compile_status = Command::new(compiler)
            .args([&c_file, "-o", &output_file])
            .status()
            .map_err(|e| format!("Failed to run C compiler: {}", e))?;

        if !compile_status.success() {
            return Err("C compilation failed".to_string());
        }

        // Clean up C file
        let _ = fs::remove_file(&c_file);

        Ok(())
    }

    fn get_target_triple() -> String {
        // Detect target triple based on current platform
        if cfg!(target_os = "windows") {
            if cfg!(target_arch = "x86_64") {
                "x86_64-pc-windows-msvc".to_string()
            } else {
                "i686-pc-windows-msvc".to_string()
            }
        } else if cfg!(target_os = "macos") {
            if cfg!(target_arch = "aarch64") {
                "aarch64-apple-darwin".to_string()
            } else {
                "x86_64-apple-darwin".to_string()
            }
        } else if cfg!(target_arch = "x86_64") {
            "x86_64-unknown-linux-gnu".to_string()
        } else if cfg!(target_arch = "aarch64") {
            "aarch64-unknown-linux-gnu".to_string()
        } else {
            "i686-unknown-linux-gnu".to_string()
        }
    }

    #[allow(dead_code)]
    pub fn set_optimization_level(&mut self, level: u8) {
        self.optimization_level = level.min(3);
    }

    #[allow(dead_code)]
    pub fn set_target(&mut self, target: String) {
        self.target_triple = target;
    }
}

impl Default for AotCompiler {
    fn default() -> Self {
        Self::new()
    }
}
