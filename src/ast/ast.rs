use super::error::Error;
use super::util::*;
use super::NodeKind;
use super::{
    Context, Declaration, Designator, FuncPrototype, Function, Gvar, Ident, Initializer,
    LocalContext, Lvar, Node, Program, Var,
};
use crate::base_types::{self, Enum, Member, Struct, TagTypeKind, TypeKind};
use crate::token::{Block, KeyWord, Operator, TokenIter, TokenKind};
use std::rc::Rc;
use std::{cell::RefCell, cmp::min};

// program         = (function | declaration ("=" initialize)? ";" | func-prototype )*
pub fn program(iter: &mut TokenIter) -> Result<Program, Error> {
    let mut program = Program::new();
    let mut ctx = &mut program.ctx;
    while iter.peek() != None {
        // to distinguish global variable and function
        // read base type and ident
        // then peek next token.
        // if next token is ; or [], it is global variable
        // if next token is ( , it is function

        let (type_kind, (is_typedef, is_static, is_extern)) = type_specifier(iter, ctx)?;
        let type_kind = Rc::new(RefCell::new(type_kind.clone()));
        let mut ident = Ident::new_anonymous();
        let type_kind = declarator(iter, ctx, type_kind, &mut ident)?;

        if let Some(next) = iter.next() {
            match &next.kind {
                // function
                TokenKind::Reserved(Operator::LParen) => {
                    let mut fn_params = Vec::new();
                    if !consume(iter, Operator::RParen) {
                        fn_params = params(iter, ctx)?;
                    }

                    let func_prototype = FuncPrototype::new(
                        type_kind.replace(TypeKind::Int),
                        ident,
                        fn_params,
                        is_static,
                    );
                    let checked_func_prototype = Rc::new(check_func_prototype(
                        iter,
                        &ctx.g.func_prototype_mp,
                        func_prototype,
                    )?);
                    ctx.g.func_prototype_mp.insert(
                        checked_func_prototype.ident.name.clone(),
                        checked_func_prototype.clone(),
                    );
                    if consume_semi(iter) {
                        continue;
                    }
                    let sc = ctx.s.enter();
                    let func = function(iter, checked_func_prototype, &mut ctx)?;
                    ctx.s.leave(sc);
                    program.functions.push(func);
                }
                x => {
                    let mut init = Vec::new();
                    if x == &TokenKind::Reserved(Operator::Assign) {
                        gvar_initializer(iter, ctx, &mut init, type_kind.clone())?;
                        expect_semi(iter)?;
                    } else if x != &TokenKind::SemiColon {
                        return Err(Error::unexpected_token(
                            iter.filepath,
                            iter.s,
                            next,
                            TokenKind::SemiColon,
                        ));
                    }
                    let mut dec = Declaration::new(type_kind.borrow().clone(), ident);
                    dec.is_typedef = is_typedef;
                    dec.is_static = is_static;
                    dec.is_extern = is_extern;
                    if is_typedef {
                        let result = ctx.s.insert_t(
                            Rc::new(dec.ident.clone()),
                            TagTypeKind::Typedef(Rc::new(dec.clone())),
                        );
                        if let Some(_) = result {
                            return Err(Error::re_declare(
                                iter.filepath,
                                iter.s,
                                dec.ident.clone(),
                                iter.pos,
                                None,
                            ));
                        }
                        continue;
                    }
                    ctx.insert_g(Rc::new(check_g_var(iter, &ctx.g.gvar_mp, dec, init)?));
                }
            }
        }
    }
    Ok(program)
}

pub fn gvar_initializer(
    iter: &mut TokenIter,
    ctx: &mut Context,
    initializers: &mut Vec<Initializer>,
    type_kind: Rc<RefCell<TypeKind>>,
) -> Result<(), Error> {
    if let TypeKind::Array(size, base, is_sized) = &mut *type_kind.borrow_mut() {
        if &*base.borrow() == &TypeKind::Char {
            if let Some(string) = consume_string(iter) {
                if !*is_sized {
                    *is_sized = true;
                    *size = string.len() as u64;
                }
                let len = min(string.len() as u64, *size);
                let string = string.as_bytes();
                let mut i = 0;

                while i < len {
                    new_init_val(initializers, 1, string[i as usize] as i64);
                    i += 1;
                }
                if i < *size {
                    new_init_zero(initializers, base.borrow().size() * (*size - i));
                }
                return Ok(());
            }
        }
    }

    match &mut *type_kind.borrow_mut() {
        TypeKind::Array(size, base, is_sized) => {
            let open = consume_block(iter, Block::LParen);
            let mut i = 0;
            let limit = if !*is_sized { u64::MAX } else { *size };
            if !peek_end(iter) {
                while {
                    gvar_initializer(iter, ctx, initializers, base.clone())?;
                    i += 1;
                    i < limit && !peek_end(iter) && consume_comma(iter)
                } {}
            }

            if open && !consume_end(iter) {
                skip_excess_elements(iter, ctx)?;
            }
            if i < *size {
                new_init_zero(initializers, base.borrow().size() * (*size - i));
            }

            if !*is_sized {
                *is_sized = true;
                *size = i;
            }
            return Ok(());
        }
        TypeKind::Struct(_struct) => {
            let open = consume_block(iter, Block::LParen);
            let members = _struct.borrow().members.clone();
            let mut i = 0;
            if !peek_end(iter) {
                while {
                    gvar_initializer(
                        iter,
                        ctx,
                        initializers,
                        Rc::new(RefCell::new(members[i].type_kind.as_ref().clone())), // todo
                    )?;
                    let next = if members.len() > i + 1 {
                        Some(members[i + 1].clone())
                    } else {
                        None
                    };
                    emit_struct_padding(
                        initializers,
                        _struct.borrow().get_size(),
                        members[i].clone(),
                        next,
                    );
                    i += 1;

                    i < members.len() && !peek_end(iter) && consume_comma(iter)
                } {}
            }
            if open && !consume_end(iter) {
                skip_excess_elements(iter, ctx)?;
            }
            if members.len() > i {
                let size = type_kind.borrow().size() - members[i].offset;
                new_init_zero(initializers, size);
            }
            return Ok(());
        }
        _ => (),
    }

    let open = consume_block(iter, Block::LParen);
    let mut node = conditional(iter, ctx)?;
    if open {
        expect_end(iter)?;
    }
    let mut var = None;
    let addend = eval2(&mut node, &mut var)?;

    match var {
        Some(gvar) => {
            new_init_label(initializers, gvar.dec.ident.name, addend);
            return Ok(());
        }
        None => {
            new_init_val(initializers, type_kind.borrow().size(), addend);
            return Ok(());
        }
    }
}

// type-specifier  = builtin-type | struct-dec | typedef-name | enum-specifier"
// builtin-type    = "void"
//                 | "_Bool"
//                 | "char"
//                 | "short" | "short" "int" | "int" "short"
//                 | "int"
//                 | "long" | "int" "long" | "long" "int"
// static, typedef and extern can appear anywhere in type-specifier
pub fn type_specifier(
    iter: &mut TokenIter,
    ctx: &mut Context,
) -> Result<(TypeKind, (bool, bool, bool)), Error> {
    let mut ty_vec = Vec::new();
    let mut is_typedef = false;
    let mut is_static = false;
    let mut is_extern = false;
    let mut ty = None;
    while let Some(x) = iter.peek() {
        if let TokenKind::TypeKind(ref type_kind) = x.kind {
            iter.next();
            ty_vec.push(type_kind.clone());
        } else if x.kind == TokenKind::KeyWord(KeyWord::Struct) {
            return Ok((
                TypeKind::Struct(struct_dec(iter, ctx)?),
                (is_typedef, is_static, is_extern),
            ));
        } else if x.kind == TokenKind::KeyWord(KeyWord::Enum) {
            return Ok((
                TypeKind::Enum(enum_specifier(iter, ctx)?),
                (is_typedef, is_static, is_extern),
            ));
        } else if x.kind == TokenKind::KeyWord(KeyWord::Typedef) {
            iter.next();
            is_typedef = true;
            continue;
        } else if x.kind == TokenKind::KeyWord(KeyWord::Static) {
            iter.next();
            is_static = true;
            continue;
        } else if x.kind == TokenKind::KeyWord(KeyWord::Extern) {
            iter.next();
            is_extern = true;
            continue;
        } else {
            if let Some(xx) = ty {
                return Ok((xx, (is_typedef, is_static, is_extern)));
            }
            if let TokenKind::Ident(ref ident) = x.kind {
                let ident = Rc::new(Ident::from(ident.clone()));
                if let Some(dec) = is_typedef_name(ident, ctx) {
                    iter.next();
                    return Ok((dec.type_kind.clone(), (is_typedef, is_static, is_extern)));
                }

                // else {
                //     return Err(Error::undefined_tag(
                //         iter.filepath,
                //         iter.s,
                //         iter.pos,
                //         ident,
                //         None,
                //     ));
                // }
            }
        }
        {
            use TypeKind::*;
            let type_kind = match ty_vec.as_slice() {
                [Void] => Void,
                [_Bool] => _Bool,
                [Char] => Char,
                [Short] | [Short, Int] | [Int, Short] => Short,
                [Int] => Int,
                [Long] | [Long, Int] | [Int, Long] => Long,
                [] if is_typedef => match ty {
                    Some(x) => x,
                    None => Int,
                },
                _ => {
                    iter.prev();
                    return Err(Error::unexpected_token(
                        iter.filepath,
                        iter.s,
                        x.clone(),
                        TokenKind::TypeKind(base_types::TypeKind::Int),
                    ));
                }
            };
            ty = Some(type_kind);
        }
    }
    Err(Error::eof(
        iter.filepath,
        iter.s,
        iter.pos,
        TokenKind::TypeKind(base_types::TypeKind::Int),
        None,
    ))
}

// declarator      = "*"* ("(" declarator ")" | ident) type-suffix
pub fn declarator(
    iter: &mut TokenIter,
    ctx: &mut Context,
    mut type_kind: Rc<RefCell<TypeKind>>,
    ident: &mut Ident,
) -> Result<Rc<RefCell<TypeKind>>, Error> {
    loop {
        if consume(iter, Operator::Mul) {
            type_kind = Rc::new(RefCell::new(TypeKind::Ptr(type_kind)));
        } else {
            break;
        }
    }

    if consume(iter, Operator::LParen) {
        let placeholder = Rc::new(RefCell::new(TypeKind::PlaceHolder));
        let new = declarator(iter, ctx, placeholder.clone(), ident)?;
        expect(iter, Operator::RParen)?;
        *placeholder.borrow_mut() = type_suffix(iter, ctx, type_kind)?.borrow().clone();
        return Ok(new);
    }
    *ident = expect_ident(iter)?;
    type_suffix(iter, ctx, type_kind)
}

