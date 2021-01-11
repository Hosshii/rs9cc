use super::error::Error;
use crate::ast::{Node, NodeKind, Program};
use crate::base_types::{self, TypeKind};

// jump の連番とかを格納しておく
pub struct Context {
    jump_label: usize,
    break_label: usize,
}

impl Context {
    pub fn new() -> Self {
        Self {
            jump_label: 1,
            break_label: 0,
        }
    }
}

const ARGREG1: [&str; 6] = ["dil", "sil", "dl", "cl", "r8b", "r9b"];
const ARGREG2: [&str; 6] = ["di", "si", "dx", "cx", "r8w", "r9w"];
const ARGREG4: [&str; 6] = ["edi", "esi", "edx", "ecx", "r8d", "r9d"];
const ARGREG8: [&str; 6] = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];

pub fn code_gen(program: Program) -> Result<(), Error> {
    // アセンブリの前半部分を出力
    println!(".intel_syntax noprefix");

    println!(".data");
    // define global variable
    for (name, gvar) in program.ctx.g.gvar_mp {
        println!("{}:", name);
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
                        println!("    .{} {}", word, x);
                    } else {
                        unreachable!();
                    }
                }
                for _ in 0..(size - gvar.init.len() as u64) {
                    println!("    .{} {}", word, 0);
                }
            }
            TypeKind::Ptr(_) => {
                if gvar.init.len() <= 0 {
                    println!("    .quad {}", 0);
                } else {
                    if let Ok(gvar) = &gvar.init[0].get_gvar() {
                        println!("    .quad {}", gvar.dec.ident.name);
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
                    println!("    .{} {}", word, 0);
                } else {
                    if let NodeKind::Num(x) = gvar.init[0].kind {
                        println!("    .{} {}", word, x);
                    } else {
                        println!("    .{} {}", word, 0);
                    }
                }
            }
        }
    }
    for (content, label) in program.ctx.g.tk_string {
        println!("{}:", label);
        for c in content.chars() {
            println!("    .byte {}", c as u8);
        }
    }

    let mut ctx = Context::new();
    println!(".text");
    println!(".global main");
    // asm生成
    for function in program.functions {
        println!("# start prologue");

        println!("{}:", function.def.ident.name);
        // プロローグ
        println!("    push rbp");
        println!("    mov rbp, rsp");
        println!("    sub rsp, {}", function.get_all_var_size());

        // 引数をローカル変数としてスタックに載せる
        let mut offset = 0;
        for i in 0..function.def.param_num {
            let type_kind = &function.def.params[i].type_kind;
            offset += type_kind.size();
            offset = base_types::align_to(offset, type_kind.align());
            println!("    mov rax, rbp");
            println!("    sub rax, {}", offset);
            let reg = match type_kind.size() {
                1 => ARGREG1[i],
                2 => ARGREG2[i],
                4 => ARGREG4[i],
                8 => ARGREG8[i],
                _ => unreachable!(),
            };
            println!("    mov [rax], {}", reg);
        }

        println!("# end prologue");

        for node in function.nodes {
            gen(&node, &mut ctx)?;
        }
        println!("    pop rax");
        // エピローグ
        // 最後の式の結果がRAXに残っているのでそれが返り値になる
        println!("    mov rsp, rbp");
        println!("    pop rbp");
        println!("    ret");
    }
    Ok(())
}

