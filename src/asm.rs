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
        _ => (),
    }

    println!("    push rax");
}