// abstract-declarator     = "*"* ("(" declarator ")")? type-suffix
pub fn abstract_declarator(
    iter: &mut TokenIter,
    ctx: &mut Context,
    mut type_kind: Rc<RefCell<TypeKind>>,
) -> Result<Rc<RefCell<TypeKind>>, Error> {
    loop {
        if consume(iter, Operator::Mul) {
            type_kind = Rc::new(RefCell::new(TypeKind::Ptr(type_kind)));
        } else {
            break;
        }
    }

    if consume(iter, Operator::LParen) {
        let placeholder = Rc::new(RefCell::new(TypeKind::PlaceHolder));
        let new = abstract_declarator(iter, ctx, placeholder.clone())?;
        expect(iter, Operator::RParen)?;
        *placeholder.borrow_mut() = type_suffix(iter, ctx, type_kind)?.borrow().clone();
        return Ok(new);
    }
    type_suffix(iter, ctx, type_kind)
}

// type-suffix     = ("[" const-expr? "]" type-suffix)?
pub fn type_suffix(
    iter: &mut TokenIter,
    ctx: &mut Context,
    type_kind: Rc<RefCell<TypeKind>>,
) -> Result<Rc<RefCell<TypeKind>>, Error> {
    if !consume(iter, Operator::LArr) {
        return Ok(type_kind);
    }
    if consume(iter, Operator::RArr) {
        let type_kind = type_suffix(iter, ctx, type_kind)?;
        return Ok(Rc::new(RefCell::new(TypeKind::array_of(
            0, type_kind, false,
        ))));
    }
    let idx = const_expr(iter, ctx)?;
    expect(iter, Operator::RArr)?;
    let type_kind = type_suffix(iter, ctx, type_kind)?;
    return Ok(Rc::new(RefCell::new(TypeKind::array_of(
        idx as u64, type_kind, true,
    ))));
}

// type-name               = type-specifier abstract-declarator type-suffix
pub fn type_name(iter: &mut TokenIter, ctx: &mut Context) -> Result<Rc<RefCell<TypeKind>>, Error> {
    let (type_kind, _) = type_specifier(iter, ctx)?;
    let type_kind = Rc::new(RefCell::new(type_kind));
    let type_kind = abstract_declarator(iter, ctx, type_kind)?;
    type_suffix(iter, ctx, type_kind)
}

// struct-dec      = "struct" ident? ("{" declaration ";" "}")?
pub fn struct_dec(iter: &mut TokenIter, ctx: &mut Context) -> Result<Rc<RefCell<Struct>>, Error> {
    expect_keyword(iter, KeyWord::Struct)?;
    let ident = consume_ident(iter);

    if let Some(ident) = &ident {
        if !consume_block(iter, Block::LParen) {
            if let Some(tag) = ctx.s.find_upper_tag(Rc::new(ident.clone())) {
                if let TagTypeKind::Struct(_struct) = tag.as_ref() {
                    return Ok(_struct.clone());
                } else {
                    dbg!("not a struct tag");
                    return Err(Error::todo(iter.filepath, iter.s, iter.pos));
                }
            } else {
                let ident = Rc::new(ident.clone());
                let mut _struct = Struct::new(ident.clone(), Rc::new(Vec::new()));

                _struct.is_incomplete = true;
                let _struct = Rc::new(RefCell::new(_struct));
                ctx.s
                    .insert_t(ident.clone(), TagTypeKind::Struct(_struct.clone()));

                return Ok(_struct);
            }
        } else {
            iter.pos.bytes -= 1;
            iter.pos.tk -= 1;
        }
    }

    if !consume_block(iter, Block::LParen) {
        let mut _struct = Struct::new_anonymous(Rc::new(Vec::new()));
        _struct.is_incomplete = true;
        return Ok(Rc::new(RefCell::new(_struct)));
    }

    let _struct = if let Some(ident) = &ident {
        if let Some(tag) = ctx.s.find_upper_tag(Rc::new(ident.clone())) {
            if let TagTypeKind::Struct(_struct) = tag.as_ref() {
                _struct.clone()
            } else {
                dbg!("not a struct tag");
                return Err(Error::todo(iter.filepath, iter.s, iter.pos));
            }
        } else {
            let mut _struct = Struct::new(Rc::new(ident.clone()), Rc::new(Vec::new()));
            _struct.is_incomplete = true;
            let _struct = Rc::new(RefCell::new(_struct));
            ctx.s
                .insert_t(Rc::new(ident.clone()), TagTypeKind::Struct(_struct.clone()));

            _struct
        }
    } else {
        Rc::new(RefCell::new(Struct::new_anonymous(Rc::new(Vec::new()))))
    };

    let sc = ctx.s.enter();
    let mut members = Vec::new();
    while !consume_block(iter, Block::RParen) {
        members.push(declaration(iter, ctx)?);
        expect_semi(iter)?;
    }
    ctx.s.leave(sc);
    let mut offset = 0;
    let members: Vec<Rc<Member>> = members
        .into_iter()
        .map(|m| {
            offset = base_types::align_to(offset, m.type_kind.align());
            let _offset = offset;
            offset += m.type_kind.size();
            let mem = Member::new(Rc::new(m.type_kind), _offset, m.ident);
            Rc::new(mem)
        })
        .collect();

    _struct.borrow_mut().members = Rc::new(members);
    _struct.borrow_mut().is_incomplete = false;
    Ok(_struct)
}

// enum-specifier          = enum ident? "{" enum-list? "}"
//                         | enum ident
// enum-list               = enum-elem ("," enum-elem)* ","?
// enum-elem               = ident ("=" const-expr)?
pub fn enum_specifier(iter: &mut TokenIter, ctx: &mut Context) -> Result<Rc<Enum>, Error> {
    expect_keyword(iter, KeyWord::Enum)?;
    let tag = consume_ident(iter);
    let prev_pos = iter.prev_pos; // for error
    if !consume_block(iter, Block::LParen) {
        match &tag {
            Some(x) => {
                if let Some(tag) = ctx.s.find_upper_tag(Rc::new(x.clone())) {
                    if let TagTypeKind::Enum(_enum) = tag.as_ref() {
                        return Ok(_enum.clone());
                    }
                } else {
                    iter.prev();
                    return Err(Error::undefined_tag(
                        iter.filepath,
                        iter.s,
                        iter.pos,
                        x.clone(),
                        None,
                    ));
                }
            }
            None => {
                dbg!("incomplete enum dec");
                return Err(Error::todo(iter.filepath, iter.s, iter.pos));
            }
        }
    }

    if let Some(t) = &tag {
        if let Some(_) = ctx.s.find_cur_tag(Rc::new(t.clone())) {
            iter.pos = prev_pos;
            return Err(Error::re_declare(
                iter.filepath,
                iter.s,
                t.clone(),
                iter.pos,
                None,
            ));
        }
    }

    let mut enum_list = Vec::new();
    let mut count = 0;
    while !consume_block(iter, Block::RParen) {
        let ident = expect_ident(iter)?;
        if consume(iter, Operator::Assign) {
            count = const_expr(iter, ctx)?;
        }
        let l = ctx
            .l
            .lvar
            .as_ref()
            .map(|lvar| lvar.borrow().offset)
            .unwrap_or(0);
        ctx.push_front(
            Declaration::new_const(TypeKind::Int, ident.clone(), count),
            l,
        );
        enum_list.push(Rc::new((ident, count)));
        count += 1;
        consume_comma(iter);
    }

    let enum_list = Rc::new(enum_list);
    let mut _enum = if let Some(tag) = tag {
        let tag = Rc::new(tag);
        let _enum = Rc::new(Enum::new(tag.clone(), enum_list));
        let result = ctx
            .s
            .insert_t(tag.clone(), TagTypeKind::Enum(_enum.clone()));
        if let Some(_) = result {
            unreachable!()
        }
        _enum
    } else {
        Rc::new(Enum::new(
            Rc::new(Ident::new(".struct.anonymous")),
            enum_list,
        ))
    };
    Ok(_enum)
}

// declaration     = type-specifier declarator type-suffix
//                 | type-specifier
pub(crate) fn declaration(iter: &mut TokenIter, ctx: &mut Context) -> Result<Declaration, Error> {
    let (type_kind, (is_typedef, is_static, is_extern)) = type_specifier(iter, ctx)?;
    let type_kind = Rc::new(RefCell::new(type_kind));
    let mut ident = Ident::new_anonymous();
    let mut dec = if let Some(dec) = consume_declarator(iter, ctx, type_kind.clone(), &mut ident) {
        let type_suffix = type_suffix(iter, ctx, dec)?;
        let type_suffix = type_suffix.borrow().clone();
        Declaration::new(type_suffix, ident) // todo
    } else {
        let type_kind = type_kind.borrow().clone();
        Declaration::new(type_kind, ident)
    };

    dec.is_typedef = is_typedef;
    dec.is_static = is_static;
    dec.is_extern = is_extern;

    match (
        ctx.s.find_cur_lvar(dec.ident.clone()),
        ctx.s.find_cur_gvar(dec.ident.clone()),
    ) {
        (None, None) => (),
        _ => {
            iter.prev();
            return Err(Error::re_declare(
                iter.filepath,
                iter.s,
                dec.ident.clone(),
                iter.pos,
                None,
            ));
        }
    }

    if is_typedef {
        let result = ctx.s.insert_t(
            Rc::new(dec.ident.clone()),
            TagTypeKind::Typedef(Rc::new(dec.clone())),
        );
        if let Some(_) = result {
            return Err(Error::re_declare(
                iter.filepath,
                iter.s,
                dec.ident.clone(),
                iter.pos,
                None,
            ));
        }
    }

    if is_static {
        //     let ident = Ident::new(ctx.make_label());
        //     ctx.g.gvar_mp.insert(
        //         ident.name.clone(),
        //         Rc::new(check_g_var(
        //             iter,
        //             &ctx.g.gvar_mp,
        //             dec.type_kind.clone(),
        //             ident,
        //             vec![],
        //         )?),
        //     );
        //     // ctx.s.push_front(dec.clone(),)
    }
    Ok(dec)
}

// function    =  type-specifier declarator "(" params? ")" "{" stmt* "}"
pub fn function(
    iter: &mut TokenIter,
    func_prototype: Rc<FuncPrototype>,
    ctx: &mut Context,
) -> Result<Function, Error> {
    expect_block(iter, Block::LParen)?;

    ctx.l = LocalContext::new();
    for fn_param in func_prototype.params.clone() {
        let l = ctx
            .l
            .lvar
            .as_ref()
            .map(|lvar| lvar.borrow().offset)
            .unwrap_or(0);
        ctx.push_front(fn_param, l)
    }

    let mut stmt_vec = Vec::new();
    loop {
        if consume_block(iter, Block::RParen) {
            let is_static = func_prototype.is_static;
            return Ok(Function::new(
                func_prototype,
                ctx.l.lvar.clone(),
                ctx.l.lvar_count.clone(),
                stmt_vec,
                is_static,
            ));
        }
        stmt_vec.push(stmt(iter, ctx)?);
    }
}

