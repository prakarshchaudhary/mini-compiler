// codegen_bytecode.rs
use std::collections::HashMap;
use crate::ast::{Program, Stmt, Expr};

#[derive(Debug, Clone)]
pub enum Instr {
    PushInt(i32),
    Load(String),       // push variable value
    Store(String),      // pop and store into variable
    Add,
    Sub,
    Mul,
    Div,
    Gt,
    Lt,
    Eq,
    Neq,
    Jump(usize),        // unconditional jump to instruction index
    JumpIfFalse(usize), // pop value; if false (0) jump
    Pop,
    Halt,
}

pub struct Emitter {
    pub code: Vec<Instr>,
    // temporary stack for backpatch addresses, if needed
}

impl Emitter {
    pub fn new() -> Self { Emitter { code: vec![] } }

    pub fn emit(&mut self, instr: Instr) {
        self.code.push(instr);
    }

    // helper to get current instruction index
    pub fn pc(&self) -> usize { self.code.len() }

    // backpatch helper
    pub fn patch(&mut self, idx: usize, instr: Instr) {
        self.code[idx] = instr;
    }
}

pub fn compile_program(program: &Program) -> Vec<Instr> {
    let mut e = Emitter::new();
    for s in &program.statements {
        compile_stmt(&mut e, s);
    }
    e.emit(Instr::Halt);
    e.code
}

fn compile_stmt(e: &mut Emitter, stmt: &Stmt) {
    match stmt {
        Stmt::VarDecl { name, var_type: _, value } => {
            compile_expr(e, value);
            e.emit(Instr::Store(name.clone()));
        }
        Stmt::Assignment { name, value } => {
            compile_expr(e, value);
            e.emit(Instr::Store(name.clone()));
        }
        Stmt::IfStmt { condition, then_branch } => {
            compile_expr(e, condition);
            // emit placeholder for JumpIfFalse, will patch after body
            let jmp_if_false_pos = e.pc();
            e.emit(Instr::JumpIfFalse(0)); // placeholder
            for s in then_branch {
                compile_stmt(e, s);
            }
            // patch to jump to next instruction after body
            let after_body = e.pc();
            e.patch(jmp_if_false_pos, Instr::JumpIfFalse(after_body));
        }
    }
}

fn compile_expr(e: &mut Emitter, expr: &Expr) {
    match expr {
        Expr::Number(n) => e.emit(Instr::PushInt(*n)),
        Expr::Identifier(name) => e.emit(Instr::Load(name.clone())),
        Expr::Binary { left, operator, right } => {
            compile_expr(e, left);
            compile_expr(e, right);
            match operator.as_str() {
                "+" => e.emit(Instr::Add),
                "-" => e.emit(Instr::Sub),
                "*" => e.emit(Instr::Mul),
                "/" => e.emit(Instr::Div),
                ">" => e.emit(Instr::Gt),
                "<" => e.emit(Instr::Lt),
                "==" => e.emit(Instr::Eq),
                "!=" => e.emit(Instr::Neq),
                _ => panic!("Unknown operator {}", operator),
            }
        }
    }
}

pub struct VM {
    pub ip: usize,
    pub stack: Vec<i32>,
    pub code: Vec<Instr>,
    pub vars: HashMap<String, i32>,
}

impl VM {
    pub fn new(code: Vec<Instr>) -> Self {
        VM { ip: 0, stack: Vec::new(), code, vars: HashMap::new() }
    }

    pub fn run(&mut self) {
        loop {
            if self.ip >= self.code.len() { break; }
            match &self.code[self.ip] {
                Instr::PushInt(n) => { self.stack.push(*n); self.ip += 1; }
                Instr::Load(name) => {
                    let v = *self.vars.get(name).unwrap_or(&0);
                    self.stack.push(v);
                    self.ip += 1;
                }
                Instr::Store(name) => {
                    let v = self.stack.pop().expect("stack underflow on Store");
                    self.vars.insert(name.clone(), v);
                    self.ip += 1;
                }
                Instr::Add => {
                    let b = self.stack.pop().expect("stack underflow Add");
                    let a = self.stack.pop().expect("stack underflow Add");
                    self.stack.push(a + b);
                    self.ip += 1;
                }
                Instr::Sub => {
                    let b = self.stack.pop().expect("stack underflow Sub");
                    let a = self.stack.pop().expect("stack underflow Sub");
                    self.stack.push(a - b);
                    self.ip += 1;
                }
                Instr::Mul => {
                    let b = self.stack.pop().expect("stack underflow Mul");
                    let a = self.stack.pop().expect("stack underflow Mul");
                    self.stack.push(a * b);
                    self.ip += 1;
                }
                Instr::Div => {
                    let b = self.stack.pop().expect("stack underflow Div");
                    let a = self.stack.pop().expect("stack underflow Div");
                    self.stack.push(a / b);
                    self.ip += 1;
                }
                Instr::Gt => {
                    let b = self.stack.pop().expect("stack underflow Gt");
                    let a = self.stack.pop().expect("stack underflow Gt");
                    self.stack.push((a > b) as i32);
                    self.ip += 1;
                }
                Instr::Lt => {
                    let b = self.stack.pop().expect("stack underflow Lt");
                    let a = self.stack.pop().expect("stack underflow Lt");
                    self.stack.push((a < b) as i32);
                    self.ip += 1;
                }
                Instr::Eq => {
                    let b = self.stack.pop().expect("stack underflow Eq");
                    let a = self.stack.pop().expect("stack underflow Eq");
                    self.stack.push((a == b) as i32);
                    self.ip += 1;
                }
                Instr::Neq => {
                    let b = self.stack.pop().expect("stack underflow Neq");
                    let a = self.stack.pop().expect("stack underflow Neq");
                    self.stack.push((a != b) as i32);
                    self.ip += 1;
                }
                Instr::Jump(addr) => {
                    self.ip = *addr;
                }
                Instr::JumpIfFalse(addr) => {
                    let v = self.stack.pop().expect("stack underflow JumpIfFalse");
                    if v == 0 { self.ip = *addr; } else { self.ip += 1; }
                }
                Instr::Pop => { self.stack.pop(); self.ip += 1; }
                Instr::Halt => { break; }
            }
        }
    }
}
