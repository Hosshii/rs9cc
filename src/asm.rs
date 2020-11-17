use crate::ast::{Node, NodeKind};

pub fn gen(node: &Node) {
    if let NodeKind::Num(x) = node.kind {
        println!("    push {}", x);
        return;
    }

    if let Some(lhs) = &node.lhs {
        gen(lhs);
    }
    if let Some(rhs) = &node.rhs {
        gen(rhs);
    }

    println!("    pop rdi");
    println!("    pop rax");

    match node.kind {
        NodeKind::Add => println!("    add rax, rdi"),
        NodeKind::Sub => println!("    sub rax, rdi"),
        NodeKind::Mul => println!("    imul rax, rdi"),
        NodeKind::Div => {
            println!("    cqo");
            println!("    idiv rax, rdi");
        }
        NodeKind::Equal => {
            println!("    cmp rax, rdi");
            println!("    sete al");
            println!("    movzb rax, al");
        }
        NodeKind::Leq => {
            println!("    cmp rax, rdi");
            println!("    setle al");
            println!("    movzb rax, al");
        }
        NodeKind::Lesser => {
            println!("    cmp rax, rdi");
            println!("    setl al");
            println!("    movzb rax, al");
        }
        NodeKind::Neq => {
            println!("    cmp rax, rdi");
            println!("    setne al");
            println!("    movzb rax, al");
        }
        _ => (),
    }

    println!("    push rax");
}