// params      = declaration ("," declaration)* | "void"
pub fn params(iter: &mut TokenIter, ctx: &mut Context) -> Result<Vec<Declaration>, Error> {
    let pos = iter.pos;
    if consume_type_kind(iter) == Some(TypeKind::Void) && consume(iter, Operator::RParen) {
        return Ok(Vec::new());
    }
    iter.pos = pos;
    let mut params = vec![read_param(iter, ctx)?];
    while !consume(iter, Operator::RParen) {
        expect_comma(iter)?;
        params.push(read_param(iter, ctx)?);
    }
    Ok(params)
}

pub fn read_param(iter: &mut TokenIter, ctx: &mut Context) -> Result<Declaration, Error> {
    let mut dec = declaration(iter, ctx)?;
    if let TypeKind::Array(_, base, _) = &dec.type_kind {
        dec.type_kind = TypeKind::ptr_to(base.clone());
    }
    Ok(dec)
}

pub fn lvar_init_zero(
    iter: &mut TokenIter,
    ctx: &mut Context,
    lvar: Rc<RefCell<Lvar>>,
    type_kind: Rc<RefCell<TypeKind>>,
    desg: &mut Option<Box<Designator>>,
) -> Result<Node, Error> {
    let mut init = Vec::new();
    match &*type_kind.borrow() {
        TypeKind::Array(size, base, _) => {
            let mut i = 0;
            while i < *size {
                let mut desg2 = Some(Box::new(Designator::new(i, desg.clone(), None)));
                i += 1;
                let node = lvar_init_zero(iter, ctx, lvar.clone(), base.clone(), &mut desg2)?;
                init.push(node);
                i += 1;
            }
            return Ok(Node::new_init(
                NodeKind::Declaration(lvar.borrow().dec.clone()),
                init,
            ));
        }
        _ => (),
    }
    Ok(Node::new_init(
        NodeKind::Declaration(lvar.borrow().dec.clone()),
        vec![new_desg_node(Var::L(lvar.clone()), desg, Node::new_num(0))?],
    ))
}

// lvar-initializer = assign
//                  | "{" lvar-initializer ("," lvar-initializer)* ","? "}"
fn lvar_initializer(
    iter: &mut TokenIter,
    ctx: &mut Context,
    lvar: Rc<RefCell<Lvar>>,
    type_kind: Rc<RefCell<TypeKind>>,
    desg: &mut Option<Box<Designator>>,
) -> Result<(Node, Rc<RefCell<TypeKind>>), Error> {
    let var = Var::L(lvar.clone());
    if let TypeKind::Array(size, base, is_sized) = &mut *type_kind.borrow_mut() {
        if &*base.borrow() == &TypeKind::Char {
            if let Some(string) = consume_string(iter) {
                if !*is_sized {
                    *is_sized = true;
                    *size = string.len() as u64;
                }
                let len = min(string.len() as u64, *size);
                let mut i = 0;
                let string = string.as_bytes();
                let mut init = Vec::new();
                while i < len {
                    let mut desg2 = Some(Box::new(Designator::new(i, desg.clone(), None)));
                    let rhs = Node::new_num(string[i as usize] as i64); //
                    init.push(new_desg_node(var.clone(), &mut desg2, rhs)?);
                    i += 1;
                }
                for i in i..*size {
                    let mut desg2 = Some(Box::new(Designator::new(i, desg.clone(), None)));
                    let node = lvar_init_zero(iter, ctx, lvar.clone(), base.clone(), &mut desg2)?;
                    init.push(node);
                }
                return Ok((
                    Node::new_init(NodeKind::Declaration(lvar.borrow().dec.clone()), init),
                    type_kind.clone(),
                ));
            }
        }
    }

    match &mut *type_kind.borrow_mut() {
        TypeKind::Array(size, base, is_sized) => {
            let mut init = Vec::new();
            let mut i = 0;
            let open = consume_block(iter, Block::LParen);
            let limit = if !*is_sized { u64::MAX } else { *size };
            if !peek_end(iter) {
                while {
                    let mut desg2 = Some(Box::new(Designator::new(i, desg.clone(), None)));
                    i += 1;
                    let node = lvar_initializer(iter, ctx, lvar.clone(), base.clone(), &mut desg2)?;
                    init.push(node.0);
                    i < limit && !peek_end(iter) && consume_comma(iter)
                } {}
            }

            if open && !consume_end(iter) {
                skip_excess_elements(iter, ctx)?;
            }

            while i < *size {
                let mut desg2 = Some(Box::new(Designator::new(i, desg.clone(), None)));
                i += 1;
                let node = lvar_init_zero(iter, ctx, lvar.clone(), base.clone(), &mut desg2)?;
                init.push(node);
            }
            if !*is_sized {
                *is_sized = true;
                *size = init.len() as u64;
            }
            return Ok((
                Node::new_init(NodeKind::Declaration(lvar.borrow().dec.clone()), init),
                type_kind.clone(),
            ));
        }
        TypeKind::Struct(_struct) => {
            let members = _struct.borrow().members.clone();
            let mut init = Vec::new();
            let mut i = 0;
            let open = consume_block(iter, Block::LParen);
            if !peek_end(iter) {
                while {
                    let mut desg2 = Some(Box::new(Designator::new(
                        0,
                        desg.clone(),
                        Some(members[i].clone()),
                    )));
                    let node = lvar_initializer(
                        iter,
                        ctx,
                        lvar.clone(),
                        // todo struct {} だとパニックになる。
                        // あとrefcellとかもう少しいいやり方ありそう
                        Rc::new(RefCell::new(members[i].type_kind.as_ref().clone())),
                        &mut desg2,
                    )?;
                    i += 1;
                    init.push(node.0);
                    i < members.len() && !peek_end(iter) && consume_comma(iter)
                } {}
            }
            if open && !consume_end(iter) {
                skip_excess_elements(iter, ctx)?;
            }
            while members.len() > i {
                let mut desg2 = Some(Box::new(Designator::new(
                    0,
                    desg.clone(),
                    Some(members[i].clone()),
                )));

                let node = lvar_init_zero(
                    iter,
                    ctx,
                    lvar.clone(),
                    // todo struct {} だとパニックになる。
                    // あとrefcellとかもう少しいいやり方ありそう
                    Rc::new(RefCell::new(members[i].type_kind.as_ref().clone())),
                    &mut desg2,
                )?;
                init.push(node);
                i += 1;
            }

            return Ok((
                Node::new_init(NodeKind::Declaration(lvar.borrow().dec.clone()), init),
                type_kind.clone(),
            ));
        }
        _ => {}
    }
    let open = consume_block(iter, Block::LParen);
    let init = new_desg_node(var, desg, assign(iter, ctx)?)?;
    if open {
        expect_block(iter, Block::RParen)?;
    }
    Ok((
        Node::new_init(NodeKind::Declaration(lvar.borrow().dec.clone()), vec![init]),
        type_kind,
    ))
}

