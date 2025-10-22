# Mini Compiler

A simple **mini compiler** that demonstrates the core concepts of compiling, from **lexical analysis** to **code generation**. This project is designed as a learning tool to understand how programming languages are parsed, compiled, and executed.

---

## Table of Contents
1. [Introduction](#introduction)  
2. [Features](#features)  
3. [Architecture & Design](#architecture--design)  
4. [Technical Details](#technical-details)  
5. [Installation](#installation)  
6. [Usage](#usage)  
7. [Examples](#examples)  
8. [Limitations](#limitations)  
9. [Future Improvements](#future-improvements)  
10. [Contributing](#contributing)  
11. [License](#license)  

---

## Introduction
This mini compiler is an **educational project** to illustrate the compilation process. It supports:

- Lexical analysis  
- Parsing expressions and statements  
- Semantic analysis (basic type checking)  
- Intermediate code generation  
- Optional simple target code generation (e.g., assembly or virtual machine instructions)  

It helps learners understand **how a programming language is converted from source code to executable instructions**.

---

## Features
- **Lexer (Tokenizer):** Converts raw source code into tokens.  
- **Parser:** Builds an **Abstract Syntax Tree (AST)** from tokens.  
- **Semantic Analyzer:** Performs **basic type checking** and symbol table management.  
- **Intermediate Code Generation:** Generates pseudo-code or intermediate instructions.  
- **Optional Target Code Generation:** Can generate code for a small VM or simulate assembly instructions.  
- **Error Handling:** Reports **lexical, syntactical, and semantic errors** with line numbers.  

---

## Architecture & Design
The mini compiler follows a **classic compiler pipeline**:

1. **Lexical Analysis (Lexer)**  
   - Input: Source code string  
   - Output: List of tokens (identifiers, keywords, operators, literals)  

2. **Parsing (Parser)**  
   - Input: Token list  
   - Output: Abstract Syntax Tree (AST)  
   - Uses **recursive descent parsing** (or other parser)  

3. **Semantic Analysis**  
   - Input: AST  
   - Checks types, variable declarations, and scoping rules  
   - Maintains a **symbol table**  

4. **Intermediate Code Generation**  
   - Converts AST into **3-address code** or **VM instructions**  
   - Handles **expressions, assignments, loops, and conditions**  

5. **Optional Target Code Generation**  
   - Converts intermediate code to executable instructions or a simulated VM code  

6. **Error Reporting**  
   - Lexical errors: invalid characters  
   - Syntax errors: invalid grammar  
   - Semantic errors: undeclared variables, type mismatch  

---
