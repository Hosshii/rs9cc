use super::error::Error;
use crate::ast::{Node, NodeKind, Program};
use crate::base_types::{self, TypeKind};
use std::fmt::Write;

// jump の連番とかを格納しておく
pub struct Context {
    jump_label: usize,
    break_label: usize,
    asm: String,
}

impl Context {
    pub fn new() -> Self {
        Self {
            jump_label: 1,
            break_label: 0,
            asm: String::new(),
        }
    }
}

const ARGREG1: [&str; 6] = ["dil", "sil", "dl", "cl", "r8b", "r9b"];
const ARGREG2: [&str; 6] = ["di", "si", "dx", "cx", "r8w", "r9w"];
const ARGREG4: [&str; 6] = ["edi", "esi", "edx", "ecx", "r8d", "r9d"];
const ARGREG8: [&str; 6] = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];

pub fn code_gen(program: Program) -> Result<String, Error> {
    let mut ctx = Context::new();

    // アセンブリの前半部分を出力
    writeln!(ctx.asm, ".intel_syntax noprefix")?;

    writeln!(ctx.asm, ".data")?;
    // define global variable
    for (name, gvar) in program.ctx.g.gvar_mp {
        writeln!(ctx.asm, "{}:", name)?;
        match &gvar.dec.type_kind {
            TypeKind::Array(size, type_kind, _) => {
                let word = match &type_kind.borrow().size() {
                    1 => "byte",
                    4 => "long",
                    8 => "quad",
                    _ => todo!(),
                };
                for i in &gvar.init {
                    if let NodeKind::Num(x) = i.kind {
                        writeln!(ctx.asm, "    .{} {}", word, x)?;
                    } else {
                        unreachable!();
                    }
                }
                for _ in 0..(size - gvar.init.len() as u64) {
                    writeln!(ctx.asm, "    .{} {}", word, 0)?;
                }
            }
            TypeKind::Ptr(_) => {
                if gvar.init.len() <= 0 {
                    writeln!(ctx.asm, "    .quad {}", 0)?;
                } else {
                    if let Ok(gvar) = &gvar.init[0].get_gvar() {
                        writeln!(ctx.asm, "    .quad {}", gvar.dec.ident.name)?;
                    } else {
                        return Err(Error::not_gvar());
                    }
                }
            }
            type_kind => {
                let word = match &type_kind.size() {
                    1 => "byte",
                    4 => "long",
                    8 => "quad",
                    _ => todo!(),
                };
                if gvar.init.len() <= 0 {
                    writeln!(ctx.asm, "    .{} {}", word, 0)?;
                } else {
                    if let NodeKind::Num(x) = gvar.init[0].kind {
                        writeln!(ctx.asm, "    .{} {}", word, x)?;
                    } else {
                        writeln!(ctx.asm, "    .{} {}", word, 0)?;
                    }
                }
            }
        }
    }
    for (content, label) in program.ctx.g.tk_string {
        writeln!(ctx.asm, "{}:", label)?;
        for c in content.chars() {
            writeln!(ctx.asm, "    .byte {}", c as u8)?;
        }
    }

    writeln!(ctx.asm, ".text")?;
    writeln!(ctx.asm, ".global main")?;
    // asm生成
    for function in program.functions {
        writeln!(ctx.asm, "# start prologue")?;

        writeln!(ctx.asm, "{}:", function.def.ident.name)?;
        // プロローグ
        writeln!(ctx.asm, "    push rbp")?;
        writeln!(ctx.asm, "    mov rbp, rsp")?;
        writeln!(ctx.asm, "    sub rsp, {}", function.get_all_var_size())?;

        // 引数をローカル変数としてスタックに載せる
        let mut offset = 0;
        for i in 0..function.def.param_num {
            let type_kind = &function.def.params[i].type_kind;
            offset += type_kind.size();
            offset = base_types::align_to(offset, type_kind.align());
            writeln!(ctx.asm, "    mov rax, rbp")?;
            writeln!(ctx.asm, "    sub rax, {}", offset)?;
            let reg = match type_kind.size() {
                1 => ARGREG1[i],
                2 => ARGREG2[i],
                4 => ARGREG4[i],
                8 => ARGREG8[i],
                _ => unreachable!(),
            };
            writeln!(ctx.asm, "    mov [rax], {}", reg)?;
        }

        writeln!(ctx.asm, "# end prologue")?;

        for node in function.nodes {
            gen(&node, &mut ctx)?;
        }
        writeln!(ctx.asm, "    pop rax")?;
        // エピローグ
        // 最後の式の結果がRAXに残っているのでそれが返り値になる
        writeln!(ctx.asm, "    mov rsp, rbp")?;
        writeln!(ctx.asm, "    pop rbp")?;
        writeln!(ctx.asm, "    ret")?;
    }
    Ok(ctx.asm)
}