// stmt        = expr ";"
//             | "return" expr? ";"
//             | "if" "(" expr ")" stmt
//             | "while" "(" expr ")" stmt
//             | "do" stmt "while" "(" expr ")" ";"
//             | "for" "(" stmt? ";" expr? ";" expr? ")" stmt
//             | "{" stmt* "}"
//             | declaration ("=" initialize)? ";"
//             | "break" ";"
//             | "continue" ";"
//             | "goto" ident ";"
//             | ident ":" stmt
//             | "switch" "("expr")" stmt
//             | "case" const-expr ":" stmt
//             | "default" ":" stmt
pub fn stmt(iter: &mut TokenIter, ctx: &mut Context) -> Result<Node, Error> {
    if let Some(x) = iter.peek() {
        match x.kind {
            TokenKind::KeyWord(key) => {
                match key {
                    KeyWord::Return => {
                        iter.next();
                        if consume_semi(iter) {
                            return Ok(Node::new_leaf(NodeKind::Return));
                        }
                        let node = Node::new_unary(NodeKind::Return, expr(iter, ctx)?);
                        expect_semi(iter)?;
                        return Ok(node);
                    }
                    KeyWord::If => {
                        iter.next();
                        expect(iter, Operator::LParen)?;
                        let mut node = Node::new_cond(NodeKind::If, expr(iter, ctx)?);
                        expect(iter, Operator::RParen)?;
                        node.then = Some(Box::new(stmt(iter, ctx)?));
                        if let Some(x) = iter.peek() {
                            if x.kind == TokenKind::KeyWord(KeyWord::Else) {
                                iter.next();
                                node.els = Some(Box::new(stmt(iter, ctx)?));
                            }
                        }
                        return Ok(node);
                    }
                    KeyWord::While => {
                        iter.next();
                        expect(iter, Operator::LParen)?;
                        let mut node = Node::new_cond(NodeKind::While, expr(iter, ctx)?);
                        expect(iter, Operator::RParen)?;
                        node.then = Some(Box::new(stmt(iter, ctx)?));
                        return Ok(node);
                    }
                    KeyWord::Do => {
                        iter.next();
                        let mut node = Node::new_leaf(NodeKind::Do);
                        node.then = Some(Box::new(stmt(iter, ctx)?));
                        expect_keyword(iter, KeyWord::While)?;
                        expect(iter, Operator::LParen)?;
                        node.cond = Some(Box::new(expr(iter, ctx)?));
                        expect(iter, Operator::RParen)?;
                        expect_semi(iter)?;
                        return Ok(node);
                    }
                    KeyWord::For => {
                        iter.next();
                        expect(iter, Operator::LParen)?;
                        let mut node = Node::new_none(NodeKind::For);
                        let sc = ctx.s.enter();

                        if !consume_semi(iter) {
                            node.init = Some(vec![stmt(iter, ctx)?]);
                        }
                        if !consume_semi(iter) {
                            node.cond = Some(Box::new(expr(iter, ctx)?));
                            expect_semi(iter)?;
                        }
                        if !consume(iter, Operator::RParen) {
                            node.inc = Some(Box::new(read_expr_stmt(iter, ctx)?));
                            expect(iter, Operator::RParen)?;
                        }
                        node.then = Some(Box::new(stmt(iter, ctx)?));
                        ctx.s.leave(sc);
                        return Ok(node);
                    }
                    KeyWord::Break => {
                        iter.next();
                        expect_semi(iter)?;
                        return Ok(Node::new_leaf(NodeKind::Break));
                    }
                    KeyWord::Continue => {
                        iter.next();
                        expect_semi(iter)?;
                        return Ok(Node::new_leaf(NodeKind::Continue));
                    }
                    KeyWord::Goto => {
                        iter.next();
                        let ident = expect_ident(iter)?;
                        expect_semi(iter)?;
                        return Ok(Node::new_leaf(NodeKind::Goto(ident)));
                    }
                    KeyWord::Switch => {
                        iter.next();
                        expect(iter, Operator::LParen)?;
                        // let mut node = Node::new_leaf(NodeKind::Switch);
                        let cond = Some(Box::new(expr(iter, ctx)?));
                        expect(iter, Operator::RParen)?;

                        let sw = std::mem::replace(&mut ctx.cur_switch, Some(vec![]));
                        let then = Some(Box::new(stmt(iter, ctx)?));

                        let cases = std::mem::replace(&mut ctx.cur_switch, sw)
                            .ok_or(Error::todo(iter.filepath, iter.s, iter.pos))?;
                        let mut node = Node::new_leaf(NodeKind::Switch(cases));
                        node.cond = cond;
                        node.then = then;
                        return Ok(node);
                    }
                    KeyWord::Case => {
                        iter.next();
                        match ctx.cur_switch.take() {
                            Some(mut cur_case) => {
                                let val = const_expr(iter, ctx)?;
                                expect_colon(iter)?;
                                let node = Node::new_unary(NodeKind::Case(val), stmt(iter, ctx)?);
                                cur_case.push(node.clone());
                                ctx.cur_switch = Some(cur_case);
                                return Ok(node);
                            }
                            None => return Err(Error::stray_case(iter.filepath, iter.s, iter.pos)),
                        }
                    }
                    KeyWord::Default => {
                        iter.next();
                        match ctx.cur_switch.take() {
                            Some(mut cur_case) => {
                                expect_colon(iter)?;
                                let node = Node::new_unary(NodeKind::DefaultCase, stmt(iter, ctx)?);
                                cur_case.push(node.clone());
                                ctx.cur_switch = Some(cur_case);
                                return Ok(node);
                            }
                            None => return Err(Error::stray_case(iter.filepath, iter.s, iter.pos)),
                        }
                    }

                    _ => (),
                }
            }
            TokenKind::Block(block) => match block {
                Block::LParen => {
                    iter.next();
                    let mut stmt_vec = Vec::new();
                    let sc = ctx.s.enter();

                    while !consume_block(iter, Block::RParen) {
                        stmt_vec.push(stmt(iter, ctx)?);
                    }
                    ctx.s.leave(sc);

                    return Ok(Node::new_none(NodeKind::Block(stmt_vec)));
                }
                _ => {
                    return Err(Error::unexpected_token(
                        iter.filepath,
                        iter.s,
                        x.clone(),
                        TokenKind::Block(Block::LParen),
                    ));
                }
            },
            _ => (),
        }
    }

    {
        let cur = iter.clone();
        if let Some(ident) = consume_ident(iter) {
            if consume_colon(iter) {
                return Ok(Node::new_unary(NodeKind::Label(ident), stmt(iter, ctx)?));
            } else {
                *iter = cur;
            }
        }
    }

    if is_typename(iter, ctx) {
        let mut dec = declaration(iter, ctx)?;

        // todo re declaration err handling
        if consume_semi(iter) {
            if dec.ident.is_anonymous() || dec.is_typedef {
                return Ok(Node::new_leaf(NodeKind::Null));
            }
            if dec.is_static {
                let size = dec.type_kind.size();
                let mut label = ctx.make_label();
                std::mem::swap(&mut dec.ident.name, &mut label);
                let gvar = Rc::new(Gvar::new(dec.clone(), size, vec![]));
                ctx.g.gvar_mp.insert(dec.ident.name.clone(), gvar.clone());
                ctx.s.insert_v(Ident::new(label), Rc::new(Var::G(gvar)));
            } else if dec.is_extern {
                let size = dec.type_kind.size();
                let gvar = Rc::new(Gvar::new(dec.clone(), size, vec![]));
                ctx.s.insert_v(dec.ident.clone(), Rc::new(Var::G(gvar)));
            } else {
                ctx.push_front(
                    dec.clone(),
                    ctx.l
                        .lvar
                        .as_ref()
                        .map(|lvar| lvar.borrow().offset)
                        .unwrap_or(0),
                );
            }
            return Ok(Node::new_leaf(NodeKind::Declaration(dec)));
        }
        expect(iter, Operator::Assign)?;

        ctx.push_front(
            dec.clone(),
            ctx.l
                .lvar
                .as_ref()
                .map(|lvar| lvar.borrow().offset)
                .unwrap_or(0),
        );
        let lvar = ctx.s.find_cur_lvar(dec.ident.clone()).unwrap();
        let (node, type_kind) = lvar_initializer(
            iter,
            ctx,
            lvar.clone(),
            Rc::new(RefCell::new(lvar.borrow().dec.type_kind.clone())),
            &mut None,
        )?;
        lvar.borrow_mut().dec.type_kind = type_kind.borrow().clone();
        lvar.borrow_mut().offset += type_kind.borrow().size();
        expect_semi(iter)?;
        return Ok(node);
    }

    let node = read_expr_stmt(iter, ctx)?;
    expect_semi(iter)?;
    Ok(node)
}

pub fn read_expr_stmt(iter: &mut TokenIter, ctx: &mut Context) -> Result<Node, Error> {
    Ok(Node::new_unary(NodeKind::ExprStmt, expr(iter, ctx)?))
}

// expr        = assign ("," assign)*
pub fn expr(iter: &mut TokenIter, ctx: &mut Context) -> Result<Node, Error> {
    let mut node = assign(iter, ctx)?;
    while consume_comma(iter) {
        node = Node::new_expr_stmt(node);
        node = Node::new(NodeKind::Comma, node, assign(iter, ctx)?);
    }
    Ok(node)
}

fn eval(node: &mut Node) -> Result<i64, Error> {
    eval2(node, &mut None)
}

fn eval2(node: &mut Node, var: &mut Option<Gvar>) -> Result<i64, Error> {
    use NodeKind::*;
    match &node.kind {
        Add => {
            let lhs = eval2(node.lhs.as_mut().unwrap(), var)?;

            return Ok(lhs + eval2(&mut node.rhs.as_mut().unwrap(), var)?);
        }
        Sub => {
            let lhs = eval2(node.lhs.as_mut().unwrap(), var)?;
            return Ok(lhs - eval(&mut node.rhs.as_mut().unwrap())?);
        }
        Mul => {
            return Ok(
                eval(&mut node.lhs.as_mut().unwrap())? * eval(&mut node.rhs.as_mut().unwrap())?
            )
        }
        Div => {
            return Ok(
                eval(&mut node.lhs.as_mut().unwrap())? / eval(&mut node.rhs.as_mut().unwrap())?
            )
        }
        BitAnd => {
            return Ok(
                eval(&mut node.lhs.as_mut().unwrap())? & eval(&mut node.rhs.as_mut().unwrap())?
            )
        }
        BitOr => {
            return Ok(
                eval(&mut node.lhs.as_mut().unwrap())? | eval(&mut node.rhs.as_mut().unwrap())?
            )
        }
        BitXor => {
            return Ok(
                eval(&mut node.lhs.as_mut().unwrap())? | eval(&mut node.rhs.as_mut().unwrap())?
            )
        }
        LShift => {
            return Ok(
                eval(&mut node.lhs.as_mut().unwrap())? << eval(&mut node.rhs.as_mut().unwrap())?
            )
        }
        RShift => {
            return Ok(
                eval(&mut node.lhs.as_mut().unwrap())? >> eval(&mut node.rhs.as_mut().unwrap())?
            )
        }
        Equal => {
            if eval(&mut node.lhs.as_mut().unwrap())? == eval(&mut node.rhs.as_mut().unwrap())? {
                return Ok(1);
            } else {
                return Ok(0);
            }
        }
        Neq => {
            if eval(&mut node.lhs.as_mut().unwrap())? != eval(&mut node.rhs.as_mut().unwrap())? {
                return Ok(1);
            } else {
                return Ok(0);
            }
        }
        Lesser => {
            if eval(&mut node.lhs.as_mut().unwrap())? < eval(&mut node.rhs.as_mut().unwrap())? {
                return Ok(1);
            } else {
                return Ok(0);
            }
        }
        Leq => {
            if eval(&mut node.lhs.as_mut().unwrap())? <= eval(&mut node.rhs.as_mut().unwrap())? {
                return Ok(1);
            } else {
                return Ok(0);
            }
        }
        Ternary => {
            if eval(&mut node.cond.as_mut().unwrap())? != 0 {
                return eval(&mut node.then.as_mut().unwrap());
            } else {
                return eval(&mut node.els.as_mut().unwrap());
            }
        }
        Comma => return eval(&mut node.rhs.as_mut().unwrap()),
        Not => {
            if eval(&mut node.lhs.as_mut().unwrap())? == 0 {
                return Ok(1);
            } else {
                return Ok(0);
            }
        }
        BitNot => return Ok(!eval(&mut node.lhs.as_mut().unwrap())?),
        LogAnd => {
            if (eval(&mut node.lhs.as_mut().unwrap())? != 0)
                && (eval(&mut node.rhs.as_mut().unwrap())? != 0)
            {
                return Ok(1);
            } else {
                return Ok(0);
            }
        }
        LogOr => {
            if (eval(&mut node.lhs.as_mut().unwrap())? != 0)
                || (eval(&mut node.rhs.as_mut().unwrap())? != 0)
            {
                return Ok(1);
            } else {
                return Ok(0);
            }
        }
        Num(num) => return Ok(*num as i64),
        Addr => match var {
            Some(_) => todo!(),
            None => {
                if let NodeKind::Gvar(gvar) = &node.lhs.as_ref().unwrap().kind {
                    *var = Some(gvar.as_ref().clone());
                    return Ok(0);
                }
                todo!()
            }
        },
        Gvar(gvar) => match var {
            Some(_) => todo!(),
            None => {
                if let crate::base_types::TypeKind::Array(_, _, _) = gvar.dec.type_kind {
                    *var = Some(gvar.as_ref().clone());
                    return Ok(0);
                }
                todo!()
            }
        },
        _ => todo!(),
    }
}

pub fn const_expr(iter: &mut TokenIter, ctx: &mut Context) -> Result<i64, Error> {
    eval(&mut conditional(iter, ctx)?)
}

