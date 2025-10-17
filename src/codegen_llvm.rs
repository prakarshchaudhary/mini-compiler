use inkwell::context::Context;
use inkwell::values::{BasicValueEnum, IntValue, PointerValue};
use inkwell::OptimizationLevel;
use inkwell::targets::{Target, InitializationConfig, TargetMachine, RelocMode, CodeModel, FileType};
use crate::ast::{Program, Stmt, Expr};
use std::collections::HashMap;

pub struct LLVMCodegen<'ctx> {
    pub context: &'ctx Context,
    pub module: inkwell::module::Module<'ctx>,
    pub builder: inkwell::builder::Builder<'ctx>,
    pub function: Option<inkwell::values::FunctionValue<'ctx>>,
    /// stack of var maps for scoping: each entry maps var name -> alloca pointer
    pub vars_stack: Vec<HashMap<String, PointerValue<'ctx>>>,
}

impl<'ctx> LLVMCodegen<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
        // initialize targets for target machines (native, wasm, etc.)
        Target::initialize_all(&InitializationConfig::default());

        let module = context.create_module(module_name);
        let builder = context.create_builder();
        LLVMCodegen {
            context,
            module,
            builder,
            function: None,
            vars_stack: vec![],
        }
    }

    /// Helper: current vars map (innermost scope)
    fn current_vars(&mut self) -> &mut HashMap<String, PointerValue<'ctx>> {
        if self.vars_stack.is_empty() {
            self.vars_stack.push(HashMap::new());
        }
        self.vars_stack.last_mut().unwrap()
    }

    /// Push / pop scope for local variables
    fn push_scope(&mut self) {
        self.vars_stack.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        self.vars_stack.pop();
    }

    /// Create an alloca in the function entry block and return pointer.
    /// This follows LLVM convention: perform alloca in entry for optimization friendliness.
    fn create_entry_alloca(&self, name: &str) -> PointerValue<'ctx> {
        let function = self.function.expect("function must exist to create entry alloca");
        let entry = function.get_first_basic_block().expect("function entry block expected");
        // Save current insertion point
        let current_bb = self.builder.get_insert_block();
        // Position at start of entry block
        self.builder.position_at_end(entry);
        let i32_type = self.context.i32_type();
        let alloca = self.builder.build_alloca(i32_type, name);
        // restore insertion point
        if let Some(bb) = current_bb {
            self.builder.position_at_end(bb);
        }
        alloca
    }

    /// Compile program: add top-level functions and a main wrapper that runs top-level stmts
    pub fn compile_program(&mut self, program: &Program) {
        // Create a main function that will execute top-level statements
        let i32_type = self.context.i32_type();
        let fn_type = i32_type.fn_type(&[], false);
        let main_fn = self.module.add_function("main", fn_type, None);
        let entry = self.context.append_basic_block(main_fn, "entry");
        self.builder.position_at_end(entry);
        self.function = Some(main_fn);

        // push scope for main
        self.push_scope();

        for stmt in &program.statements {
            // For top-level function definitions, create actual functions rather than code in main
            match stmt {
                Stmt::Function { .. } => {
                    // generate function definitions separately
                    self.compile_stmt(stmt);
                }
                _ => {
                    self.compile_stmt(stmt);
                }
            }
        }

        // return 0 at end of main
        self.builder.build_return(Some(&i32_type.const_int(0, false)));

        // pop main scope
        self.pop_scope();
    }

    fn compile_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::VarDecl { name, value, .. } => {
                let val = self.compile_expr(value);
                // allocate in entry
                let ptr = self.create_entry_alloca(name.as_str());
                self.builder.build_store(ptr, val);
                self.current_vars().insert(name.clone(), ptr);
            }

            Stmt::Assignment { name, value } => {
                let val = self.compile_expr(value);
                // find ptr in vars_stack (from innermost outward)
                for map in self.vars_stack.iter().rev() {
                    if let Some(ptr) = map.get(name) {
                        self.builder.build_store(*ptr, val);
                        return;
                    }
                }
                panic!("unknown variable {}", name);
            }

            Stmt::IfStmt { condition, then_branch, else_branch } => {
                let cond_val = self.compile_expr(condition);
                let parent = self.function.expect("function exists");
                let then_bb = self.context.append_basic_block(parent, "then");
                let else_bb = self.context.append_basic_block(parent, "else");
                let after_bb = self.context.append_basic_block(parent, "after_if");

                let cond_bool = self.builder.build_int_compare(
                    inkwell::IntPredicate::NE,
                    cond_val.into_int_value(),
                    self.context.i32_type().const_int(0, false),
                    "ifcond",
                );

                // If there is no else branch, branch to after directly from else_bb
                let has_else = else_branch.is_some();
                if has_else {
                    self.builder.build_conditional_branch(cond_bool, then_bb, else_bb);
                } else {
                    // use after_bb as else target
                    self.builder.build_conditional_branch(cond_bool, then_bb, after_bb);
                }

                // THEN branch
                self.builder.position_at_end(then_bb);
                self.push_scope();
                for s in then_branch {
                    self.compile_stmt(s);
                }
                self.pop_scope();
                self.builder.build_unconditional_branch(after_bb);

                // ELSE branch (if any)
                if let Some(else_stmts) = else_branch {
                    self.builder.position_at_end(else_bb);
                    self.push_scope();
                    for s in else_stmts {
                        self.compile_stmt(s);
                    }
                    self.pop_scope();
                    self.builder.build_unconditional_branch(after_bb);
                }

                // continue after
                self.builder.position_at_end(after_bb);
            }

            Stmt::While { condition, body } => {
                let parent = self.function.expect("function exists");
                let cond_bb = self.context.append_basic_block(parent, "while_cond");
                let body_bb = self.context.append_basic_block(parent, "while_body");
                let after_bb = self.context.append_basic_block(parent, "while_after");

                // jump to condition first
                self.builder.build_unconditional_branch(cond_bb);

                // condition block
                self.builder.position_at_end(cond_bb);
                let cond_val = self.compile_expr(condition);
                let cond_bool = self.builder.build_int_compare(
                    inkwell::IntPredicate::NE,
                    cond_val.into_int_value(),
                    self.context.i32_type().const_int(0, false),
                    "whilecond",
                );
                self.builder.build_conditional_branch(cond_bool, body_bb, after_bb);

                // body block
                self.builder.position_at_end(body_bb);
                self.push_scope();
                for s in body {
                    self.compile_stmt(s);
                }
                self.pop_scope();
                // after body, jump back to cond
                self.builder.build_unconditional_branch(cond_bb);

                // continue at after_bb
                self.builder.position_at_end(after_bb);
            }

            Stmt::Function { name, params, ret_type: _, body } => {
                // Build function type: all params and return type are i32 for now
                let i32_type = self.context.i32_type();
                let param_types: Vec<inkwell::types::BasicTypeEnum> =
                    params.iter().map(|_| i32_type.into()).collect();
                let fn_type = i32_type.fn_type(&param_types.iter().map(|t| t.as_ref()).collect::<Vec<_>>(), false);
                let function = self.module.add_function(name.as_str(), fn_type, None);
                let entry = self.context.append_basic_block(function, "entry");
                let previous_fn = self.function;
                self.function = Some(function);
                self.builder.position_at_end(entry);

                // new function scope for locals
                self.push_scope();

                // create allocas for parameters and store incoming values
                for (i, (pname, _ptype)) in params.iter().enumerate() {
                    let param_val = function.get_nth_param(i as u32).unwrap().into_int_value();
                    let alloca = self.create_entry_alloca(pname.as_str());
                    self.builder.build_store(alloca, param_val);
                    self.current_vars().insert(pname.clone(), alloca);
                }

                // compile body
                for s in body {
                    self.compile_stmt(s);
                }

                // if no explicit return, default return 0
                let i32_type = self.context.i32_type();
                self.builder.build_return(Some(&i32_type.const_int(0, false)));

                // pop fn scope and restore previous function
                self.pop_scope();
                self.function = previous_fn;
            }

            Stmt::Return(expr_opt) => {
                if let Some(expr) = expr_opt {
                    let val = self.compile_expr(expr);
                    self.builder.build_return(Some(&val.into_int_value()));
                } else {
                    let i32_type = self.context.i32_type();
                    self.builder.build_return(Some(&i32_type.const_int(0, false)));
                }
            }

            Stmt::ExprStmt(e) => {
                // evaluate expr and drop result
                let _ = self.compile_expr(e);
            }
        }
    }

    fn compile_expr(&mut self, expr: &Expr) -> BasicValueEnum<'ctx> {
        match expr {
            Expr::Number(n) => self.context.i32_type().const_int(*n as u64, true).into(),

            Expr::Identifier(name) => {
                // lookup pointer from vars stack
                for map in self.vars_stack.iter().rev() {
                    if let Some(ptr) = map.get(name) {
                        return self.builder.build_load(*ptr, name.as_str());
                    }
                }
                panic!("unknown variable {}", name);
            }

            Expr::Binary { left, operator, right } => {
                let l = self.compile_expr(left).into_int_value();
                let r = self.compile_expr(right).into_int_value();
                match operator.as_str() {
                    "+" => self.builder.build_int_add(l, r, "addtmp").into(),
                    "-" => self.builder.build_int_sub(l, r, "subtmp").into(),
                    "*" => self.builder.build_int_mul(l, r, "multmp").into(),
                    "/" => self.builder.build_int_signed_div(l, r, "divtmp").into(),
                    ">" => self.build_compare(l, r, inkwell::IntPredicate::SGT),
                    "<" => self.build_compare(l, r, inkwell::IntPredicate::SLT),
                    "==" => self.build_compare(l, r, inkwell::IntPredicate::EQ),
                    "!=" => self.build_compare(l, r, inkwell::IntPredicate::NE),
                    _ => panic!("unknown op {}", operator),
                }
            }

            Expr::Call { name, args } => {
                // compile args first
                let mut compiled_args: Vec<inkwell::values::BasicMetadataValueEnum> = Vec::new();
                for a in args {
                    let v = self.compile_expr(a).into_int_value();
                    compiled_args.push(v.into());
                }
                // find function
                if let Some(func) = self.module.get_function(name.as_str()) {
                    let call_site = self.builder.build_call(func, &compiled_args, "calltmp");
                    // returns i32
                    match call_site.try_as_basic_value().left() {
                        Some(bv) => bv,
                        None => panic!("expected function to return a basic value"),
                    }
                } else {
                    panic!("unknown function {}", name);
                }
            }
        }
    }

    fn build_compare(&self, l: IntValue<'ctx>, r: IntValue<'ctx>, pred: inkwell::IntPredicate) -> BasicValueEnum<'ctx> {
        let cmp = self.builder.build_int_compare(pred, l, r, "cmptmp");
        self.builder.build_int_z_extend(cmp, self.context.i32_type(), "bool_to_i32").into()
    }

    pub fn dump_module(&self) {
        self.module.print_to_stderr();
    }

    pub fn jit_run(&self) {
        let execution_engine = self.module.create_jit_execution_engine(OptimizationLevel::None).unwrap();
        unsafe {
            let main: inkwell::execution_engine::JitFunction<unsafe extern "C" fn() -> i32> =
                execution_engine.get_function("main").expect("main func");
            let res = main.call();
            println!("JIT main returned {}", res);
        }
    }

    /// Write object file for a given target triple (e.g., "wasm32-unknown-unknown" or default triple)
    pub fn write_target_file(&self, file_name: &str, target_triple: &str) {
        let target = Target::from_triple(target_triple).expect("target from triple");
        let machine = target
            .create_target_machine(
                target_triple,
                "generic",
                "",
                OptimizationLevel::Default,
                RelocMode::Default,
                CodeModel::Default,
            )
            .expect("create target machine");
        machine.write_to_file(&self.module, FileType::Object, std::path::Path::new(file_name)).expect("write file");
    }
}
