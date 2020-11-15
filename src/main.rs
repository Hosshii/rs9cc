extern crate rs9cc;

use rs9cc::asm::gen;
use rs9cc::ast::expr;
use rs9cc::token::tokenize;
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

    // token生成
    let mut iter = tokenize(&s).peekable();

    // ast生成
    let node = expr(&mut iter).unwrap();

    // asm生成
    gen(&node);

    // スタックトップに乗っているはずの式全体の答えをとりだして返り値にする
    println!("    pop rax");
    println!("    ret");
}

fn help() {
    println!("this is help");
    println!("some error occurs");
}
