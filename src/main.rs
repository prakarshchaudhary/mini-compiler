// CLI, reads source file
mod lexer;
mod parser;
mod ast;
mod semantic;
mod optimiser;
mod codegen;
mod utils;

fn main() {
    let source = r#"
    let x: i32 = 5;
    if x > 3 {
        x = x + 1;
    }
"#.to_string();

    let mut lexer = lexer::Lexer::new(source);
    let tokens = lexer.tokenize();

    for t in tokens {
        println!("{:?}", t);
    }

}