pub fn gen(node: &Node, ctx: &mut Context) -> Result<(), Error> {
    match &node.kind {
        NodeKind::Num(x) => {
            println!("# number");
            if x > &(u32::MAX as u64) {
                println!("    movabs rax, {}", x);
                println!("    push rax");
            } else {
                println!("    push {}", x);
            }
            return Ok(());
        }
        NodeKind::Lvar(_) | NodeKind::Gvar(_) => {
            println!("# NodeKind::Lvar, Gvar");
            gen_val(node, ctx)?;
            if let Ok(TypeKind::Array(_, _, _)) = node.get_type() {
                return Ok(());
            }
            load(node);
            return Ok(());
        }
        NodeKind::Member(_, member) => {
            println!("# NodeKind::Member");
            gen_val(node, ctx)?;
            if let TypeKind::Array(_, _, _) = &*member.get_type() {
                return Ok(());
            }
            load(node);
            return Ok(());
        }
        NodeKind::Assign => {
            println!("# NodeKind::Assign");
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

            store(node);
            return Ok(());
        }
        NodeKind::AAdd | NodeKind::ASub | NodeKind::AMul | NodeKind::ADiv => {
            let lhs = node.lhs.as_ref().expect("lhs not found");
            let rhs = node.rhs.as_ref().expect("rhs not found");
            gen_val(lhs, ctx)?;
            println!("    push [rsp]");
            load(lhs);
            gen(rhs, ctx)?;
            println!("    pop rdi");
            println!("    pop rax");
            match &node.kind {
                NodeKind::AAdd => {
                    ptr_op(node);
                    println!("    add rax, rdi");
                }
                NodeKind::ASub => {
                    ptr_op(node);
                    println!("    sub rax, rdi");
                }
                NodeKind::AMul => {
                    println!("    imul rax, rdi");
                }
                NodeKind::ADiv => {
                    println!("    cqo");
                    println!("    idiv rdi");
                }
                _ => unreachable!(),
            }
            println!("    push rax");
            store(node);
            return Ok(());
        }
        NodeKind::Return => {
            println!("# NodeKind::Return");
            if let Some(lhs) = &node.lhs {
                gen(&lhs, ctx)?;
            } else {
                return Err(Error::not_found());
            }
            println!("    pop rax");
            println!("    mov rsp, rbp");
            println!("    pop rbp");
            println!("    ret");
            return Ok(());
        }
        NodeKind::If => {
            println!("# NodeKind::If");
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
            return Ok(());
        }
        NodeKind::While => {
            println!("# NodeKind::While");
            let jlb_num = ctx.jump_label;
            let break_num = ctx.break_label;
            ctx.jump_label += 1;
            ctx.break_label = jlb_num;
            println!(".Lbegin{}:", jlb_num);
            if let Some(cond) = &node.cond {
                gen(cond, ctx)?;
            } else {
                return Err(Error::not_found());
            }

            println!("    pop rax");
            println!("    cmp rax, 0");
            println!("    je  .L.break.{}", jlb_num);
            if let Some(then) = &node.then {
                gen(then, ctx)?;
            } else {
                return Err(Error::not_found());
            }
            println!("    jmp  .Lbegin{}", jlb_num);
            println!(".L.break.{}:", jlb_num);
            ctx.break_label = break_num;
            return Ok(());
        }
        NodeKind::For => {
            println!("# NodeKind::For");
            let jlb_num = ctx.jump_label;
            let break_num = ctx.break_label;
            ctx.jump_label += 1;
            ctx.break_label = jlb_num;
            if let Some(init) = &node.init {
                for i in init {
                    gen(i, ctx)?;
                }
            }

            println!(".Lbegin{}:", jlb_num);
            if let Some(cond) = &node.cond {
                gen(cond, ctx)?;
                println!("    pop rax");
                println!("    cmp rax, 0");
                println!("    je  .L.break.{}", jlb_num);
            }

            if let Some(then) = &node.then {
                gen(then, ctx)?;
            } else {
                return Err(Error::not_found());
            }

            if let Some(inc) = &node.inc {
                gen(inc, ctx)?;
            }

            println!("    jmp  .Lbegin{}", jlb_num);
            println!(".L.break.{}:", jlb_num);
            ctx.break_label = break_num;
            return Ok(());
        }
        NodeKind::Break => match ctx.break_label {
            0 => return Err(Error::stray_break()),
            _ => {
                println!("    jmp .L.break.{}", ctx.break_label);
                return Ok(());
            }
        },
        NodeKind::Block(stmts) | NodeKind::StmtExpr(stmts) => {
            println!("# NodeKind::Block,StmtExpr");
            for stmt in stmts {
                gen(stmt, ctx)?;
            }
            return Ok(());
        }
        NodeKind::Func(func_prototype, args) => {
            println!("# NodeKind::Func");
            let jlb_num = ctx.jump_label;
            ctx.jump_label += 1;

            // printf 関数はalに浮動小数点数の引数の個数をいれる必要がある
            // 今はないので決め打ちで0にする
            println!("    mov al, 0");

            for i in args {
                gen(i, ctx)?;
            }
            for i in (0..args.len()).rev() {
                println!("    pop {}", ARGREG8[i]);
            }
            // 8の倍数じゃなかったら8の倍数にする
            println!("    mov rax, rsp");
            println!("    and rax, 7");
            println!("    jnz .LcallFour{}", jlb_num);
            println!("    sub rsp, 4");
            {
                // 16の倍数にしてcall
                println!("    mov rax, rsp");
                println!("    and rax, 15");
                println!("    jnz .LcallF{}", jlb_num);
                println!("    mov rax, 0");
                println!("    call {}", func_prototype.ident.name);
                println!("    add rsp, 4");
                println!("    jmp .LendF{}", jlb_num);
                println!(".LcallF{}:", jlb_num);
                println!("    sub rsp, 8");
                println!("    mov rax, 0");
                println!("    call {}", func_prototype.ident.name);
                println!("    add rsp, 12");
                println!(".LendF{}:", jlb_num);
                println!("    push rax");
            }
            println!("    jmp .Lend{}", jlb_num);

            println!(".LcallFour{}:", jlb_num);
            println!("    mov rax, 0");
            {
                // 16の倍数にしてcall
                println!("    mov rax, rsp");
                println!("    and rax, 15");
                println!("    jnz .LcallFH{}", jlb_num);
                println!("    mov rax, 0");
                println!("    call {}", func_prototype.ident.name);
                println!("    jmp .LendFH{}", jlb_num);
                println!(".LcallFH{}:", jlb_num);
                println!("    sub rsp, 8");
                println!("    mov rax, 0");
                println!("    call {}", func_prototype.ident.name);
                println!("    add rsp, 8");
                println!(".LendFH{}:", jlb_num);
                println!("    push rax");
            }
            println!(".Lend{}:", jlb_num);
            cast(&func_prototype.type_kind);
            return Ok(());
        }
        NodeKind::Addr => {
            println!("# NodeKind::Addr");
            if let Some(lhs) = &node.lhs {
                gen_val(&lhs, ctx)?;
            } else {
                return Err(Error::not_found());
            }
            return Ok(());
        }
        NodeKind::Deref => {
            println!("# NodeKind::Deref");
            if let Some(lhs) = &node.lhs {
                gen(&lhs, ctx)?;
                if let Ok(TypeKind::Array(_, _, _)) = node.get_type() {
                    return Ok(());
                }
                load(node);
            } else {
                return Err(Error::not_found());
            }
            return Ok(());
        }
        NodeKind::Declaration(_) => {
            println!("# declaration");
            if let Some(ref init) = node.init {
                for i in init {
                    gen(i, ctx)?
                }
            }

            return Ok(());
        }
        NodeKind::ExprStmt => {
            println!("# NodeKind::ExprStmt");
            if let Some(lhs) = &node.lhs {
                gen(&lhs, ctx)?;
                println!("    add rsp, 8");
            } else {
                return Err(Error::not_found());
            }
            return Ok(());
        }
        NodeKind::Null => return Ok(()),
        NodeKind::Cast(type_kind) => {
            println!("# cast");
            gen(&node.lhs.as_ref().unwrap(), ctx)?;
            cast(type_kind);
            return Ok(());
        }
        NodeKind::Comma => {
            println!("# comma");
            gen(node.lhs.as_ref().unwrap(), ctx)?;
            gen(node.rhs.as_ref().unwrap(), ctx)?;
        }
        NodeKind::PreInc => {
            println!("# preinc");
            gen_val(node.lhs.as_ref().unwrap(), ctx)?;
            println!("    push [rsp]");
            load(node);
            inc(node);
            store(node);
            return Ok(());
        }
        NodeKind::PreDec => {
            println!("# predec");
            gen_val(node.lhs.as_ref().unwrap(), ctx)?;
            println!("    push [rsp]");
            load(node);
            dec(node);
            store(node);
            return Ok(());
        }
        NodeKind::PostInc => {
            println!("# postinc");
            gen_val(node.lhs.as_ref().unwrap(), ctx)?;
            println!("    push [rsp]");
            load(node);
            inc(node);
            store(node);
            dec(node);
            return Ok(());
        }
        NodeKind::PostDec => {
            println!("# postdec");
            gen_val(node.lhs.as_ref().unwrap(), ctx)?;
            println!("    push [rsp]");
            load(node);
            dec(node);
            store(node);
            inc(node);
            return Ok(());
        }
        NodeKind::Not => {
            println!("# not");
            gen(node.lhs.as_ref().unwrap(), ctx)?;
            println!("    pop rax");
            println!("    cmp rax, 0");
            println!("    sete al");
            println!("    movzb rax, al");
            println!("    push rax");
            return Ok(());
        }
        NodeKind::BitNot => {
            println!("# bit not");
            gen(node.lhs.as_ref().unwrap(), ctx)?;
            println!("    pop rax");
            println!("    not rax");
            println!("    push rax");
            return Ok(());
        }
        NodeKind::LogOr => {
            println!("# log or");
            let jlb_num = ctx.jump_label;
            ctx.jump_label += 1;
            gen(node.lhs.as_ref().unwrap(), ctx)?;
            println!("    pop rax");
            println!("    cmp rax, 0");
            println!("    jne  .Ltrue{}", jlb_num);
            gen(node.rhs.as_ref().unwrap(), ctx)?;
            println!("    pop rax");
            println!("    cmp rax, 0");
            println!("    jne  .Ltrue{}", jlb_num);
            println!("    push 0");
            println!("    jmp .Lend{}", jlb_num);
            println!(".Ltrue{}:", jlb_num);
            println!("    push 1");
            println!(".Lend{}:", jlb_num);
            return Ok(());
        }
        NodeKind::LogAnd => {
            println!("# log and");
            let jlb_num = ctx.jump_label;
            ctx.jump_label += 1;
            gen(node.lhs.as_ref().unwrap(), ctx)?;
            println!("    pop rax");
            println!("    cmp rax, 0");
            println!("    je  .Lfalse{}", jlb_num);
            gen(node.rhs.as_ref().unwrap(), ctx)?;
            println!("    pop rax");
            println!("    cmp rax, 0");
            println!("    je  .Lfalse{}", jlb_num);
            println!("    push 1");
            println!("    jmp .Lend{}", jlb_num);
            println!(".Lfalse{}:", jlb_num);
            println!("    push 0");
            println!(".Lend{}:", jlb_num);
            return Ok(());
        }
        _ => (),
    }

    if let Some(lhs) = &node.lhs {
        println!("# lhs");
        gen(lhs, ctx)?;
    }
    if let Some(rhs) = &node.rhs {
        println!("# rhs");
        gen(rhs, ctx)?;
    }

    println!("# pop");
    println!("    pop rdi");
    println!("    pop rax");

    match node.kind {
        NodeKind::Add => {
            println!("# Add");
            ptr_op(node);
            println!("    add rax, rdi");
        }
        NodeKind::Sub => {
            println!("# Sub");
            ptr_op(node);
            println!("    sub rax, rdi");
        }
        NodeKind::Mul => {
            println!("# Mul");
            ptr_op(node);
            println!("    imul rax, rdi");
        }
        NodeKind::Div => {
            println!("# Div");
            println!("    cqo");
            println!("    idiv rdi");
        }
        NodeKind::BitAnd => {
            println!("    and rax, rdi");
        }
        NodeKind::BitOr => {
            println!("    or rax, rdi");
        }
        NodeKind::BitXor => {
            println!("    xor rax, rdi");
        }
        NodeKind::Equal => {
            println!("# Equal");
            println!("    cmp rax, rdi");
            println!("    sete al");
            println!("    movzb rax, al");
        }
        NodeKind::Leq => {
            println!("# Leq");
            println!("    cmp rax, rdi");
            println!("    setle al");
            println!("    movzb rax, al");
        }
        NodeKind::Lesser => {
            println!("# Lesser");
            println!("    cmp rax, rdi");
            println!("    setl al");
            println!("    movzb rax, al");
        }
        NodeKind::Neq => {
            println!("# Neq");
            println!("    cmp rax, rdi");
            println!("    setne al");
            println!("    movzb rax, al");
        }
        _ => (),
    }

    println!("    push rax");
    Ok(())
}

