use rs9cc::{input, input_inner, read_value};

fn main() {
    input! {
        u: usize,
    }

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    println!("  mov rax, {}", u);
    println!("  ret");
}
