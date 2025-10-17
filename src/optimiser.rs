
use inkwell::module::Module;
use inkwell::passes::PassManager;
use inkwell::context::Context;

pub fn run_llvm_optimizations(module: &Module) {
    // Function pass manager
    let fpm = PassManager::create(module);
    fpm.add_instruction_combining_pass();
    fpm.add_reassociate_pass();
    fpm.add_gvn_pass();
    fpm.add_cfg_simplification_pass();
    fpm.add_dead_store_elimination_pass();
    fpm.initialize();

    for func in module.get_functions() {
        fpm.run_on(&func);
    }

    // Optionally you could also use a ModulePassManager (not shown here)
}