// assign                  = conditional (assign-op assign)?
// assign-op               = "=" | "+=" | "-=" | "*=" | "/=" | "<<=" | ">>="
pub fn assign(iter: &mut TokenIter, ctx: &mut Context) -> Result<Node, Error> {
    let mut node = conditional(iter, ctx)?;
    if consume(iter, Operator::Assign) {
        let rhs = assign(iter, ctx)?;
        // 左右の型が違っても受け入れる

        // let lhs_type = node
        //     .get_type()
        //     .unwrap_or(TypeKind::_Invalid("invalid lhs".to_string()));
        // let rhs_type = rhs
        //     .get_type()
        //     .unwrap_or(TypeKind::_Invalid("invalid rhs".to_string()));

        // 配列とポインタの比較が中途半端
        // if lhs_type != rhs_type && !TypeKind::partial_comp(&lhs_type, &rhs_type) {
        //     return Err(Error::invalid_assignment(
        //         iter.filepath,
        //         iter.s,
        //         iter.pos,
        //         lhs_type,
        //         rhs_type,
        //     ));
        // }
        node = Node::new(NodeKind::Assign, node, rhs);
    } else if consume(iter, Operator::APlus) {
        let rhs = assign(iter, ctx)?;
        node = Node::new(NodeKind::AAdd, node, rhs);
    } else if consume(iter, Operator::AMinus) {
        let rhs = assign(iter, ctx)?;
        node = Node::new(NodeKind::ASub, node, rhs);
    } else if consume(iter, Operator::AMul) {
        let rhs = assign(iter, ctx)?;
        node = Node::new(NodeKind::AMul, node, rhs);
    } else if consume(iter, Operator::ADiv) {
        let rhs = assign(iter, ctx)?;
        node = Node::new(NodeKind::ADiv, node, rhs);
    } else if consume(iter, Operator::ALShift) {
        let rhs = assign(iter, ctx)?;
        node = Node::new(NodeKind::ALShift, node, rhs);
    } else if consume(iter, Operator::ARShift) {
        let rhs = assign(iter, ctx)?;
        node = Node::new(NodeKind::ARShift, node, rhs);
    }
    return Ok(node);
}

// conditional             = logor ("?" expr ":" conditional)?
pub fn conditional(iter: &mut TokenIter, ctx: &mut Context) -> Result<Node, Error> {
    let node = log_or(iter, ctx)?;
    if !consume_question(iter) {
        return Ok(node);
    }
    let mut ternary = Node::new_leaf(NodeKind::Ternary);
    ternary.cond = Some(Box::new(node));
    ternary.then = Some(Box::new(expr(iter, ctx)?));
    expect_colon(iter)?;
    ternary.els = Some(Box::new(conditional(iter, ctx)?));
    return Ok(ternary);
}

// logor                   = logand ("||" logand)*
pub fn log_or(iter: &mut TokenIter, ctx: &mut Context) -> Result<Node, Error> {
    let mut node = log_and(iter, ctx)?;
    while consume(iter, Operator::LogOr) {
        node = Node::new(NodeKind::LogOr, node, log_and(iter, ctx)?);
    }
    return Ok(node);
}

// logand                  = bitor ("&&" bitor)*
pub fn log_and(iter: &mut TokenIter, ctx: &mut Context) -> Result<Node, Error> {
    let mut node = bit_or(iter, ctx)?;
    while consume(iter, Operator::LogAnd) {
        node = Node::new(NodeKind::LogAnd, node, bit_or(iter, ctx)?);
    }
    return Ok(node);
}

// bitor                   = bitxor ("|" bitxor)*
pub fn bit_or(iter: &mut TokenIter, ctx: &mut Context) -> Result<Node, Error> {
    let mut node = bit_xor(iter, ctx)?;
    while consume(iter, Operator::BitOr) {
        node = Node::new(NodeKind::BitOr, node, bit_xor(iter, ctx)?);
    }
    return Ok(node);
}

// bitxor                  = bitand ("^" bitand)*
pub fn bit_xor(iter: &mut TokenIter, ctx: &mut Context) -> Result<Node, Error> {
    let mut node = bit_and(iter, ctx)?;
    while consume(iter, Operator::BitXor) {
        node = Node::new(NodeKind::BitXor, node, bit_and(iter, ctx)?);
    }
    return Ok(node);
}

// bitand                  = equality ("&" equality)*
pub fn bit_and(iter: &mut TokenIter, ctx: &mut Context) -> Result<Node, Error> {
    let mut node = equality(iter, ctx)?;
    while consume(iter, Operator::Ampersand) {
        node = Node::new(NodeKind::BitAnd, node, equality(iter, ctx)?);
    }
    return Ok(node);
}

// equality    = relational ("==" relational | "!=" relational)*
pub fn equality(iter: &mut TokenIter, ctx: &mut Context) -> Result<Node, Error> {
    let mut node = relational(iter, ctx)?;
    loop {
        if consume(iter, Operator::Equal) {
            node = Node::new(NodeKind::Equal, node, relational(iter, ctx)?);
        } else if consume(iter, Operator::Neq) {
            node = Node::new(NodeKind::Neq, node, relational(iter, ctx)?);
        } else {
            return Ok(node);
        }
    }
}

// relational              = shift ("<" shift | "<=" | ">" shift | ">=" shift)*
pub fn relational(iter: &mut TokenIter, ctx: &mut Context) -> Result<Node, Error> {
    let mut node = shift(iter, ctx)?;
    loop {
        if consume(iter, Operator::Lesser) {
            node = Node::new(NodeKind::Lesser, node, shift(iter, ctx)?);
        } else if consume(iter, Operator::Leq) {
            node = Node::new(NodeKind::Leq, node, shift(iter, ctx)?);
        } else if consume(iter, Operator::Greater) {
            // 左右を入れ替えて読み変える
            node = Node::new(NodeKind::Lesser, shift(iter, ctx)?, node);
        } else if consume(iter, Operator::Geq) {
            // 左右を入れ替えて読み変える
            node = Node::new(NodeKind::Leq, shift(iter, ctx)?, node);
        } else {
            return Ok(node);
        }
    }
}

// shift                   = add ("<<" add | ">>" add)*
pub fn shift(iter: &mut TokenIter, ctx: &mut Context) -> Result<Node, Error> {
    let mut node = add(iter, ctx)?;
    loop {
        if consume(iter, Operator::LShift) {
            node = Node::new(NodeKind::LShift, node, add(iter, ctx)?);
        } else if consume(iter, Operator::RShift) {
            node = Node::new(NodeKind::RShift, node, add(iter, ctx)?);
        } else {
            return Ok(node);
        }
    }
}

// add         = mul ("+" mul | "-" mul)*
pub fn add(iter: &mut TokenIter, ctx: &mut Context) -> Result<Node, Error> {
    let mut node = mul(iter, ctx)?;
    loop {
        if consume(iter, Operator::Plus) {
            node = Node::new(NodeKind::Add, node, mul(iter, ctx)?)
        } else if consume(iter, Operator::Minus) {
            node = Node::new(NodeKind::Sub, node, mul(iter, ctx)?)
        } else {
            return Ok(node);
        }
    }
}

// mul         = cast ("*" cast | "/" cast)*
pub fn mul(iter: &mut TokenIter, ctx: &mut Context) -> Result<Node, Error> {
    let mut node = cast(iter, ctx)?;
    loop {
        if consume(iter, Operator::Mul) {
            node = Node::new(NodeKind::Mul, node, cast(iter, ctx)?)
        } else if consume(iter, Operator::Div) {
            node = Node::new(NodeKind::Div, node, cast(iter, ctx)?)
        } else {
            return Ok(node);
        }
    }
}

// cast                    = "(" type-name ")" cast | unary
pub fn cast(iter: &mut TokenIter, ctx: &mut Context) -> Result<Node, Error> {
    if consume(iter, Operator::LParen) {
        if let Some(x) = iter.peek() {
            if let TokenKind::TypeKind(_) = x.kind {
                let ty = type_name(iter, ctx)?;
                expect(iter, Operator::RParen)?;
                return Ok(Node::new_unary(
                    NodeKind::Cast(ty.replace(TypeKind::Int)),
                    cast(iter, ctx)?,
                ));
            }
            // `(`をconsumeした分を戻す
            iter.pos.tk -= 1;
            iter.pos.bytes -= 1;
        }
    }

    unary(iter, ctx)
}

// unary       = ("+" | "-" | "*" | "&" | "!")? cast
//             | ("++" | "--") unary
//             | postfix
pub fn unary(iter: &mut TokenIter, ctx: &mut Context) -> Result<Node, Error> {
    if consume(iter, Operator::Plus) {
        return cast(iter, ctx);
    } else if consume(iter, Operator::Minus) {
        return Ok(Node::new(NodeKind::Sub, Node::new_num(0), cast(iter, ctx)?));
    } else if consume(iter, Operator::Mul) {
        return Ok(Node::new_unary(NodeKind::Deref, cast(iter, ctx)?));
    } else if consume(iter, Operator::Ampersand) {
        return Ok(Node::new_unary(NodeKind::Addr, cast(iter, ctx)?));
    } else if consume(iter, Operator::PlusPlus) {
        return Ok(Node::new_unary(NodeKind::PreInc, unary(iter, ctx)?));
    } else if consume(iter, Operator::MinusMinus) {
        return Ok(Node::new_unary(NodeKind::PreDec, unary(iter, ctx)?));
    } else if consume(iter, Operator::Not) {
        return Ok(Node::new_unary(NodeKind::Not, unary(iter, ctx)?));
    } else if consume(iter, Operator::BitNot) {
        return Ok(Node::new_unary(NodeKind::BitNot, unary(iter, ctx)?));
    }

    return postfix(iter, ctx);
}

// postfix     = primary ("[" expr "]" | "." ident | "->" ident | "++" | "--")*
pub fn postfix(iter: &mut TokenIter, ctx: &mut Context) -> Result<Node, Error> {
    let mut pri = primary(iter, ctx)?;
    loop {
        if consume(iter, Operator::LArr) {
            let idx = expr(iter, ctx)?;
            expect(iter, Operator::RArr)?;
            pri = Node::new_unary(NodeKind::Deref, Node::new(NodeKind::Add, pri, idx));
            continue;
        }

        if consume(iter, Operator::PlusPlus) {
            pri = Node::new_unary(NodeKind::PostInc, pri);
            continue;
        }
        if consume(iter, Operator::MinusMinus) {
            pri = Node::new_unary(NodeKind::PostDec, pri);
            continue;
        }

        if let Some(x) = iter.peek() {
            match x.kind {
                TokenKind::Period => {
                    iter.next();
                }
                TokenKind::Reserved(Operator::Arrow) => {
                    iter.next();
                    pri = Node::new_unary(NodeKind::Deref, pri);
                }
                _ => (),
            }
            match x.kind {
                TokenKind::Period | TokenKind::Reserved(Operator::Arrow) => {
                    let member_name = expect_ident(iter)?;
                    match &pri.get_type() {
                        Ok(type_kind) => match type_kind {
                            TypeKind::Struct(_struct) => {
                                let member = _struct.borrow().find_field(&member_name).ok_or(
                                    Error::undefined_member(
                                        iter.filepath,
                                        iter.s,
                                        iter.pos,
                                        member_name.clone(),
                                        None,
                                    ),
                                )?;
                                pri = Node::new_unary(NodeKind::Member(member_name, member), pri);
                                continue;
                            }
                            _ => {
                                dbg!("not a struct");
                                return Err(Error::todo(iter.filepath, iter.s, iter.pos));
                            }
                        },
                        Err(_) => {
                            dbg!();
                            return Err(Error::todo(iter.filepath, iter.s, iter.pos));
                        }
                    }
                }
                _ => (),
            }
        }

        return Ok(pri);
    }
}