pub fn gen(node: &Node, ctx: &mut Context) -> Result<(), Error> {
    match &node.kind {
        NodeKind::Num(x) => {
            writeln!(ctx.asm, "# number")?;
            if x > &(u32::MAX as u64) {
                writeln!(ctx.asm, "    movabs rax, {}", x)?;
                writeln!(ctx.asm, "    push rax")?;
            } else {
                writeln!(ctx.asm, "    push {}", x)?;
            }
            return Ok(());
        }
        NodeKind::Lvar(_) | NodeKind::Gvar(_) => {
            writeln!(ctx.asm, "# NodeKind::Lvar, Gvar")?;
            gen_val(node, ctx)?;
            if let Ok(TypeKind::Array(_, _, _)) = node.get_type() {
                return Ok(());
            }
            load(node, ctx)?;
            return Ok(());
        }
        NodeKind::Member(_, member) => {
            writeln!(ctx.asm, "# NodeKind::Member")?;
            gen_val(node, ctx)?;
            if let TypeKind::Array(_, _, _) = &*member.get_type() {
                return Ok(());
            }
            load(node, ctx)?;
            return Ok(());
        }
        NodeKind::Assign => {
            writeln!(ctx.asm, "# NodeKind::Assign")?;
            if let Some(lhs) = &node.lhs {
                gen_val(&lhs, ctx)?;
            } else {
                return Err(Error::not_found());
            }
            if let Some(rhs) = &node.rhs {
                gen(&rhs, ctx)?;
            } else {
                return Err(Error::not_found());
            }

            store(node, ctx)?;
            return Ok(());
        }
        NodeKind::AAdd | NodeKind::ASub | NodeKind::AMul | NodeKind::ADiv => {
            let lhs = node.lhs.as_ref().expect("lhs not found");
            let rhs = node.rhs.as_ref().expect("rhs not found");
            gen_val(lhs, ctx)?;
            writeln!(ctx.asm, "    push [rsp]")?;
            load(lhs, ctx)?;
            gen(rhs, ctx)?;
            writeln!(ctx.asm, "    pop rdi")?;
            writeln!(ctx.asm, "    pop rax")?;
            match &node.kind {
                NodeKind::AAdd => {
                    ptr_op(node, ctx)?;
                    writeln!(ctx.asm, "    add rax, rdi")?;
                }
                NodeKind::ASub => {
                    ptr_op(node, ctx)?;
                    writeln!(ctx.asm, "    sub rax, rdi")?;
                }
                NodeKind::AMul => {
                    writeln!(ctx.asm, "    imul rax, rdi")?;
                }
                NodeKind::ADiv => {
                    writeln!(ctx.asm, "    cqo")?;
                    writeln!(ctx.asm, "    idiv rdi")?;
                }
                _ => unreachable!(),
            }
            writeln!(ctx.asm, "    push rax")?;
            store(node, ctx)?;
            return Ok(());
        }
        NodeKind::Return => {
            writeln!(ctx.asm, "# NodeKind::Return")?;
            if let Some(lhs) = &node.lhs {
                gen(&lhs, ctx)?;
            } else {
                return Err(Error::not_found());
            }
            writeln!(ctx.asm, "    pop rax")?;
            writeln!(ctx.asm, "    mov rsp, rbp")?;
            writeln!(ctx.asm, "    pop rbp")?;
            writeln!(ctx.asm, "    ret")?;
            return Ok(());
        }
        NodeKind::If => {
            writeln!(ctx.asm, "# NodeKind::If")?;
            if let Some(cond) = &node.cond {
                gen(cond, ctx)?;
            } else {
                return Err(Error::not_found());
            }

            let jlb_num = ctx.jump_label;
            ctx.jump_label += 1;
            writeln!(ctx.asm, "    pop rax")?;
            writeln!(ctx.asm, "    cmp rax, 0")?;

            if let Some(els) = &node.els {
                writeln!(ctx.asm, "    je  .Lelse{}", jlb_num)?;
                if let Some(then) = &node.then {
                    gen(then, ctx)?;
                } else {
                    return Err(Error::not_found());
                }
                writeln!(ctx.asm, "    jmp .Lend{}", jlb_num)?;
                writeln!(ctx.asm, ".Lelse{}:", jlb_num)?;
                gen(els, ctx)?;
                writeln!(ctx.asm, ".Lend{}:", jlb_num)?;
            } else {
                writeln!(ctx.asm, "    je  .Lend{}", jlb_num)?;

                if let Some(then) = &node.then {
                    gen(then, ctx)?;
                } else {
                    return Err(Error::not_found());
                }
                writeln!(ctx.asm, ".Lend{}:", jlb_num)?;
            }
            return Ok(());
        }
        NodeKind::While => {
            writeln!(ctx.asm, "# NodeKind::While")?;
            let jlb_num = ctx.jump_label;
            let break_num = ctx.break_label;
            ctx.jump_label += 1;
            ctx.break_label = jlb_num;
            writeln!(ctx.asm, ".Lbegin{}:", jlb_num)?;
            if let Some(cond) = &node.cond {
                gen(cond, ctx)?;
            } else {
                return Err(Error::not_found());
            }

            writeln!(ctx.asm, "    pop rax")?;
            writeln!(ctx.asm, "    cmp rax, 0")?;
            writeln!(ctx.asm, "    je  .L.break.{}", jlb_num)?;
            if let Some(then) = &node.then {
                gen(then, ctx)?;
            } else {
                return Err(Error::not_found());
            }
            writeln!(ctx.asm, "    jmp  .Lbegin{}", jlb_num)?;
            writeln!(ctx.asm, ".L.break.{}:", jlb_num)?;
            ctx.break_label = break_num;
            return Ok(());
        }
        NodeKind::For => {
            writeln!(ctx.asm, "# NodeKind::For")?;
            let jlb_num = ctx.jump_label;
            let break_num = ctx.break_label;
            ctx.jump_label += 1;
            ctx.break_label = jlb_num;
            if let Some(init) = &node.init {
                for i in init {
                    gen(i, ctx)?;
                }
            }

            writeln!(ctx.asm, ".Lbegin{}:", jlb_num)?;
            if let Some(cond) = &node.cond {
                gen(cond, ctx)?;
                writeln!(ctx.asm, "    pop rax")?;
                writeln!(ctx.asm, "    cmp rax, 0")?;
                writeln!(ctx.asm, "    je  .L.break.{}", jlb_num)?;
            }

            if let Some(then) = &node.then {
                gen(then, ctx)?;
            } else {
                return Err(Error::not_found());
            }

            if let Some(inc) = &node.inc {
                gen(inc, ctx)?;
            }

            writeln!(ctx.asm, "    jmp  .Lbegin{}", jlb_num)?;
            writeln!(ctx.asm, ".L.break.{}:", jlb_num)?;
            ctx.break_label = break_num;
            return Ok(());
        }
        NodeKind::Break => match ctx.break_label {
            0 => return Err(Error::stray_break()),
            _ => {
                writeln!(ctx.asm, "    jmp .L.break.{}", ctx.break_label)?;
                return Ok(());
            }
        },
        NodeKind::Block(stmts) | NodeKind::StmtExpr(stmts) => {
            writeln!(ctx.asm, "# NodeKind::Block,StmtExpr")?;
            for stmt in stmts {
                gen(stmt, ctx)?;
            }
            return Ok(());
        }
        NodeKind::Func(func_prototype, args) => {
            writeln!(ctx.asm, "# NodeKind::Func")?;
            let jlb_num = ctx.jump_label;
            ctx.jump_label += 1;

            // printf 関数はalに浮動小数点数の引数の個数をいれる必要がある
            // 今はないので決め打ちで0にする
            writeln!(ctx.asm, "    mov al, 0")?;

            for i in args {
                gen(i, ctx)?;
            }
            for i in (0..args.len()).rev() {
                writeln!(ctx.asm, "    pop {}", ARGREG8[i])?;
            }
            // 8の倍数じゃなかったら8の倍数にする
            writeln!(ctx.asm, "    mov rax, rsp")?;
            writeln!(ctx.asm, "    and rax, 7")?;
            writeln!(ctx.asm, "    jnz .LcallFour{}", jlb_num)?;
            writeln!(ctx.asm, "    sub rsp, 4")?;
            {
                // 16の倍数にしてcall
                writeln!(ctx.asm, "    mov rax, rsp")?;
                writeln!(ctx.asm, "    and rax, 15")?;
                writeln!(ctx.asm, "    jnz .LcallF{}", jlb_num)?;
                writeln!(ctx.asm, "    mov rax, 0")?;
                writeln!(ctx.asm, "    call {}", func_prototype.ident.name)?;
                writeln!(ctx.asm, "    add rsp, 4")?;
                writeln!(ctx.asm, "    jmp .LendF{}", jlb_num)?;
                writeln!(ctx.asm, ".LcallF{}:", jlb_num)?;
                writeln!(ctx.asm, "    sub rsp, 8")?;
                writeln!(ctx.asm, "    mov rax, 0")?;
                writeln!(ctx.asm, "    call {}", func_prototype.ident.name)?;
                writeln!(ctx.asm, "    add rsp, 12")?;
                writeln!(ctx.asm, ".LendF{}:", jlb_num)?;
                writeln!(ctx.asm, "    push rax")?;
            }
            writeln!(ctx.asm, "    jmp .Lend{}", jlb_num)?;

            writeln!(ctx.asm, ".LcallFour{}:", jlb_num)?;
            writeln!(ctx.asm, "    mov rax, 0")?;
            {
                // 16の倍数にしてcall
                writeln!(ctx.asm, "    mov rax, rsp")?;
                writeln!(ctx.asm, "    and rax, 15")?;
                writeln!(ctx.asm, "    jnz .LcallFH{}", jlb_num)?;
                writeln!(ctx.asm, "    mov rax, 0")?;
                writeln!(ctx.asm, "    call {}", func_prototype.ident.name)?;
                writeln!(ctx.asm, "    jmp .LendFH{}", jlb_num)?;
                writeln!(ctx.asm, ".LcallFH{}:", jlb_num)?;
                writeln!(ctx.asm, "    sub rsp, 8")?;
                writeln!(ctx.asm, "    mov rax, 0")?;
                writeln!(ctx.asm, "    call {}", func_prototype.ident.name)?;
                writeln!(ctx.asm, "    add rsp, 8")?;
                writeln!(ctx.asm, ".LendFH{}:", jlb_num)?;
                writeln!(ctx.asm, "    push rax")?;
            }
            writeln!(ctx.asm, ".Lend{}:", jlb_num)?;
            cast(&func_prototype.type_kind, ctx)?;
            return Ok(());
        }
        NodeKind::Addr => {
            writeln!(ctx.asm, "# NodeKind::Addr")?;
            if let Some(lhs) = &node.lhs {
                gen_val(&lhs, ctx)?;
            } else {
                return Err(Error::not_found());
            }
            return Ok(());
        }
        NodeKind::Deref => {
            writeln!(ctx.asm, "# NodeKind::Deref")?;
            if let Some(lhs) = &node.lhs {
                gen(&lhs, ctx)?;
                if let Ok(TypeKind::Array(_, _, _)) = node.get_type() {
                    return Ok(());
                }
                load(node, ctx)?;
            } else {
                return Err(Error::not_found());
            }
            return Ok(());
        }
        NodeKind::Declaration(_) => {
            writeln!(ctx.asm, "# declaration")?;
            if let Some(ref init) = node.init {
                for i in init {
                    gen(i, ctx)?
                }
            }

            return Ok(());
        }
        NodeKind::ExprStmt => {
            writeln!(ctx.asm, "# NodeKind::ExprStmt")?;
            if let Some(lhs) = &node.lhs {
                gen(&lhs, ctx)?;
                writeln!(ctx.asm, "    add rsp, 8")?;
            } else {
                return Err(Error::not_found());
            }
            return Ok(());
        }
        NodeKind::Null => return Ok(()),
        NodeKind::Cast(type_kind) => {
            writeln!(ctx.asm, "# cast")?;
            gen(&node.lhs.as_ref().unwrap(), ctx)?;
            cast(type_kind, ctx)?;
            return Ok(());
        }
        NodeKind::Comma => {
            writeln!(ctx.asm, "# comma")?;
            gen(node.lhs.as_ref().unwrap(), ctx)?;
            gen(node.rhs.as_ref().unwrap(), ctx)?;
        }
        NodeKind::PreInc => {
            writeln!(ctx.asm, "# preinc")?;
            gen_val(node.lhs.as_ref().unwrap(), ctx)?;
            writeln!(ctx.asm, "    push [rsp]")?;
            load(node, ctx)?;
            inc(node, ctx)?;
            store(node, ctx)?;
            return Ok(());
        }
        NodeKind::PreDec => {
            writeln!(ctx.asm, "# predec")?;
            gen_val(node.lhs.as_ref().unwrap(), ctx)?;
            writeln!(ctx.asm, "    push [rsp]")?;
            load(node, ctx)?;
            dec(node, ctx)?;
            store(node, ctx)?;
            return Ok(());
        }
        NodeKind::PostInc => {
            writeln!(ctx.asm, "# postinc")?;
            gen_val(node.lhs.as_ref().unwrap(), ctx)?;
            writeln!(ctx.asm, "    push [rsp]")?;
            load(node, ctx)?;
            inc(node, ctx)?;
            store(node, ctx)?;
            dec(node, ctx)?;
            return Ok(());
        }
        NodeKind::PostDec => {
            writeln!(ctx.asm, "# postdec")?;
            gen_val(node.lhs.as_ref().unwrap(), ctx)?;
            writeln!(ctx.asm, "    push [rsp]")?;
            load(node, ctx)?;
            dec(node, ctx)?;
            store(node, ctx)?;
            inc(node, ctx)?;
            return Ok(());
        }
        NodeKind::Not => {
            writeln!(ctx.asm, "# not")?;
            gen(node.lhs.as_ref().unwrap(), ctx)?;
            writeln!(ctx.asm, "    pop rax")?;
            writeln!(ctx.asm, "    cmp rax, 0")?;
            writeln!(ctx.asm, "    sete al")?;
            writeln!(ctx.asm, "    movzb rax, al")?;
            writeln!(ctx.asm, "    push rax")?;
            return Ok(());
        }
        NodeKind::BitNot => {
            writeln!(ctx.asm, "# bit not")?;
            gen(node.lhs.as_ref().unwrap(), ctx)?;
            writeln!(ctx.asm, "    pop rax")?;
            writeln!(ctx.asm, "    not rax")?;
            writeln!(ctx.asm, "    push rax")?;
            return Ok(());
        }
        NodeKind::LogOr => {
            writeln!(ctx.asm, "# log or")?;
            let jlb_num = ctx.jump_label;
            ctx.jump_label += 1;
            gen(node.lhs.as_ref().unwrap(), ctx)?;
            writeln!(ctx.asm, "    pop rax")?;
            writeln!(ctx.asm, "    cmp rax, 0")?;
            writeln!(ctx.asm, "    jne  .Ltrue{}", jlb_num)?;
            gen(node.rhs.as_ref().unwrap(), ctx)?;
            writeln!(ctx.asm, "    pop rax")?;
            writeln!(ctx.asm, "    cmp rax, 0")?;
            writeln!(ctx.asm, "    jne  .Ltrue{}", jlb_num)?;
            writeln!(ctx.asm, "    push 0")?;
            writeln!(ctx.asm, "    jmp .Lend{}", jlb_num)?;
            writeln!(ctx.asm, ".Ltrue{}:", jlb_num)?;
            writeln!(ctx.asm, "    push 1")?;
            writeln!(ctx.asm, ".Lend{}:", jlb_num)?;
            return Ok(());
        }
        NodeKind::LogAnd => {
            writeln!(ctx.asm, "# log and")?;
            let jlb_num = ctx.jump_label;
            ctx.jump_label += 1;
            gen(node.lhs.as_ref().unwrap(), ctx)?;
            writeln!(ctx.asm, "    pop rax")?;
            writeln!(ctx.asm, "    cmp rax, 0")?;
            writeln!(ctx.asm, "    je  .Lfalse{}", jlb_num)?;
            gen(node.rhs.as_ref().unwrap(), ctx)?;
            writeln!(ctx.asm, "    pop rax")?;
            writeln!(ctx.asm, "    cmp rax, 0")?;
            writeln!(ctx.asm, "    je  .Lfalse{}", jlb_num)?;
            writeln!(ctx.asm, "    push 1")?;
            writeln!(ctx.asm, "    jmp .Lend{}", jlb_num)?;
            writeln!(ctx.asm, ".Lfalse{}:", jlb_num)?;
            writeln!(ctx.asm, "    push 0")?;
            writeln!(ctx.asm, ".Lend{}:", jlb_num)?;
            return Ok(());
        }
        _ => (),
    }

    if let Some(lhs) = &node.lhs {
        writeln!(ctx.asm, "# lhs")?;
        gen(lhs, ctx)?;
    }
    if let Some(rhs) = &node.rhs {
        writeln!(ctx.asm, "# rhs")?;
        gen(rhs, ctx)?;
    }

    writeln!(ctx.asm, "# pop")?;
    writeln!(ctx.asm, "    pop rdi")?;
    writeln!(ctx.asm, "    pop rax")?;

    match node.kind {
        NodeKind::Add => {
            writeln!(ctx.asm, "# Add")?;
            ptr_op(node, ctx)?;
            writeln!(ctx.asm, "    add rax, rdi")?;
        }
        NodeKind::Sub => {
            writeln!(ctx.asm, "# Sub")?;
            ptr_op(node, ctx)?;
            writeln!(ctx.asm, "    sub rax, rdi")?;
        }
        NodeKind::Mul => {
            writeln!(ctx.asm, "# Mul")?;
            ptr_op(node, ctx)?;
            writeln!(ctx.asm, "    imul rax, rdi")?;
        }
        NodeKind::Div => {
            writeln!(ctx.asm, "# Div")?;
            writeln!(ctx.asm, "    cqo")?;
            writeln!(ctx.asm, "    idiv rdi")?;
        }
        NodeKind::BitAnd => {
            writeln!(ctx.asm, "    and rax, rdi")?;
        }
        NodeKind::BitOr => {
            writeln!(ctx.asm, "    or rax, rdi")?;
        }
        NodeKind::BitXor => {
            writeln!(ctx.asm, "    xor rax, rdi")?;
        }
        NodeKind::Equal => {
            writeln!(ctx.asm, "# Equal")?;
            writeln!(ctx.asm, "    cmp rax, rdi")?;
            writeln!(ctx.asm, "    sete al")?;
            writeln!(ctx.asm, "    movzb rax, al")?;
        }
        NodeKind::Leq => {
            writeln!(ctx.asm, "# Leq")?;
            writeln!(ctx.asm, "    cmp rax, rdi")?;
            writeln!(ctx.asm, "    setle al")?;
            writeln!(ctx.asm, "    movzb rax, al")?;
        }
        NodeKind::Lesser => {
            writeln!(ctx.asm, "# Lesser")?;
            writeln!(ctx.asm, "    cmp rax, rdi")?;
            writeln!(ctx.asm, "    setl al")?;
            writeln!(ctx.asm, "    movzb rax, al")?;
        }
        NodeKind::Neq => {
            writeln!(ctx.asm, "# Neq")?;
            writeln!(ctx.asm, "    cmp rax, rdi")?;
            writeln!(ctx.asm, "    setne al")?;
            writeln!(ctx.asm, "    movzb rax, al")?;
        }
        _ => (),
    }

    writeln!(ctx.asm, "    push rax")?;
    Ok(())
}

