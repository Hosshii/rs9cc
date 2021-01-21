extern crate rs9cc;

use rs9cc::asm::code_gen;
use rs9cc::ast::program;
use rs9cc::token::tokenize;
use std::env;
use std::fs;
use std::rc::Rc;

fn main() {
    let filepath = env::args().nth(1).unwrap();
    let mut content = fs::read_to_string(&filepath).expect(&format!("{} is not exist", filepath));
    if content.len() == 0 || content.chars().last().unwrap() != '\n' {
        content += "\n";
    }

    // token生成
    let mut token_stream = match tokenize(Rc::new(content), Rc::new(filepath)) {
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
