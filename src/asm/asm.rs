use super::error::Error;
use crate::ast::{Initializer, Node, NodeKind, Program};
use crate::base_types::{self, TypeKind};
use std::fmt::Write;

// jump の連番とかを格納しておく
pub struct Context {
    jump_label: usize,
    break_label: usize,
    continue_label: usize,
    case_label: (usize, usize), // case_label, end_label
    func_name: String,
    asm: String,
}

impl Context {
    pub fn new() -> Self {
        Self {
            jump_label: 1,
            break_label: 0,
            continue_label: 0,
            case_label: (0, 0),
            func_name: String::new(),
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
        if gvar.dec.is_extern {
            continue;
        }
        if gvar.dec.is_static {
            writeln!(ctx.asm, ".local {}", name)?;
        } else {
            writeln!(ctx.asm, ".global {}", name)?;
        }
        writeln!(ctx.asm, "{}:", name)?;
        if gvar.init.len() == 0 {
            writeln!(ctx.asm, "    .zero {}", gvar.size)?;
        } else {
            for i in &gvar.init {
                match i {
                    Initializer::Label(label, addend) => {
                        writeln!(ctx.asm, "    .quad {}+{}", label, addend)?;
                    }
                    Initializer::Val(size, val) => {
                        if *size == 1 {
                            writeln!(ctx.asm, "    .byte {}", val)?;
                        } else {
                            writeln!(ctx.asm, "    .{}byte {}", size, val)?;
                        }
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
    // asm生成
    for function in program.functions {
        ctx.func_name = function.def.ident.name.clone();
        #[cfg(debug_assertions)]
        writeln!(ctx.asm, "# start prologue")?;
        if !function.is_static {
            writeln!(ctx.asm, ".global {}", function.def.ident.name)?;
        }

        writeln!(ctx.asm, "{}:", function.def.ident.name)?;
        // プロローグ
        writeln!(ctx.asm, "    push rbp")?;
        writeln!(ctx.asm, "    mov rbp, rsp")?;
        writeln!(ctx.asm, "    sub rsp, {}", function.get_all_var_size())?;

        // save arg registers if variadic
        if let Some(_) = function.va_area {
            let gp = function.def.param_num; //  `...`より前にある引数の数を入れる。
                                             // def.param_numは

            // let off = function
            //     .all_vars
            //     .as_ref()
            //     .map(|v| v.borrow().offset)
            //     .unwrap_or(0)
            //     - function.get_param_size();
            let off = 136;
            // va_elem
            // gp_offset
            writeln!(ctx.asm, "    mov dword ptr [rbp-{}], {}", off, gp * 8)?;
            // fp_offset
            writeln!(ctx.asm, "    mov dword ptr [rbp-{}], 0", off - 4)?;
            // overflow_area
            writeln!(ctx.asm, "    mov qword ptr [rbp-{}], 48", off - 8)?;
            // reg_save_area
            writeln!(ctx.asm, "    mov qword ptr [rbp-{}], rbp", off - 16)?;
            writeln!(
                ctx.asm,
                "    sub qword ptr [rbp-{}], {}",
                off - 16,
                off - 24
            )?;

            // __reg_save_area__
            writeln!(ctx.asm, "    mov qword ptr [rbp-{}], rdi", off - 24)?;
            writeln!(ctx.asm, "    mov qword ptr [rbp-{}], rsi", off - 32)?;
            writeln!(ctx.asm, "    mov qword ptr [rbp-{}], rdx", off - 40)?;
            writeln!(ctx.asm, "    mov qword ptr [rbp-{}], rcx", off - 48)?;
            writeln!(ctx.asm, "    mov qword ptr [rbp-{}], r8", off - 56)?;
            writeln!(ctx.asm, "    mov qword ptr [rbp-{}], r9", off - 64)?;
            writeln!(ctx.asm, "    movsd xmm0, [rbp-{}]", off - 72)?;
            writeln!(ctx.asm, "    movsd xmm1, [rbp-{}]", off - 80)?;
            writeln!(ctx.asm, "    movsd xmm2, [rbp-{}]", off - 88)?;
            writeln!(ctx.asm, "    movsd xmm3, [rbp-{}]", off - 96)?;
            writeln!(ctx.asm, "    movsd xmm4, [rbp-{}]", off - 104)?;
            writeln!(ctx.asm, "    movsd xmm5, [rbp-{}]", off - 112)?;
            writeln!(ctx.asm, "    movsd xmm6, [rbp-{}]", off - 120)?;
            writeln!(ctx.asm, "    movsd xmm7, [rbp-{}]", off - 128)?;
        }

        // 引数をローカル変数としてスタックに載せる
        let mut offset = function
            .all_vars
            .as_ref()
            .map(|v| v.borrow().offset)
            .unwrap_or(0)
            - function.get_param_size();

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

        #[cfg(debug_assertions)]
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
            #[cfg(debug_assertions)]
            writeln!(ctx.asm, "# number")?;
            if x > &(i32::MAX as i64) {
                writeln!(ctx.asm, "    movabs rax, {}", x)?;
                writeln!(ctx.asm, "    push rax")?;
            } else {
                writeln!(ctx.asm, "    push {}", x)?;
            }
            return Ok(());
        }
        NodeKind::Lvar(_) | NodeKind::Gvar(_) => {
            #[cfg(debug_assertions)]
            writeln!(ctx.asm, "# NodeKind::Lvar, Gvar")?;
            gen_val(node, ctx)?;
            match node.get_type() {
                Ok(TypeKind::Array(_, _, _)) | Ok(TypeKind::Struct(_)) => return Ok(()),
                _ => (),
            }
            load(node, ctx)?;
            return Ok(());
        }
        NodeKind::Member(_, member) => {
            #[cfg(debug_assertions)]
            writeln!(ctx.asm, "# NodeKind::Member")?;
            gen_val(node, ctx)?;
            if let TypeKind::Array(_, _, _) = &*member.get_type() {
                return Ok(());
            }
            load(node, ctx)?;
            return Ok(());
        }
        NodeKind::Assign => {
            #[cfg(debug_assertions)]
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
        NodeKind::AAdd
        | NodeKind::ASub
        | NodeKind::AMul
        | NodeKind::ADiv
        | NodeKind::ALShift
        | NodeKind::ARShift => {
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
                NodeKind::ALShift => {
                    writeln!(ctx.asm, "    mov cl, dil")?;
                    writeln!(ctx.asm, "    shl rax, cl")?;
                }
                NodeKind::ARShift => {
                    writeln!(ctx.asm, "    mov cl, dil")?;
                    writeln!(ctx.asm, "    sar rax, cl")?;
                }
                _ => unreachable!(),
            }
            writeln!(ctx.asm, "    push rax")?;
            store(node, ctx)?;
            return Ok(());
        }
        NodeKind::Return => {
            #[cfg(debug_assertions)]
            writeln!(ctx.asm, "# NodeKind::Return")?;
            if let Some(lhs) = &node.lhs {
                gen(&lhs, ctx)?;
                writeln!(ctx.asm, "    pop rax")?;
            }
            writeln!(ctx.asm, "    mov rsp, rbp")?;
            writeln!(ctx.asm, "    pop rbp")?;
            writeln!(ctx.asm, "    ret")?;
            return Ok(());
        }
        NodeKind::If => {
            #[cfg(debug_assertions)]
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
            #[cfg(debug_assertions)]
            writeln!(ctx.asm, "# NodeKind::While")?;
            let jlb_num = ctx.jump_label;
            let break_num = ctx.break_label;
            let continue_num = ctx.continue_label;
            ctx.jump_label += 1;
            ctx.break_label = jlb_num;
            ctx.continue_label = jlb_num;
            writeln!(ctx.asm, ".L.continue.{}:", jlb_num)?;
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
            writeln!(ctx.asm, "    jmp  .L.continue.{}", jlb_num)?;
            writeln!(ctx.asm, ".L.break.{}:", jlb_num)?;
            ctx.break_label = break_num;
            ctx.continue_label = continue_num;
            return Ok(());
        }
        NodeKind::Do => {
            #[cfg(debug_assertions)]
            writeln!(ctx.asm, "# NodeKind::Do")?;
            let jlb_num = ctx.jump_label;
            let break_num = ctx.break_label;
            let continue_num = ctx.continue_label;
            ctx.jump_label += 1;
            ctx.break_label = jlb_num;
            ctx.continue_label = jlb_num;
            writeln!(ctx.asm, ".L.continue.{}:", jlb_num)?;
            if let Some(then) = &node.then {
                gen(then, ctx)?;
            } else {
                return Err(Error::not_found());
            }
            if let Some(cond) = &node.cond {
                gen(cond, ctx)?;
            } else {
                return Err(Error::not_found());
            }
            writeln!(ctx.asm, "    pop rax")?;
            writeln!(ctx.asm, "    cmp rax, 0")?;
            writeln!(ctx.asm, "    je  .L.break.{}", jlb_num)?;

            writeln!(ctx.asm, "    jmp  .L.continue.{}", jlb_num)?;
            writeln!(ctx.asm, ".L.break.{}:", jlb_num)?;
            ctx.break_label = break_num;
            ctx.continue_label = continue_num;
            return Ok(());
        }
        NodeKind::For => {
            #[cfg(debug_assertions)]
            writeln!(ctx.asm, "# NodeKind::For")?;
            let jlb_num = ctx.jump_label;
            let break_num = ctx.break_label;
            let continue_num = ctx.continue_label;
            ctx.jump_label += 1;
            ctx.break_label = jlb_num;
            ctx.continue_label = jlb_num;
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

            writeln!(ctx.asm, ".L.continue.{}:", jlb_num)?;

            if let Some(inc) = &node.inc {
                gen(inc, ctx)?;
            }

            writeln!(ctx.asm, "    jmp  .Lbegin{}", jlb_num)?;
            writeln!(ctx.asm, ".L.break.{}:", jlb_num)?;
            ctx.break_label = break_num;
            ctx.continue_label = continue_num;
            return Ok(());
        }
        NodeKind::Break => {
            #[cfg(debug_assertions)]
            writeln!(ctx.asm, "# NodeKind::Break")?;
            match ctx.break_label {
                0 => return Err(Error::stray_break()),
                _ => {
                    writeln!(ctx.asm, "    jmp .L.break.{}", ctx.break_label)?;
                    return Ok(());
                }
            }
        }
        NodeKind::Continue => {
            #[cfg(debug_assertions)]
            writeln!(ctx.asm, "# NodeKind::Continue")?;
            match ctx.continue_label {
                0 => return Err(Error::stray_continue()),
                _ => {
                    writeln!(ctx.asm, "    jmp .L.continue.{}", ctx.continue_label)?;
                    return Ok(());
                }
            }
        }
        NodeKind::Goto(ident) => {
            #[cfg(debug_assertions)]
            writeln!(ctx.asm, "# NodeKind::Goto")?;
            writeln!(ctx.asm, "    jmp .L.label.{}.{}", ctx.func_name, ident.name)?;
            return Ok(());
        }
        NodeKind::Label(ident) => {
            #[cfg(debug_assertions)]
            writeln!(ctx.asm, "# NodeKind::Label")?;
            writeln!(ctx.asm, ".L.label.{}.{}:", ctx.func_name, ident.name)?;
            gen(&node.lhs.as_ref().unwrap(), ctx)?;
            return Ok(());
        }
        NodeKind::Switch(cases) => {
            #[cfg(debug_assertions)]
            writeln!(ctx.asm, "# NodeKind::Switch")?;
            let jlb_num = ctx.jump_label;
            let break_num = ctx.break_label;
            ctx.break_label = jlb_num;

            gen(node.cond.as_ref().unwrap(), ctx)?;
            writeln!(ctx.asm, "    pop rax")?;

            let mut has_default = -1;
            for case in cases {
                match case.kind {
                    NodeKind::Case(num) => {
                        writeln!(ctx.asm, "    cmp rax, {}", num)?;
                        writeln!(ctx.asm, "    je .L.case.{}", ctx.jump_label)?;
                        ctx.jump_label += 1;
                    }
                    NodeKind::DefaultCase => {
                        has_default = ctx.jump_label as isize;
                        ctx.jump_label += 1;
                    }
                    _ => return Err(Error::todo()),
                }
            }
            if has_default != -1 {
                writeln!(ctx.asm, "    jmp .L.case.{}", has_default)?;
                ctx.jump_label += 1;
            }

            writeln!(ctx.asm, "    jmp .L.break.{}", jlb_num)?;
            ctx.jump_label += 1;

            let org = std::mem::replace(&mut ctx.case_label, (jlb_num, jlb_num));
            gen(node.then.as_ref().unwrap(), ctx)?;
            ctx.case_label = org;

            writeln!(ctx.asm, ".L.break.{}:", jlb_num)?;
            ctx.break_label = break_num;
            return Ok(());
        }
        NodeKind::Case(_) | NodeKind::DefaultCase => {
            writeln!(ctx.asm, ".L.case.{}:", ctx.case_label.0)?;
            ctx.case_label.0 += 1;
            let mut case_lable = ctx.case_label;
            let mut node = node;
            loop {
                match node.lhs.as_ref().unwrap().kind {
                    NodeKind::Case(_) | NodeKind::DefaultCase => {
                        writeln!(ctx.asm, ".L.case.{}:", ctx.case_label.0)?;
                        ctx.case_label.0 += 1;
                        case_lable = ctx.case_label;
                        node = node.lhs.as_ref().unwrap();
                    }
                    _ => break,
                }
            }
            gen(node.lhs.as_ref().unwrap(), ctx)?;
            ctx.case_label = case_lable;
            return Ok(());
        }
        NodeKind::Block(stmts) | NodeKind::StmtExpr(stmts) => {
            #[cfg(debug_assertions)]
            writeln!(ctx.asm, "# NodeKind::Block,StmtExpr")?;
            for stmt in stmts {
                gen(stmt, ctx)?;
            }
            return Ok(());
        }
        NodeKind::Func(func_prototype, args) => {
            #[cfg(debug_assertions)]
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
            if func_prototype.as_ref().type_kind != TypeKind::Void {
                cast(&func_prototype.type_kind, ctx)?;
            }
            return Ok(());
        }
        NodeKind::Addr => {
            #[cfg(debug_assertions)]
            writeln!(ctx.asm, "# NodeKind::Addr")?;
            if let Some(lhs) = &node.lhs {
                gen_val(&lhs, ctx)?;
            } else {
                return Err(Error::not_found());
            }
            return Ok(());
        }
        NodeKind::Deref => {
            #[cfg(debug_assertions)]
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
            #[cfg(debug_assertions)]
            writeln!(ctx.asm, "# declaration")?;
            if let Some(ref init) = node.init {
                for i in init {
                    gen(i, ctx)?
                }
            }

            return Ok(());
        }
        NodeKind::ExprStmt => {
            #[cfg(debug_assertions)]
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
            #[cfg(debug_assertions)]
            writeln!(ctx.asm, "# cast")?;
            gen(&node.lhs.as_ref().unwrap(), ctx)?;
            cast(type_kind, ctx)?;
            return Ok(());
        }
        NodeKind::Comma => {
            #[cfg(debug_assertions)]
            writeln!(ctx.asm, "# comma")?;
            gen(node.lhs.as_ref().unwrap(), ctx)?;
            gen(node.rhs.as_ref().unwrap(), ctx)?;
        }
        NodeKind::PreInc => {
            #[cfg(debug_assertions)]
            writeln!(ctx.asm, "# preinc")?;
            gen_val(node.lhs.as_ref().unwrap(), ctx)?;
            writeln!(ctx.asm, "    push [rsp]")?;
            load(node, ctx)?;
            inc(node, ctx)?;
            store(node, ctx)?;
            return Ok(());
        }
        NodeKind::PreDec => {
            #[cfg(debug_assertions)]
            writeln!(ctx.asm, "# predec")?;
            gen_val(node.lhs.as_ref().unwrap(), ctx)?;
            writeln!(ctx.asm, "    push [rsp]")?;
            load(node, ctx)?;
            dec(node, ctx)?;
            store(node, ctx)?;
            return Ok(());
        }
        NodeKind::PostInc => {
            #[cfg(debug_assertions)]
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
            #[cfg(debug_assertions)]
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
            #[cfg(debug_assertions)]
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
            #[cfg(debug_assertions)]
            writeln!(ctx.asm, "# bit not")?;
            gen(node.lhs.as_ref().unwrap(), ctx)?;
            writeln!(ctx.asm, "    pop rax")?;
            writeln!(ctx.asm, "    not rax")?;
            writeln!(ctx.asm, "    push rax")?;
            return Ok(());
        }
        NodeKind::LogOr => {
            #[cfg(debug_assertions)]
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
            #[cfg(debug_assertions)]
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
        NodeKind::Ternary => {
            #[cfg(debug_assertions)]
            writeln!(ctx.asm, "# ternary")?;

            let jlb_num = ctx.jump_label;
            ctx.jump_label += 1;
            gen(node.cond.as_ref().unwrap(), ctx)?;
            writeln!(ctx.asm, "    pop rax")?;
            writeln!(ctx.asm, "    cmp rax, 0")?;
            writeln!(ctx.asm, "    je  .Lelse{}", jlb_num)?;
            gen(node.then.as_ref().unwrap(), ctx)?;
            writeln!(ctx.asm, "    jmp .Lend{}", jlb_num)?;
            writeln!(ctx.asm, ".Lelse{}:", jlb_num)?;
            gen(node.els.as_ref().unwrap(), ctx)?;
            writeln!(ctx.asm, ".Lend{}:", jlb_num)?;
            return Ok(());
        }
        _ => (),
    }

    if let Some(lhs) = &node.lhs {
        #[cfg(debug_assertions)]
        writeln!(ctx.asm, "# lhs")?;
        gen(lhs, ctx)?;
    }
    if let Some(rhs) = &node.rhs {
        #[cfg(debug_assertions)]
        writeln!(ctx.asm, "# rhs")?;
        gen(rhs, ctx)?;
    }

    #[cfg(debug_assertions)]
    writeln!(ctx.asm, "# pop")?;
    writeln!(ctx.asm, "    pop rdi")?;
    writeln!(ctx.asm, "    pop rax")?;

    match node.kind {
        NodeKind::Add => {
            #[cfg(debug_assertions)]
            writeln!(ctx.asm, "# Add")?;
            ptr_op(node, ctx)?;
            writeln!(ctx.asm, "    add rax, rdi")?;
        }
        NodeKind::Sub => {
            #[cfg(debug_assertions)]
            writeln!(ctx.asm, "# Sub")?;
            ptr_op(node, ctx)?;
            writeln!(ctx.asm, "    sub rax, rdi")?;
        }
        NodeKind::Mul => {
            #[cfg(debug_assertions)]
            writeln!(ctx.asm, "# Mul")?;
            ptr_op(node, ctx)?;
            writeln!(ctx.asm, "    imul rax, rdi")?;
        }
        NodeKind::Div => {
            #[cfg(debug_assertions)]
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
            #[cfg(debug_assertions)]
            writeln!(ctx.asm, "# Equal")?;
            writeln!(ctx.asm, "    cmp rax, rdi")?;
            writeln!(ctx.asm, "    sete al")?;
            writeln!(ctx.asm, "    movzb rax, al")?;
        }
        NodeKind::Leq => {
            #[cfg(debug_assertions)]
            writeln!(ctx.asm, "# Leq")?;
            writeln!(ctx.asm, "    cmp rax, rdi")?;
            writeln!(ctx.asm, "    setle al")?;
            writeln!(ctx.asm, "    movzb rax, al")?;
        }
        NodeKind::Lesser => {
            #[cfg(debug_assertions)]
            writeln!(ctx.asm, "# Lesser")?;
            writeln!(ctx.asm, "    cmp rax, rdi")?;
            writeln!(ctx.asm, "    setl al")?;
            writeln!(ctx.asm, "    movzb rax, al")?;
        }
        NodeKind::Neq => {
            #[cfg(debug_assertions)]
            writeln!(ctx.asm, "# Neq")?;
            writeln!(ctx.asm, "    cmp rax, rdi")?;
            writeln!(ctx.asm, "    setne al")?;
            writeln!(ctx.asm, "    movzb rax, al")?;
        }
        NodeKind::LShift => {
            writeln!(ctx.asm, "    mov cl, dil")?;
            writeln!(ctx.asm, "    shl rax, cl")?;
        }
        NodeKind::RShift => {
            writeln!(ctx.asm, "    mov cl, dil")?;
            writeln!(ctx.asm, "    sar rax, cl")?;
        }
        _ => (),
    }

    writeln!(ctx.asm, "    push rax")?;
    Ok(())
}

fn gen_val(node: &Node, ctx: &mut Context) -> Result<(), Error> {
    #[cfg(debug_assertions)]
    writeln!(ctx.asm, "# gen val")?;
    if !is_left_value(node) {
        return Err(Error::not_lvar());
    }

    match &node.kind {
        NodeKind::Lvar(x) => {
            #[cfg(debug_assertions)]
            writeln!(ctx.asm, "# lvar")?;
            writeln!(ctx.asm, "    mov rax, rbp")?;
            writeln!(ctx.asm, "    sub rax, {}", x.borrow().offset)?;
            writeln!(ctx.asm, "    push rax")?;
            Ok(())
        }
        NodeKind::Gvar(x) => {
            #[cfg(debug_assertions)]
            writeln!(ctx.asm, "# gvar")?;
            writeln!(ctx.asm, "    mov rax, OFFSET FLAT:{}", x.dec.ident.name)?;
            writeln!(ctx.asm, "    push rax")?;
            Ok(())
        }
        NodeKind::Deref => {
            #[cfg(debug_assertions)]
            writeln!(ctx.asm, "# deref")?;
            if let Some(lhs) = &node.lhs {
                gen(&lhs, ctx)
            } else {
                Err(Error::not_found())
            }
        }
        NodeKind::Member(_, member) => {
            #[cfg(debug_assertions)]
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
        _ => unreachable!(),
    }
}

fn is_left_value(node: &Node) -> bool {
    use NodeKind::*;
    match node.kind {
        Lvar(_) | Gvar(_) | Deref | Member(_, _) => true,
        _ => false,
    }
}

fn load(node: &Node, ctx: &mut Context) -> Result<(), Error> {
    #[cfg(debug_assertions)]
    writeln!(ctx.asm, "# load")?;
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
fn _store(node: &Node, ctx: &mut Context) -> Result<(), Error> {
    let mut word = "mov [rax], rdi";
    #[cfg(debug_assertions)]
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

/// 右辺の値が左辺値だった場合
/// ex: a = b;
fn store(node: &Node, ctx: &mut Context) -> Result<(), Error> {
    writeln!(ctx.asm, "# store left value")?;
    if let Some(rhs) = &node.rhs {
        if let Ok(t) = rhs.get_type() {
            match t {
                TypeKind::Struct(_struct) => {
                    writeln!(ctx.asm, "    pop rdi")?;
                    writeln!(ctx.asm, "    pop rax")?;
                    for i in 0.._struct.borrow().get_size() {
                        writeln!(ctx.asm, "    mov r8b, [rdi+{}]", i)?;
                        writeln!(ctx.asm, "    mov [rax+{}], r8b", i)?;
                    }
                    writeln!(ctx.asm, "    push rax")?;
                    return Ok(());
                }
                _ => {}
            }
        }
    }
    _store(node, ctx)
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
    #[cfg(debug_assertions)]
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

    #[cfg(debug_assertions)]
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
    #[cfg(debug_assertions)]
    writeln!(ctx.asm, "# inc")?;
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
    #[cfg(debug_assertions)]
    writeln!(ctx.asm, "# dec")?;
    writeln!(ctx.asm, "    pop rax")?;
    writeln!(ctx.asm, "    push rdi")?; // keep rdi
    writeln!(ctx.asm, "    mov rdi, 1")?;
    ptr_op(node, ctx)?;
    writeln!(ctx.asm, "    sub rax, rdi")?;
    writeln!(ctx.asm, "    pop rdi")?;
    writeln!(ctx.asm, "    push rax")?;
    Ok(())
}