fn gen_val(node: &Node, ctx: &mut Context) -> Result<(), Error> {
    writeln!(ctx.asm, "# gen val")?;
    match &node.kind {
        NodeKind::Lvar(x) => {
            writeln!(ctx.asm, "# lvar")?;
            writeln!(ctx.asm, "    mov rax, rbp")?;
            writeln!(ctx.asm, "    sub rax, {}", x.offset)?;
            writeln!(ctx.asm, "    push rax")?;
            Ok(())
        }
        NodeKind::Gvar(x) => {
            writeln!(ctx.asm, "# gvar")?;
            writeln!(ctx.asm, "    mov rax, OFFSET FLAT:{}", x.dec.ident.name)?;
            writeln!(ctx.asm, "    push rax")?;
            Ok(())
        }
        NodeKind::Deref => {
            writeln!(ctx.asm, "# deref")?;
            if let Some(lhs) = &node.lhs {
                gen(&lhs, ctx)
            } else {
                Err(Error::not_found())
            }
        }
        NodeKind::Member(_, member) => {
            writeln!(ctx.asm, "# member")?;
            if let Some(lhs) = &node.lhs {
                gen_val(&lhs, ctx)?;
                writeln!(ctx.asm, "    pop rax")?;
                writeln!(ctx.asm, "    add rax, {}", member.offset)?;
                writeln!(ctx.asm, "    push rax")?;
                return Ok(());
            } else {
                Err(Error::not_found())
            }
            // gen_val(node, ctx: &mut Context)
        }
        _ => Err(Error::not_lvar()),
    }
}

