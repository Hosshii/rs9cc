extern crate rs9cc;

use rs9cc::asm::code_gen;
use rs9cc::ast::program;
use rs9cc::token;
use std::env;
use std::rc::Rc;

fn main() {
    let filepath = env::args().nth(1).unwrap();
    // token生成
    let mut token_stream = match token::tokenize_file(Rc::new(filepath)) {
        Ok(tokens) => tokens,
        Err(err) => {
            eprintln!("{}", err);
            panic!();
        }
    };

    // ast生成
    // let node = expr(&mut iter).unwrap();
    let program = match program(&mut token_stream) {
        Ok(x) => x,
        Err(err) => {
            eprintln!("{}", err);
            panic!()
        }
    };

    match code_gen(program) {
        Err(err) => {
            eprintln!("{}", err);
            panic!()
        }
        Ok(asm) => print!("{}", asm),
    }
}
