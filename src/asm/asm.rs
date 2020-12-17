use super::error::Error;
use crate::ast::{Node, NodeKind, Program};
use crate::base_types::TypeKind;

// jump の連番とかを格納しておく
pub struct Context {
    jump_label: usize,
}

impl Context {
    pub fn new() -> Self {
        Self { jump_label: 0 }
    }
}

const _ARGREG1: [&str; 6] = ["dil", "sil", "dl", "cl", "r8b", "r9b"];
const _ARGREG2: [&str; 6] = ["di", "si", "dx", "cx", "r8w", "r9w"];
const _ARGREG4: [&str; 6] = ["edi", "esi", "edx", "ecx", "r8d", "r9d"];
const ARGREG8: [&str; 6] = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];

pub fn code_gen(program: Program) -> Result<(), Error> {
    // アセンブリの前半部分を出力
    println!(".intel_syntax noprefix");

    println!(".data");
    // define global variable
    for (name, gvar) in program.ctx.g.gvar_mp {
        println!("{}:", name);
        println!("    .zero {}", gvar.size);
    }
    for (content, label) in program.ctx.g.tk_string {
        println!("{}:", label);
        println!("    .string \"{}\"", content);
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
        // スタックのpush popが8バイト単位なのでとりあえず引数はint とかも8バイトにする
        // println!("    sub rsp, {}", function.get_all_var_size());
        println!("    sub rsp, {}", function._get_all_var_size());

        // 引数をローカル変数としてスタックに載せる
        let mut offset = 0;
        for i in 0..function.def.param_num {
            offset += function.def.params[i].base_type.kind.eight_size();
            println!("    mov rax, rbp");
            // スタックのpush popが8バイト単位なのでとりあえずint とかも8バイトにする
            // println!("    sub rax, {}", function.params[i].base_type.kind.size());
            println!("    sub rax, {}", offset);
            println!("    mov [rax], {}", ARGREG8[i]);
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
            println!("    push {}", x);
            return Ok(());
        }
        NodeKind::Lvar(lvar) => {
            println!("# NodeKind::Lvar");
            gen_val(node, ctx)?;
            if let TypeKind::Array(_, _) = lvar.dec.base_type.kind {
                return Ok(());
            }
            load(node);
            return Ok(());
        }
        NodeKind::Gvar(gvar) => {
            println!("# NodeKind::Gvar");
            gen_val(node, ctx)?;
            if let TypeKind::Array(_, _) = gvar.dec.base_type.kind {
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
            return Ok(());
        }
        NodeKind::For => {
            println!("# NodeKind::For");
            let jlb_num = ctx.jump_label;
            ctx.jump_label += 1;
            if let Some(init) = &node.init {
                gen(init, ctx)?;
            }

            println!(".Lbegin{}:", jlb_num);
            if let Some(cond) = &node.cond {
                gen(cond, ctx)?;
            }

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
            return Ok(());
        }
        NodeKind::Block(stmts) => {
            println!("# NodeKind::Block");
            for stmt in stmts {
                gen(stmt, ctx)?;
            }
            return Ok(());
        }
        NodeKind::Func(func_def, args) => {
            println!("# NodeKind::Func");
            let jlb_num = ctx.jump_label;
            ctx.jump_label += 1;
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
                println!("    call {}", func_def.ident.name);
                println!("    add rsp, 4");
                println!("    jmp .LendF{}", jlb_num);
                println!(".LcallF{}:", jlb_num);
                println!("    sub rsp, 8");
                println!("    mov rax, 0");
                println!("    call {}", func_def.ident.name);
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
                println!("    call {}", func_def.ident.name);
                println!("    jmp .LendFH{}", jlb_num);
                println!(".LcallFH{}:", jlb_num);
                println!("    sub rsp, 8");
                println!("    mov rax, 0");
                println!("    call {}", func_def.ident.name);
                println!("    add rsp, 8");
                println!(".LendFH{}:", jlb_num);
                println!("    push rax");
            }
            println!(".Lend{}:", jlb_num);
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
                load(lhs);
            } else {
                return Err(Error::not_found());
            }
            return Ok(());
        }
        NodeKind::Declaration(_) => return Ok(()),
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
        _ => Err(Error::not_lvar()),
    }
}

// fn gen_gvar(gvar: &GvarMp) {
//     println!("# gen gval");

// }

fn load(node: &Node) {
    println!("# load");
    let mut word = "mov rax, [rax]";
    if let Ok(type_kind) = node.get_type() {
        match type_kind {
            TypeKind::Array(_, b_type) => {
                word = gen_load_asm(b_type.kind.size(), true).unwrap_or(word)
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
        4 => Some("movsxd rax, dword ptr [rax]"),
        8 => Some("mov rax, [rax]"),
        _ => None,
    }
}

fn store(node: &Node) {
    let mut word = "mov [rax], rdi";
    if let Ok(type_kind) = node.get_type() {
        match type_kind {
            TypeKind::Array(_, b_type) => word = gen_store_asm(b_type.kind.size()).unwrap_or(word),
            x => word = gen_store_asm(x.size()).unwrap_or(word),
        }
    }
    println!("# store");
    println!("    pop rdi");
    println!("    pop rax");
    println!("    {}", word);
    println!("    push rdi");
}

fn gen_store_asm(size: u64) -> Option<&'static str> {
    match size {
        1 => Some("mov [rax], dil"),
        4 => Some("mov [rax], edi"),
        8 => Some("mov [rax], rdi"),
        _ => None,
    }
}

fn ptr_op(node: &Node) {
    println!("# ptr op");
    if let Some(ref lhs) = node.lhs {
        if let NodeKind::Lvar(ref lvar) = lhs.kind {
            match &lvar.dec.base_type.kind {
                TypeKind::Ptr(ptr) | TypeKind::Array(_, ptr) => {
                    println!("    imul rdi, {}", ptr.kind.size());
                }
                _ => (),
            }
        }
    }
}