// fn gen_gvar(gvar: &GvarMp) {
//     writeln!(ctx.asm,"# gen gval")?;

// }

fn load(node: &Node, ctx: &mut Context) -> Result<(), Error> {
    let mut word = "mov rax, [rax]";
    if let Ok(type_kind) = node.get_type() {
        match type_kind {
            TypeKind::Array(_, type_kind, _) => {
                word = gen_load_asm(type_kind.borrow().size(), true).unwrap_or(word)
            }
            x => word = gen_load_asm(x.size(), true).unwrap_or(word),
        }
    }
    writeln!(ctx.asm, "    pop rax")?;
    writeln!(ctx.asm, "    {}", word)?;
    writeln!(ctx.asm, "    push rax")?;
    Ok(())
}

/// generate asm depending on the size.
/// if ext is true, use sign extension
fn gen_load_asm(size: u64, signed: bool) -> Option<&'static str> {
    match size {
        1 => {
            if signed {
                Some("movsx rax, byte ptr [rax]")
            } else {
                Some("movzx rax, byte ptr [rax]")
            }
        }
        2 => Some("movsx rax, word ptr [rax]"),
        4 => Some("movsxd rax, dword ptr [rax]"),
        8 => Some("mov rax, [rax]"),
        _ => None,
    }
}