fn gen_val(node: &Node, ctx: &mut Context) -> Result<(), Error> {
    println!("# gen val");
    match &node.kind {
        NodeKind::Lvar(x) => {
            println!("# lvar");
            println!("    mov rax, rbp");
            println!("    sub rax, {}", x.offset);
            println!("    push rax");
            Ok(())
        }
        NodeKind::Gvar(x) => {
            println!("# gvar");
            println!("    mov rax, OFFSET FLAT:{}", x.dec.ident.name);
            println!("    push rax");
            Ok(())
        }
        NodeKind::Deref => {
            println!("# deref");
            if let Some(lhs) = &node.lhs {
                gen(&lhs, ctx)
            } else {
                Err(Error::not_found())
            }
        }
        NodeKind::Member(_, member) => {
            println!("# member");
            if let Some(lhs) = &node.lhs {
                gen_val(&lhs, ctx)?;
                println!("    pop rax");
                println!("    add rax, {}", member.offset);
                println!("    push rax");
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
//     println!("# gen gval");

// }

fn load(node: &Node) {
    let mut word = "mov rax, [rax]";
    if let Ok(type_kind) = node.get_type() {
        match type_kind {
            TypeKind::Array(_, type_kind, _) => {
                word = gen_load_asm(type_kind.borrow().size(), true).unwrap_or(word)
            }
            x => word = gen_load_asm(x.size(), true).unwrap_or(word),
        }
    }
    println!("    pop rax");
    println!("    {}", word);
    println!("    push rax");
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
fn store(node: &Node) {
    let mut word = "mov [rax], rdi";
    println!("# store");
    println!("    pop rdi");
    println!("    pop rax");
    if let Ok(type_kind) = node.get_type() {
        match type_kind {
            TypeKind::Array(_, b_type, _) => {
                word = gen_store_asm(TypeKind::Ptr(b_type).size()).unwrap_or(word)
            }
            TypeKind::_Bool => {
                println!("    cmp rdi, 0");
                println!("  setne dil");
                println!("  movzb rdi, dil");
            }
            x => word = gen_store_asm(x.size()).unwrap_or(word),
        }
    }
    println!("    {}", word);
    println!("    push rdi");
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

fn ptr_op(node: &Node) {
    println!("# ptr op");
    if let Some(ref lhs) = node.lhs {
        if let Ok(type_kind) = &lhs.get_type() {
            match type_kind {
                TypeKind::Ptr(ptr) | TypeKind::Array(_, ptr, _) => {
                    println!("    imul rdi, {}", ptr.borrow().size());
                }
                _ => (),
            }
        }
    }
}

fn cast(type_kind: &TypeKind) {
    use TypeKind::*;

    println!("# cast");
    println!("    pop rax");
    if type_kind == &_Bool {
        println!("    cmp rax, 0");
        println!("    setne al");
    }

    match type_kind.size() {
        1 => println!("    movsx rax, al"),
        2 => println!("    movsx rax, ax"),
        4 => println!("    movsx rax, eax"),
        _ => (),
    }
    println!("    push rax");
}

pub fn inc(node: &Node) {
    println!("    pop rax");
    println!("    push rdi"); // keep rdi
    println!("    mov rdi, 1");
    ptr_op(node);
    println!("    add rax, rdi");
    println!("    pop rdi");
    println!("    push rax");
}

pub fn dec(node: &Node) {
    println!("    pop rax");
    println!("    push rdi"); // keep rdi
    println!("    mov rdi, 1");
    ptr_op(node);
    println!("    sub rax, rdi");
    println!("    pop rdi");
    println!("    push rax");
}
