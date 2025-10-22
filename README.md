# Mini Compiler

A **mini compiler** project written in Rust that demonstrates the core stages of compilation: **lexical analysis, parsing, semantic analysis, optimization, and code generation**. This project is designed for learning and experimentation with compiler design concepts.

---

## Table of Contents
1. [Introduction](#introduction)  
2. [Features](#features)  
3. [Project Structure](#project-structure)  
4. [Technical Details](#technical-details)  
5. [Installation](#installation)  

---

## Introduction
This mini compiler implements a **basic compilation pipeline** in Rust:

- Tokenizes source code using a **lexer**  
- Builds an **Abstract Syntax Tree (AST)** with a **parser**  
- Performs **semantic analysis** for type checking and symbol validation  
- Applies **optimizations** on the intermediate representation  
- Generates **bytecode** or **LLVM IR** as output  

It is intended for educational purposes and as a base for experimenting with compiler techniques.

---

## Features
- **Lexer:** Converts source code into tokens (`lexer.rs`)  
- **Parser:** Builds an AST from tokens (`parser.rs`)  
- **Semantic Analyzer:** Checks types, variable declarations, and scope (`semantic.rs`)  
- **Optimizer:** Performs simple code optimizations (`optimiser.rs`)  
- **Code Generation:**  
  - Bytecode generation for a virtual machine (`codegen_bytecode.rs`)  
  - LLVM IR generation for further compilation (`codegen_llvm.rs`)  
- **Utilities:** Shared functions and helpers (`utils.rs`)  
- **Entry Point:** Orchestrates compilation stages (`main.rs`)  

---

## Project Structure

| File | Description |
|------|-------------|
| `main.rs` | Entry point; reads source files, runs lexer, parser, semantic, optimizer, and code generation stages. |
| `lexer.rs` | Tokenizes the input source code into a stream of tokens. |
| `parser.rs` | Builds the AST using a **recursive descent parser**. |
| `ast.rs` | Defines **AST node structures** for expressions, statements, and program constructs. |
| `semantic.rs` | Performs **semantic analysis**: symbol table management, type checking, and validation. |
| `optimiser.rs` | Applies **optimizations** to the AST or intermediate representation. |
| `codegen_bytecode.rs` | Converts the intermediate representation into **bytecode** for a simple VM. |
| `codegen_llvm.rs` | Converts AST or IR into **LLVM IR** for compilation to machine code. |
| `utils.rs` | Helper functions shared across modules (error handling, token utilities, etc.). |

---

## Technical Details
- **Language:** Rust  
- **Parser Type:** Recursive Descent  
- **AST Representation:** Structs representing expressions, statements, literals, identifiers, and program nodes  
- **Symbol Table:** Maintains variable names, types, and scope levels using Rust `HashMap`s  
- **Error Handling:**  
  - Lexer: Invalid characters  
  - Parser: Syntax errors with line/column info  
  - Semantic: Undeclared variables, type mismatches  
- **Intermediate Representations:**  
  - Bytecode instructions for a simple stack-based VM  
  - LLVM IR instructions for further compilation or optimization  
- **Optimizations:** Constant folding, dead code elimination (basic)  

---

## Installation
Clone the repository:

```bash
git clone https://github.com/your-username/mini-compiler.git
cd mini-compiler