/// mov [rax], rdi
/// もし`node`の要素が`array`だったら、`array`の要素のサイズに合わせてstoreする
fn store(node: &Node, ctx: &mut Context) -> Result<(), Error> {
    let mut word = "mov [rax], rdi";
    writeln!(ctx.asm, "# store")?;
    writeln!(ctx.asm, "    pop rdi")?;
    writeln!(ctx.asm, "    pop rax")?;
    if let Ok(type_kind) = node.get_type() {
        match type_kind {
            TypeKind::Array(_, b_type, _) => {
                word = gen_store_asm(TypeKind::Ptr(b_type).size()).unwrap_or(word)
            }
            TypeKind::_Bool => {
                writeln!(ctx.asm, "    cmp rdi, 0")?;
                writeln!(ctx.asm, "  setne dil")?;
                writeln!(ctx.asm, "  movzb rdi, dil")?;
            }
            x => word = gen_store_asm(x.size()).unwrap_or(word),
        }
    }
    writeln!(ctx.asm, "    {}", word)?;
    writeln!(ctx.asm, "    push rdi")?;
    Ok(())
}

fn gen_store_asm(size: u64) -> Option<&'static str> {
    match size {
        1 => Some("mov [rax], dil"),
        2 => Some("mov [rax], di"),
        4 => Some("mov [rax], edi"),
        8 => Some("mov [rax], rdi"),
        _ => None,
    }
}