// stmt-expr       = "(" "{" stmt stmt* "}" ")"
pub fn stmt_expr(iter: &mut TokenIter, ctx: &mut Context) -> Result<Node, Error> {
    // expect(iter, Operator::LParen)?;
    // expect_block(iter, Block::LParen)?;
    let sc = ctx.s.enter();
    let mut nodes = vec![stmt(iter, ctx)?];
    while !consume_block(iter, Block::RParen) {
        nodes.push(stmt(iter, ctx)?);
    }
    expect(iter, Operator::RParen)?;

    if nodes.last().unwrap().kind != NodeKind::ExprStmt {
        return Err(Error::invalid_stmt_expr(iter.filepath, iter.s, iter.pos));
    }
    *(nodes.last_mut().unwrap()) = std::mem::replace(
        nodes.last_mut().unwrap().lhs.as_mut().unwrap(),
        Node::new_num(0),
    );
    ctx.s.leave(sc);
    Ok(Node::new_leaf(NodeKind::StmtExpr(nodes)))
}

// primary     = num
//             | ident func-args?
//             | "(" expr ")"
//             | str
//             | char
//             | "(" "{" stmt-expr-tail
//             | sizeof unary
//             | sizeof "(" type-name ")"
pub fn primary(iter: &mut TokenIter, ctx: &mut Context) -> Result<Node, Error> {
    // "(" expr ")"
    if consume(iter, Operator::LParen) {
        if consume_block(iter, Block::LParen) {
            return Ok(stmt_expr(iter, ctx)?);
        }
        let node = expr(iter, ctx)?;
        expect(iter, Operator::RParen)?;
        return Ok(node);
    }

    // ident func-args?
    if let Some(ident) = consume_ident(iter) {
        if consume(iter, Operator::LParen) {
            let func_prototype = ctx.g.func_prototype_mp.get(&ident.name).ok_or_else(|| {
                iter.prev();
                Error::undefined_function(iter.filepath, iter.s, ident, iter.pos, None)
            })?;
            return Ok(Node::new_leaf(NodeKind::Func(
                func_prototype.clone(),
                func_args(iter, ctx)?,
            )));
        }
        if let Some(lvar) = ctx.s.find_upper_lvar(ident.clone()) {
            if lvar.borrow().dec.is_const.0 {
                return Ok(Node::new_num(lvar.borrow().dec.is_const.1));
            }
            return Ok(Node::new_leaf(NodeKind::Lvar(lvar)));
        } else if let Some(x) = ctx.s.find_upper_gvar(ident.clone()) {
            return Ok(Node::new_leaf(NodeKind::Gvar(x.clone())));
        } else {
            iter.prev();
            return Err(Error::undefined_variable(
                iter.filepath,
                iter.s,
                ident,
                iter.pos,
                None,
            ));
        }
    }

    // str
    if let Some(string) = consume_string(iter) {
        let string = Rc::new(string);
        let idx = ctx.g.tk_string.len();
        let label = format!(".LC{}", idx).to_string();
        ctx.g
            .tk_string
            .push((string.clone(), Rc::new(label.clone())));
        let mut init = Vec::new();
        gvar_init_string(&mut init, string.as_ref().clone());
        return Ok(Node::new_leaf(make_string_node(
            label,
            (string.len()) as u64,
            init,
        )));
    }

    // char
    if let Some(c) = consume_char(iter) {
        return Ok(Node::new_num(c as i64));
    }

    if consume(iter, Operator::Sizeof) {
        if consume(iter, Operator::LParen) {
            if is_typename(iter, ctx) {
                let ty = type_name(iter, ctx)?;
                expect(iter, Operator::RParen)?;
                return Ok(Node::new_num(ty.borrow().size() as i64));
            } else {
                iter.pos.bytes -= 1;
                iter.pos.tk -= 1;
            }
        }
        let node = unary(iter, ctx)?;
        match node.get_type() {
            Ok(x) => return Ok(Node::new_num(x.size() as i64)),
            Err(e) => {
                dbg!("{}", e);
                return Err(Error::todo(iter.filepath, iter.s, iter.pos));
            }
        }
    }

    // num
    return Ok(Node::new_num(expect_num(iter)?));
}

