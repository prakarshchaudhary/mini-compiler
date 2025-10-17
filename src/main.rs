// src/main.rs
mod lexer;
mod parser;
mod ast;
mod semantic;
mod codegen_llvm;
mod optimiser;

use inkwell::context::Context;
use std::env;

fn main() {
    // simple demo: use hardcoded source or pass filename
    let source = r#"
        fn add(a: i32, b: i32) -> i32 {
            return a + b;
        }

        let x: i32 = 5;
        let y: i32 = 10;
        let z: i32 = add(x, y);
        if z > 10 {
            z = z + 1;
        } else {
            z = z - 1;
        }

        // while example
        let i: i32 = 0;
        // while i < 3 { i = i + 1; } // (if you want to test while)
    "#.to_string();

    // Lexing & parsing
    let mut lexer = lexer::Lexer::new(source);
    let tokens = lexer.tokenize();

    // Update: parser must produce Program (ast::Program). If your parser API differs, change this line.
    let mut parser = parser::Parser::new(tokens);
    let program = parser.parse(); // expects Program

    // Semantic analysis (your implementation)
    let mut sem = semantic::SemanticAnalyzer::new();
    sem.analyze(&program);

    // Codegen
    let context = Context::create();
    let mut codegen = codegen_llvm::LLVMCodegen::new(&context, "my_module");
    codegen.compile_program(&program);

    // Optional: optimise
    optimiser::run_llvm_optimizations(&codegen.module);

    // Emit IR (for debugging)
    codegen.dump_module();

    // JIT-run for quick tests (optional)
    // codegen.jit_run();

    // Write an object file for host native
    let default_triple = inkwell::targets::TargetMachine::get_default_triple();
    let native_triple = default_triple.to_str().unwrap();
    codegen.write_target_file("output.o", native_triple);

    // Also write a wasm object (if your LLVM supports wasm target)
    // codegen.write_target_file("output_wasm.o", "wasm32-unknown-unknown");

    println!("Done: generated output.o (and optionally output_wasm.o).");
}