fn ptr_op(node: &Node, ctx: &mut Context) -> Result<(), Error> {
    writeln!(ctx.asm, "# ptr op")?;
    if let Some(ref lhs) = node.lhs {
        if let Ok(type_kind) = &lhs.get_type() {
            match type_kind {
                TypeKind::Ptr(ptr) | TypeKind::Array(_, ptr, _) => {
                    writeln!(ctx.asm, "    imul rdi, {}", ptr.borrow().size())?;
                }
                _ => (),
            }
        }
    }
    Ok(())
}

fn cast(type_kind: &TypeKind, ctx: &mut Context) -> Result<(), Error> {
    use TypeKind::*;

    writeln!(ctx.asm, "# cast")?;
    writeln!(ctx.asm, "    pop rax")?;
    if type_kind == &_Bool {
        writeln!(ctx.asm, "    cmp rax, 0")?;
        writeln!(ctx.asm, "    setne al")?;
    }

    match type_kind.size() {
        1 => writeln!(ctx.asm, "    movsx rax, al")?,
        2 => writeln!(ctx.asm, "    movsx rax, ax")?,
        4 => writeln!(ctx.asm, "    movsx rax, eax")?,
        _ => (),
    }
    writeln!(ctx.asm, "    push rax")?;
    Ok(())
}

pub fn inc(node: &Node, ctx: &mut Context) -> Result<(), Error> {
    writeln!(ctx.asm, "    pop rax")?;
    writeln!(ctx.asm, "    push rdi")?; // keep rdi
    writeln!(ctx.asm, "    mov rdi, 1")?;
    ptr_op(node, ctx)?;
    writeln!(ctx.asm, "    add rax, rdi")?;
    writeln!(ctx.asm, "    pop rdi")?;
    writeln!(ctx.asm, "    push rax")?;
    Ok(())
}

pub fn dec(node: &Node, ctx: &mut Context) -> Result<(), Error> {
    writeln!(ctx.asm, "    pop rax")?;
    writeln!(ctx.asm, "    push rdi")?; // keep rdi
    writeln!(ctx.asm, "    mov rdi, 1")?;
    ptr_op(node, ctx)?;
    writeln!(ctx.asm, "    sub rax, rdi")?;
    writeln!(ctx.asm, "    pop rdi")?;
    writeln!(ctx.asm, "    push rax")?;
    Ok(())
}
