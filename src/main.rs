extern crate rs9cc;

use rs9cc::asm::code_gen;
use rs9cc::ast::{program, Context as AstContext};
use rs9cc::token::tokenize;
use std::env;

fn main() {
    let s = env::args().nth(1).unwrap();
    if s == "-help" {
        help();
        return;
    }

    // token生成
    let mut iter = tokenize(&s);

    // ast生成
    // let node = expr(&mut iter).unwrap();
    let program = match program(&mut iter, &mut AstContext::new()) {
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
        Ok(_) => (),
    }
}

fn help() {
    println!("this is help");
    println!("some error occurs");
}
