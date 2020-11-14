pub mod my_macro;

use std::env;

pub fn cc() {
    input! {
        s: String
    }
    if s == "-help" {
        help();
        return;
    }
    header();

    println!("main:");
    let mut iter = s.as_str().chars();
    if let Some(x) = iter.next() {
        println!("    mov rax, {}", x);
    } else {
        println!("no input found");
    }

    while let Some(x) = iter.next() {
        if x == '+' {
            println!("    add rax, {}", iter.next().unwrap());
            continue;
        }

        if x == '-' {
            println!("    sub rax, {}", iter.next().unwrap());
            continue;
        }
        println!("unexpected token: {}", x);
        break;
    }
    println!("    ret");
}

fn header() {
    println!(".intel_syntax noprefix");
    println!(".globl main");
}

fn help() {
    println!("this is help");
    println!("some error occurs");
}
