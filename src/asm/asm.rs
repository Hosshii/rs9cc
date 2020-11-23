use super::error::Error;
use crate::ast::{Node, NodeKind};

// jump の連番とかを格納しておく
pub struct Context {
    jump_label: usize,
}

impl Context {
    pub fn new() -> Self {
        Self { jump_label: 0 }
    }
}

pub fn gen(node: &Node, ctx: &mut Context) -> Result<(), Error> {
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
                gen(&rhs, ctx)?;
            } else {
                return Err(Error::not_found());
            }

            println!("    pop rdi");
            println!("    pop rax");
            println!("    mov [rax], rdi");
            println!("    push rdi");
            return Ok(());
        }
        NodeKind::Return => {
            if let Some(lhs) = &node.lhs {
                gen(&lhs, ctx)?;
            } else {
                return Err(Error::not_found());
            }
            println!("    pop rax");
            println!("    mov rsp, rbp");
            println!("    pop rbp");
            println!("    ret");
        }
        NodeKind::If => {
            if let Some(cond) = &node.cond {
                gen(cond, ctx)?;
            } else {
                return Err(Error::not_found());
            }

            let jlb_num = ctx.jump_label;
            ctx.jump_label += 1;
            println!("    pop rax");
            println!("    cmp rax, 0");

            if let Some(els) = &node.els {
                println!("    je  .Lelse{}", jlb_num);
                if let Some(then) = &node.then {
                    gen(then, ctx)?;
                } else {
                    return Err(Error::not_found());
                }
                println!("    jmp .Lend{}", jlb_num);
                println!(".Lelse{}:", jlb_num);
                gen(els, ctx)?;
                println!(".Lend{}:", jlb_num);
            } else {
                println!("    je  .Lend{}", jlb_num);

                if let Some(then) = &node.then {
                    gen(then, ctx)?;
                } else {
                    return Err(Error::not_found());
                }
                println!(".Lend{}:", jlb_num);
            }
        }
        NodeKind::While => {
            let jlb_num = ctx.jump_label;
            ctx.jump_label += 1;
            println!(".Lbegin{}:", jlb_num);
            if let Some(cond) = &node.cond {
                gen(cond, ctx)?;
            } else {
                return Err(Error::not_found());
            }

            println!("    pop rax");
            println!("    cmp rax, 0");
            println!("    je  .Lend{}", jlb_num);
            if let Some(then) = &node.then {
                gen(then, ctx)?;
            } else {
                return Err(Error::not_found());
            }
            println!("    jmp  .Lbegin{}", jlb_num);
            println!(".Lend{}:", jlb_num);
        }
        NodeKind::For => {
            let jlb_num = ctx.jump_label;
            ctx.jump_label += 1;
            if let Some(init) = &node.init {
                gen(init, ctx)?;
            }
            //  else {
            //     return Err(Error::not_found());
            // }
            println!(".Lbegin{}:", jlb_num);
            if let Some(cond) = &node.cond {
                gen(cond, ctx)?;
            }
            // else {
            //     return Err(Error::not_found());
            // }

            println!("    pop rax");
            println!("    cmp rax, 0");
            println!("    je  .Lend{}", jlb_num);
            if let Some(then) = &node.then {
                gen(then, ctx)?;
            } else {
                return Err(Error::not_found());
            }

            if let Some(inc) = &node.inc {
                gen(inc, ctx)?;
            }

            println!("    jmp  .Lbegin{}", jlb_num);
            println!(".Lend{}:", jlb_num);
        }
        _ => (),
    }

    if let Some(lhs) = &node.lhs {
        gen(lhs, ctx)?;
    }
    if let Some(rhs) = &node.rhs {
        gen(rhs, ctx)?;
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
