extern crate rs9cc;

use rs9cc::asm::gen;
use rs9cc::ast::program;
use rs9cc::token::tokenize;
use std::env;

fn main() {
    let s = env::args().nth(1).unwrap();
    if s == "-help" {
        help();
        return;
    }

    // アセンブリの前半部分を出力
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    // プロローグ
    // 変数26個分の領域を確保する
    println!("    push rbp");
    println!("    mov rbp, rsp");
    println!("    sub rsp, 208");

    // token生成
    let mut iter = tokenize(&s);

    // ast生成
    // let node = expr(&mut iter).unwrap();
    let program = match program(&mut iter) {
        Ok(x) => x,
        Err(err) => {
            eprintln!("{}", err);
            panic!()
        }
    };
    // println!("{:#?}", node);

    // asm生成
    for i in program {
        if let Err(x) = gen(&i) {
            eprintln!("{}", x);
            panic!()
        }
        println!("    pop rax");
    }

    // エピローグ
    // 最後の式の結果がRAXに残っているのでそれが返り値になる
    println!("    mov rsp, rbp");
    println!("    pop rbp");
    println!("    ret");
}

fn help() {
    println!("this is help");
    println!("some error occurs");
}