// func-args   = "(" (assign ("," assign)*)? ")"
fn func_args(iter: &mut TokenIter, ctx: &mut Context) -> Result<Vec<Node>, Error> {
    if consume(iter, Operator::RParen) {
        return Ok(vec![]);
    }
    let mut args = vec![assign(iter, ctx)?];
    while consume_comma(iter) {
        args.push(assign(iter, ctx)?);
    }
    expect(iter, Operator::RParen)?;
    Ok(args)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{GlobalContext, Ident, Lvar, NodeKind};
    use crate::base_types::TypeKind;

    use std::rc::Rc;

    #[test]
    fn test_expr() {
        use crate::token;
        let tests = [
            ("1==10", make_test_node(NodeKind::Equal, 1, 10)),
            ("1 != 10", make_test_node(NodeKind::Neq, 1, 10)),
            ("1  <10", make_test_node(NodeKind::Lesser, 1, 10)),
            ("1<=10", make_test_node(NodeKind::Leq, 1, 10)),
            ("1>10", make_test_node(NodeKind::Lesser, 10, 1)), // Lesser,LeqはGreater.Geqを使って実装されてる
            ("1>=10", make_test_node(NodeKind::Leq, 10, 1)),
            ("1+10", make_test_node(NodeKind::Add, 1, 10)),
            ("1-10", make_test_node(NodeKind::Sub, 1, 10)),
            ("1*10", make_test_node(NodeKind::Mul, 1, 10)),
            ("1/10", make_test_node(NodeKind::Div, 1, 10)),
            ("+1", Node::new_num(1)),
            ("-1", make_test_node(NodeKind::Sub, 0, 1)),
            (
                "2 * ( 3 + 4)",
                Node::new(
                    NodeKind::Mul,
                    Node::new_num(2),
                    make_test_node(NodeKind::Add, 3, 4),
                ),
            ),
            ("42", Node::new_num(42)),
        ];

        for (s, expected) in &tests {
            assert_eq!(
                expected,
                &expr(&mut token::tokenize(s, ""), &mut Context::new()).unwrap()
            )
        }
    }

    #[test]
    fn test_stmt() {
        use crate::token;

        let tests = [(
            "int foo;",
            vec![Node::new_leaf(NodeKind::Declaration(make_int_dec("foo")))],
        )];

        for (s, expected) in &tests {
            let mut iter = token::tokenize(s, "");
            let mut actual = Vec::new();

            let ctx = &mut Context::new();
            while iter.peek() != None {
                actual.push(stmt(&mut iter, ctx).unwrap());
            }
            assert_eq!(expected, &actual);
        }
    }

    #[test]
    fn test_unary() {
        use crate::ast::Var;
        use crate::token;
        let expected = vec![
            Node::new_num(1),
            Node::new_num(1),
            Node::new_leaf(NodeKind::Lvar(Rc::new(RefCell::new(Lvar::new_leaf(
                make_int_dec("hoge"),
                8,
            ))))),
            Node::new(NodeKind::Sub, Node::new_num(0), Node::new_num(1)),
            Node::new(
                NodeKind::Sub,
                Node::new_num(0),
                Node::new_leaf(NodeKind::Lvar(Rc::new(RefCell::new(Lvar::new_leaf(
                    make_int_dec("hoge"),
                    8,
                ))))),
            ),
            Node::new_unary(NodeKind::Deref, Node::new_num(1)),
            Node::new_unary(NodeKind::Addr, Node::new_num(1)),
            Node::new_unary(
                NodeKind::Deref,
                Node::new_leaf(NodeKind::Lvar(Rc::new(RefCell::new(Lvar::new_leaf(
                    make_int_dec("hoge"),
                    8,
                ))))),
            ),
            Node::new_unary(
                NodeKind::Addr,
                Node::new_leaf(NodeKind::Lvar(Rc::new(RefCell::new(Lvar::new_leaf(
                    make_int_dec("hoge"),
                    8,
                ))))),
            ),
            Node::new_unary(
                NodeKind::Deref,
                Node::new_unary(
                    NodeKind::Addr,
                    Node::new_leaf(NodeKind::Lvar(Rc::new(RefCell::new(Lvar::new_leaf(
                        make_int_dec("hoge"),
                        8,
                    ))))),
                ),
            ),
        ];

        let input = "1 +1 +hoge -1 -hoge *1 &1 *hoge &hoge *&hoge ";
        let iter = &mut token::tokenize(input, "");
        for i in expected {
            let ctx = &mut Context::new();
            ctx.l.lvar = Some(Rc::new(RefCell::new(make_int_lvar("hoge", 8))));
            ctx.s.insert_v(
                Ident::new("hoge"),
                Rc::new(Var::L(Rc::new(RefCell::new(make_int_lvar("hoge", 8))))),
            );
            assert_eq!(i, unary(iter, ctx).unwrap());
        }

        let expected = vec![
            (Node::new_num(4), make_int_lvar("hoge", 8)),
            (Node::new_num(4), make_int_lvar("hoge", 8)),
            (Node::new_num(8), make_ptr_lvar("hoge", 8)),
            (Node::new_num(4), make_ptr_lvar("hoge", 8)),
        ];

        let input = "sizeof 1 sizeof (hoge) sizeof (hoge) sizeof(*hoge)";
        let iter = &mut token::tokenize(input, "");
        for i in expected {
            let ctx = &mut Context::new();
            ctx.l.lvar = Some(Rc::new(RefCell::new(i.1.clone())));
            ctx.s.insert_v(
                i.1.dec.ident.clone(),
                Rc::new(Var::L(Rc::new(RefCell::new(i.1)))),
            );

            assert_eq!(i.0, unary(iter, ctx).unwrap());
        }
    }

    #[test]
    fn test_declaration() {
        use crate::token;
        use TypeKind::*;
        let tests = [
            ("int hoge", Declaration::new(Int, Ident::new("hoge"))),
            ("int *hoge", make_ptr_lvar("hoge", 8).dec),
            (
                "int **hoge",
                Declaration::new(
                    Ptr(Rc::new(RefCell::new(Ptr(Rc::new(RefCell::new(Int)))))),
                    Ident::new("hoge"),
                ),
            ),
            (
                "int hoge[1]",
                Declaration::new(
                    Array(1, Rc::new(RefCell::new(Int)), true),
                    Ident::new("hoge"),
                ),
            ),
            (
                "int *hoge[1]",
                Declaration::new(
                    Array(
                        1,
                        Rc::new(RefCell::new(Ptr(Rc::new(RefCell::new(Int))))),
                        true,
                    ),
                    Ident::new("hoge"),
                ),
            ),
            ("char hoge", Declaration::new(Char, Ident::new("hoge"))),
            (
                "char **hoge",
                Declaration::new(
                    Ptr(Rc::new(RefCell::new(Ptr(Rc::new(RefCell::new(Char)))))),
                    Ident::new("hoge"),
                ),
            ),
            (
                "char hoge[1]",
                Declaration::new(
                    Array(1, Rc::new(RefCell::new(Char)), true),
                    Ident::new("hoge"),
                ),
            ),
            (
                "char *hoge[1]",
                Declaration::new(
                    Array(
                        1,
                        Rc::new(RefCell::new(Ptr(Rc::new(RefCell::new(Char))))),
                        true,
                    ),
                    Ident::new("hoge"),
                ),
            ),
            (
                "int hoge[2][2]",
                Declaration::new(
                    Array(
                        2,
                        Rc::new(RefCell::new(Array(2, Rc::new(RefCell::new(Int)), true))),
                        true,
                    ),
                    Ident::new("hoge"),
                ),
            ),
        ];

        for (input, expected) in &tests {
            assert_eq!(
                *expected,
                declaration(&mut token::tokenize(input, ""), &mut Context::new()).unwrap()
            );
        }
    }

    #[test]
    fn test_struct_dec() {
        use crate::token;
        use TypeKind::{Char, Int, Ptr};

        let members = Rc::new(vec![
            Rc::new(make_member(Int, "first", 0)),
            Rc::new(make_member(Int, "second", 4)),
        ]);
        let expected = Rc::new(RefCell::new(Struct::new(
            Rc::new(Ident::new("hoge")),
            members,
        )));
        let input = "struct hoge {int first; int second;}";
        let actual = struct_dec(&mut token::tokenize(input, ""), &mut Context::new()).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(8, actual.borrow().get_size());

        let members = Rc::new(vec![
            Rc::new(make_member(Int, "first", 0)),
            Rc::new(make_member(Int, "second", 4)),
            Rc::new(make_member(Char, "third", 8)),
            Rc::new(make_member(Int, "four", 12)),
        ]);
        let expected = Rc::new(RefCell::new(Struct::new(
            Rc::new(Ident::new("hoge")),
            members,
        )));
        let input = "struct hoge {int first; int second; char third; int four;}";
        let actual = struct_dec(&mut token::tokenize(input, ""), &mut Context::new()).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(16, actual.borrow().get_size());

        let members = Rc::new(vec![
            Rc::new(make_member(Int, "first", 0)),
            Rc::new(make_member(Ptr(Rc::new(RefCell::new(Int))), "second", 8)),
            Rc::new(make_member(Int, "four", 16)),
        ]);
        let expected = Rc::new(RefCell::new(Struct::new(
            Rc::new(Ident::new("hoge")),
            members,
        )));
        let input = "struct hoge {int first; int *second; int four;}";
        let actual = struct_dec(&mut token::tokenize(input, ""), &mut Context::new()).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(24, actual.borrow().get_size());
    }

    fn make_member(type_kind: TypeKind, name: impl Into<String>, offset: u64) -> Member {
        Member::new(Rc::new(type_kind), offset, Ident::new(name))
    }

    #[test]
    fn test_if() {
        use crate::token;
        let cond = Node::new(NodeKind::Equal, Node::new_num(10), Node::new_num(20));
        let then = Node::new_unary(NodeKind::Return, Node::new_num(15));
        let expected = make_if_node(cond, then);

        let input = "if ( 10 ==20 ) return 15;";
        let actual = stmt(&mut token::tokenize(input, ""), &mut Context::new()).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_if_else() {
        use crate::token;
        let cond = Node::new(NodeKind::Equal, Node::new_num(10), Node::new_num(20));
        let then = Node::new_unary(NodeKind::Return, Node::new_num(15));
        let els = Node::new_unary(
            NodeKind::Return,
            Node::new(NodeKind::Add, Node::new_num(10), Node::new_num(30)),
        );
        let expected = make_if_else_node(cond, then, els);

        let input = "if ( 10 ==20 ) return 15; else return 10+30;";
        let actual = stmt(&mut token::tokenize(input, ""), &mut Context::new()).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_while() {
        use crate::token;
        // Geqは左右を入れ替えてLeq
        let cond = Node::new(NodeKind::Leq, Node::new_num(20), Node::new_num(32));
        let then = Node::new_unary(NodeKind::Return, Node::new_num(10));
        let expected = make_while_node(cond, then);

        let input = "while (32 >= 20 ) return 10;";
        let actual = stmt(&mut token::tokenize(input, ""), &mut Context::new()).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_for() {
        use crate::ast::Var;
        use crate::token;

        let init = Node::new_unary(NodeKind::ExprStmt, make_assign_node("i", 0, 8));
        let cond = Node::new(
            NodeKind::Lesser,
            Node::new_leaf(NodeKind::Lvar(Rc::new(RefCell::new(Lvar::new_leaf(
                make_int_dec("i"),
                8,
            ))))),
            Node::new_num(10),
        );
        let tmp_inc = Node::new(
            NodeKind::Add,
            Node::new_leaf(NodeKind::Lvar(Rc::new(RefCell::new(Lvar::new_leaf(
                make_int_dec("i"),
                8,
            ))))),
            Node::new_num(1),
        );
        let inc = Node::new_unary(
            NodeKind::ExprStmt,
            Node::new(
                NodeKind::Assign,
                Node::new_leaf(NodeKind::Lvar(Rc::new(RefCell::new(Lvar::new_leaf(
                    make_int_dec("i"),
                    8,
                ))))),
                tmp_inc,
            ),
        );

        let ret = Node::new(
            NodeKind::Add,
            Node::new_leaf(NodeKind::Lvar(Rc::new(RefCell::new(Lvar::new_leaf(
                make_int_dec("i"),
                8,
            ))))),
            Node::new_num(2),
        );
        let then = Node::new_unary(NodeKind::Return, ret);

        let expected = make_for_node(Some(init), Some(cond), Some(inc), then);

        let input = "for( i=0;i<10;i=i+1)return i+2;";
        let ctx = &mut Context::new();
        ctx.l.lvar = Some(Rc::new(RefCell::new(make_int_lvar("i", 8))));
        ctx.s.insert_v(
            Ident::new("i"),
            Rc::new(Var::L(Rc::new(RefCell::new(make_int_lvar("i", 8))))),
        );
        let actual = stmt(&mut token::tokenize(input, ""), ctx).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_block() {
        use crate::token;
        let input = "{1; 2; int hoge; hoge=4;}";
        let expected = vec![
            Node::new_unary(NodeKind::ExprStmt, Node::new_num(1)),
            Node::new_unary(NodeKind::ExprStmt, Node::new_num(2)),
            Node::new_leaf(NodeKind::Declaration(make_int_dec("hoge"))),
            Node::new_unary(NodeKind::ExprStmt, make_assign_node("hoge", 4, 4)),
        ];
        let expected = vec![Node::new_none(NodeKind::Block(expected))];
        let mut iter = token::tokenize(input, "");
        let mut actual = Vec::new();
        while iter.peek() != None {
            actual.push(stmt(&mut iter, &mut Context::new()).unwrap());
        }
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_func() {
        use crate::token;
        let input = "add();";
        let expected_name = "add";
        let expected_args = vec![];
        let expected = Node::new_unary(
            NodeKind::ExprStmt,
            make_fn_node(expected_name, expected_args),
        );
        let mut g_ctx = GlobalContext::new();
        g_ctx.func_prototype_mp.insert(
            "add".to_string(),
            Rc::new(FuncPrototype::new(
                TypeKind::Int,
                Ident::new("add"),
                Vec::new(),
                false,
            )),
        );
        let mut ctx = Context::new();
        ctx.g = g_ctx;

        let mut iter = token::tokenize(input, "");
        let actual = stmt(&mut iter, &mut ctx).unwrap();
        assert_eq!(expected, actual);

        let input = "three(1,2,3);";
        let expected_name = "three";
        let expected_args = vec![Node::new_num(1), Node::new_num(2), Node::new_num(3)];
        let expected = Node::new_unary(
            NodeKind::ExprStmt,
            make_fn_node(expected_name, expected_args),
        );
        let mut g_ctx = GlobalContext::new();
        g_ctx.func_prototype_mp.insert(
            "three".to_string(),
            Rc::new(FuncPrototype::new(
                TypeKind::Int,
                Ident::new("three"),
                vec![
                    Declaration::new(TypeKind::Int, Ident::new("a")),
                    Declaration::new(TypeKind::Int, Ident::new("a")),
                    Declaration::new(TypeKind::Int, Ident::new("a")),
                ],
                false,
            )),
        );

        let mut ctx = Context::new();
        ctx.g = g_ctx;

        let mut iter = token::tokenize(input, "");
        let actual = stmt(&mut iter, &mut ctx).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_func_prototype() {
        use crate::token;

        let expected_func_prototype = Rc::new(FuncPrototype::new(
            TypeKind::Int,
            Ident::new("main"),
            Vec::new(),
            false,
        ));
        let expected_nodes = vec![Node::new_unary(NodeKind::Return, Node::new_num(1))];
        let expected = Function::new(expected_func_prototype, None, 0, expected_nodes, false);

        let input = "{return 1;}";
        let func_prototype = Rc::new(FuncPrototype::new(
            TypeKind::Int,
            Ident::new("main"),
            vec![],
            false,
        ));
        let iter = &mut token::tokenize(input, "");
        let actual = function(iter, func_prototype, &mut Context::new()).unwrap();

        assert_eq!(expected, actual);

        let func_prototype = Rc::new(FuncPrototype::new(
            TypeKind::Int,
            Ident::new("main"),
            vec![],
            false,
        ));
        let lvar1 = Lvar::new_leaf(make_int_dec("foo"), 4);
        let lvar2 = Lvar::new(lvar1.clone(), make_int_dec("bar"), 8);
        let expected_lvar = Rc::new(RefCell::new(lvar2.clone()));
        let node1 = Node::new_leaf(NodeKind::Declaration(make_int_dec("foo")));
        let node2 = Node::new_unary(NodeKind::ExprStmt, make_assign_node("foo", 1, 4));
        let node3 = Node::new_leaf(NodeKind::Declaration(make_int_dec("bar")));
        let node4 = Node::new_unary(
            NodeKind::ExprStmt,
            Node::new(
                NodeKind::Assign,
                Node::new_leaf(NodeKind::Lvar(Rc::new(RefCell::new(lvar2.clone())))),
                Node::new_num(2),
            ),
        );
        let node5 = Node::new_unary(
            NodeKind::Return,
            Node::new(
                NodeKind::Add,
                Node::new_leaf(NodeKind::Lvar(Rc::new(RefCell::new(lvar1)))),
                Node::new_leaf(NodeKind::Lvar(Rc::new(RefCell::new(lvar2)))),
            ),
        );
        let expected_nodes = vec![node1, node2, node3, node4, node5];
        let expected = Function::new(
            func_prototype.clone(),
            Some(expected_lvar),
            2,
            expected_nodes,
            false,
        );

        let input = "{int foo;foo = 1; int bar;bar = 2; return foo+bar;}";
        let iter = &mut token::tokenize(input, "");
        let actual = function(iter, func_prototype, &mut Context::new()).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_param() {
        use crate::token;

        let expected = vec![make_int_dec("hoge")];

        let input = "int hoge)";
        let iter = &mut token::tokenize(input, "");
        let actual = params(iter, &mut Context::new()).unwrap();

        assert_eq!(expected, actual);

        let expected = vec![
            make_int_dec("foo"),
            make_int_dec("bar"),
            make_int_dec("hoge"),
        ];
        let input = "int foo,int bar,int hoge)";
        let iter = &mut token::tokenize(input, "");
        let actual = params(iter, &mut Context::new()).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_fn_with_args() {
        use crate::token;

        let expected_func_prototype = Rc::new(FuncPrototype::new(
            TypeKind::Int,
            Ident::new("main"),
            Vec::new(),
            false,
        ));
        let expected_nodes = vec![Node::new_unary(NodeKind::Return, Node::new_num(0))];
        let expected = Function::new(expected_func_prototype, None, 0, expected_nodes, false);

        let expected_func_prototype = Rc::new(FuncPrototype::new(
            TypeKind::Int,
            Ident::new("main"),
            Vec::new(),
            false,
        ));
        let input = "{return 0;}";
        let iter = &mut token::tokenize(input, "");
        let actual = function(iter, expected_func_prototype, &mut Context::new()).unwrap();

        assert_eq!(expected, actual);

        let func_prototype = Rc::new(FuncPrototype::new(
            TypeKind::Int,
            Ident::new("main"),
            vec![Declaration::new(TypeKind::Int, Ident::new("foo"))],
            false,
        ));
        let expected_nodes = vec![Node::new_unary(NodeKind::Return, Node::new_num(0))];
        let expected_lvar = Lvar::new_leaf(make_int_dec("foo"), 4);
        let expected = Function::new(
            func_prototype,
            Some(Rc::new(RefCell::new(expected_lvar))),
            1,
            expected_nodes,
            false,
        );

        let expected_func_prototype = Rc::new(FuncPrototype::new(
            TypeKind::Int,
            Ident::new("main"),
            vec![Declaration::new(TypeKind::Int, Ident::new("foo"))],
            false,
        ));
        let input = "{return 0;}";
        let iter = &mut token::tokenize(input, "");
        let actual = function(iter, expected_func_prototype, &mut Context::new()).unwrap();

        assert_eq!(expected, actual);

        let expected_param = vec![
            make_int_dec("foo"),
            make_int_dec("bar"),
            make_int_dec("hoge"),
            make_int_dec("hey"),
        ];
        let expected_func_prototype = Rc::new(FuncPrototype::new(
            TypeKind::Int,
            Ident::new("main"),
            expected_param.clone(),
            false,
        ));
        let expected_nodes = vec![Node::new_unary(NodeKind::Return, Node::new_num(0))];
        let expected_lvar = Lvar::new(
            Lvar::new(
                Lvar::new(
                    Lvar::new_leaf(make_int_dec("foo"), 4),
                    make_int_dec("bar"),
                    8,
                ),
                make_int_dec("hoge"),
                12,
            ),
            make_int_dec("hey"),
            16,
        );
        let expected = Function::new(
            expected_func_prototype,
            Some(Rc::new(RefCell::new(expected_lvar))),
            4,
            expected_nodes,
            false,
        );

        let input = "{return 0;}";
        let func_prototype = Rc::new(FuncPrototype::new(
            TypeKind::Int,
            Ident::new("main"),
            expected_param,
            false,
        ));
        let iter = &mut token::tokenize(input, "");
        let actual = function(iter, func_prototype, &mut Context::new()).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_primary() {
        use crate::token;
        let mut g_ctx_1 = GlobalContext::new();
        g_ctx_1.func_prototype_mp.insert(
            "foo".to_string(),
            Rc::new(FuncPrototype::new(
                TypeKind::Int,
                Ident::new("foo"),
                Vec::new(),
                false,
            )),
        );
        let mut g_ctx_2 = GlobalContext::new();
        g_ctx_2.func_prototype_mp.insert(
            "foo".to_string(),
            Rc::new(FuncPrototype::new(
                TypeKind::Int,
                Ident::new("foo"),
                vec![Declaration::new(TypeKind::Int, Ident::new("a"))],
                false,
            )),
        );
        let mut g_ctx_3 = GlobalContext::new();
        g_ctx_3.func_prototype_mp.insert(
            "foo".to_string(),
            Rc::new(FuncPrototype::new(
                TypeKind::Int,
                Ident::new("foo"),
                vec![
                    Declaration::new(TypeKind::Int, Ident::new("a")),
                    Declaration::new(TypeKind::Int, Ident::new("a")),
                ],
                false,
            )),
        );
        let mut ini = Vec::new();
        gvar_init_string(&mut ini, "aaa\u{0}".to_string());
        let tests = [
            ("1", Node::new_num(1), g_ctx_1.clone()),
            ("foo()", make_fn_node("foo", vec![]), g_ctx_1),
            (
                "foo(1)",
                make_fn_node("foo", vec![Node::new_num(1)]),
                g_ctx_2,
            ),
            (
                "foo(1,2)",
                make_fn_node("foo", vec![Node::new_num(1), Node::new_num(2)]),
                g_ctx_3,
            ),
            (
                "\"aaa\"",
                Node::new_leaf(super::make_string_node(".LC0", 4, ini)),
                GlobalContext::new(),
            ),
        ];

        for (input, expected, g) in &tests {
            let iter = &mut token::tokenize(input, "");
            let mut ctx = Context::new();
            ctx.g = g.clone();
            assert_eq!(expected, &primary(iter, &mut ctx).unwrap());
        }
    }

    #[test]
    fn test_declarator() {
        use crate::token::tokenize;
        use TypeKind::*;
        let tests = [
            (Int, "hoge", Int),
            (Char, "hoge", Char),
            (Int, "*hoge", Ptr(Rc::new(RefCell::new(Int)))),
            (Char, "hoge[1]", Array(1, Rc::new(RefCell::new(Char)), true)),
            (
                Int,
                "**hoge[4][5]",
                TypeKind::array_of(
                    4,
                    Rc::new(RefCell::new(TypeKind::array_of(
                        5,
                        Int.get_addr_type().borrow().get_addr_type(),
                        true,
                    ))),
                    true,
                ),
            ),
            (
                Char,
                "(*hoge)[3]",
                TypeKind::array_of(3, Rc::new(RefCell::new(Char)), true)
                    .get_addr_type()
                    .replace(Int),
            ),
        ];

        for (sp, ipt, expected) in &tests {
            let mut ident = Ident::new_anonymous();
            assert_eq!(
                expected,
                &*declarator(
                    &mut tokenize(ipt, ""),
                    &mut Context::new(),
                    Rc::new(RefCell::new(sp.clone())),
                    &mut ident
                )
                .unwrap()
                .borrow()
            );
            assert_eq!(Ident::new("hoge"), ident);
        }
    }

    fn make_test_node(kind: NodeKind, lhs_num: i64, rhs_num: i64) -> Node {
        Node::new(kind, Node::new_num(lhs_num), Node::new_num(rhs_num))
    }

    fn make_assign_node(lhs: impl Into<String>, rhs: i64, offset: u64) -> Node {
        let mut node = Node::new_none(NodeKind::Assign);
        node.lhs = Some(Box::new(Node::new_leaf(NodeKind::Lvar(Rc::new(
            RefCell::new(Lvar::new_leaf(make_int_dec(lhs.into()), offset)),
        )))));
        node.rhs = Some(Box::new(Node::new_num(rhs)));
        node
    }

    fn make_if_node(cond: Node, then: Node) -> Node {
        let mut node = Node::new_none(NodeKind::If);
        node.cond = Some(Box::new(cond));
        node.then = Some(Box::new(then));
        node
    }

    fn make_if_else_node(cond: Node, then: Node, els: Node) -> Node {
        let mut node = make_if_node(cond, then);
        node.els = Some(Box::new(els));
        node
    }

    fn make_while_node(cond: Node, then: Node) -> Node {
        let mut node = Node::new_none(NodeKind::While);
        node.cond = Some(Box::new(cond));
        node.then = Some(Box::new(then));
        node
    }

    fn make_for_node(
        init: Option<Node>,
        cond: Option<Node>,
        inc: Option<Node>,
        then: Node,
    ) -> Node {
        let mut node = Node::new_none(NodeKind::For);
        node.init = init.map(|c| vec![c]);
        node.cond = cond.map(|c| Box::new(c));
        node.inc = inc.map(|c| Box::new(c));
        node.then = Some(Box::new(then));
        node
    }

    fn make_fn_node(name: impl Into<String>, args: Vec<Node>) -> Node {
        let mut dec = vec![];
        for _ in 0..args.len() {
            dec.push(Declaration::new(TypeKind::Int, Ident::new("a")));
        }
        Node::new_none(NodeKind::Func(
            Rc::new(FuncPrototype::new(
                TypeKind::Int,
                Ident::new(name),
                dec,
                false,
            )),
            args,
        ))
    }

    fn make_int_dec(name: impl Into<String>) -> Declaration {
        Declaration::new(TypeKind::Int, Ident::new(name.into()))
    }

    fn make_lvar(name: impl Into<String>, offset: u64, kind: TypeKind) -> Lvar {
        Lvar::new_leaf(Declaration::new(kind, Ident::new(name)), offset)
    }

    fn make_int_lvar(name: impl Into<String>, offset: u64) -> Lvar {
        make_lvar(name, offset, TypeKind::Int)
    }

    fn make_ptr_lvar(name: impl Into<String>, offset: u64) -> Lvar {
        make_lvar(
            name,
            offset,
            TypeKind::Ptr(Rc::new(RefCell::new(TypeKind::Int))),
        )
    }
}
