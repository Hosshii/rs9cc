use super::error::Error;
use crate::ast::{Node, NodeKind};

pub fn gen(node: &Node) -> Result<(), Error> {
    match &node.kind {
        NodeKind::Num(x) => {
            println!("    push {}", x);
            return Ok(());
        }
        NodeKind::Lvar(_) => {
            gen_lval(&node)?;
            println!("    pop rax");
            println!("    mov rax, [rax]");
            println!("    push rax");
            return Ok(());
        }
        NodeKind::Assign => {
            if let Some(lhs) = &node.lhs {
                gen_lval(&lhs)?;
            } else {
                return Err(Error::not_found());
            }
            if let Some(rhs) = &node.rhs {
                gen(&rhs)?;
            } else {
                return Err(Error::not_found());
            }

            println!("    pop rdi");
            println!("    pop rax");
            println!("    mov [rax], rdi");
            println!("    push rdi");
            return Ok(());
        }
        _ => (),
    }

    if let Some(lhs) = &node.lhs {
        gen(lhs)?;
    }
    if let Some(rhs) = &node.rhs {
        gen(rhs)?;
    }

    println!("    pop rdi");
    println!("    pop rax");

    match node.kind {
        NodeKind::Add => println!("    add rax, rdi"),
        NodeKind::Sub => println!("    sub rax, rdi"),
        NodeKind::Mul => println!("    imul rax, rdi"),
        NodeKind::Div => {
            println!("    cqo");
            println!("    idiv rdi");
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
    Ok(())
}

fn gen_lval(node: &Node) -> Result<(), Error> {
    if let NodeKind::Lvar(x) = &node.kind {
        println!("    mov rax, rbp");
        println!("    sub rax, {}", x.offset);
        println!("    push rax");
        Ok(())
    } else {
        Err(Error::not_lvar())
    }
}
