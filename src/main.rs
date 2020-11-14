use rs9cc::{tokenize, Operator, TokenKind};
use std::env;

fn main() {
    let s = env::args().nth(1).unwrap();
    if s == "-help" {
        help();
        return;
    }

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    let mut iter = tokenize(&s);
    println!("    mov rax, {}", iter.next().unwrap().expect_num());
    while let Some(token) = iter.next() {
        let n = iter.next().unwrap().expect_num();
        match token.kind {
            TokenKind::Reserved(op) => match op {
                Operator::Plus => println!("    add rax, {}", n),
                Operator::Minus => println!("    sub rax, {}", n),
            },
            x => panic!("unexpected operator: {:?}", x),
        }
    }
    println!("    ret");
}

fn help() {
    println!("this is help");
    println!("some error occurs");
}
