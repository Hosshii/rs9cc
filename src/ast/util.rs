use super::{
    ast::assign,
    error::{Error, Warn},
};
use super::{
    Context, Declaration, Designator, FuncPrototype, FuncPrototypeMp, Gvar, GvarMp, Ident,
    Initializer, Node, NodeKind, Var,
};
use crate::base_types::{self, Member, TagTypeKind, TypeKind};

use crate::token::{Block, KeyWord, Operator, TokenKind, TokenStream};
use std::{cell::RefCell, rc::Rc};

pub(crate) fn consume(iter: &mut TokenStream, op: Operator) -> bool {
    if let Some(x) = iter.peek() {
        if let TokenKind::Reserved(x) = x.kind {
            if x == op {
                iter.next();
                return true;
            }
        }
    }
    return false;
}

pub(crate) fn _consume_keyword(iter: &mut TokenStream, key: KeyWord) -> bool {
    if let Some(x) = iter.peek() {
        if let TokenKind::KeyWord(x) = x.kind {
            if x == key {
                iter.next();
                return true;
            }
        }
    }
    false
}

pub(crate) fn consume_semi(iter: &mut TokenStream) -> bool {
    if let Some(x) = iter.peek() {
        if x.kind == TokenKind::SemiColon {
            iter.next();
            return true;
        }
    }
    return false;
}

pub(crate) fn consume_colon(iter: &mut TokenStream) -> bool {
    if let Some(x) = iter.peek() {
        if x.kind == TokenKind::Colon {
            iter.next();
            return true;
        }
    }
    return false;
}

pub(crate) fn consume_question(iter: &mut TokenStream) -> bool {
    if let Some(x) = iter.peek() {
        if x.kind == TokenKind::Question {
            iter.next();
            return true;
        }
    }
    return false;
}

pub(crate) fn consume_ident(iter: &mut TokenStream) -> Option<Ident> {
    if let Some(x) = iter.peek() {
        if let TokenKind::Ident(x) = x.kind {
            iter.next();
            return Some(Ident::new(x.name));
        }
    }
    return None;
}

pub(crate) fn consume_block(iter: &mut TokenStream, block: Block) -> bool {
    if let Some(x) = iter.peek() {
        if let TokenKind::Block(x) = x.kind {
            if x == block {
                iter.next();
                return true;
            }
        }
    }
    return false;
}

pub(crate) fn consume_comma(iter: &mut TokenStream) -> bool {
    if let Some(x) = iter.peek() {
        if x.kind == TokenKind::Comma {
            iter.next();
            return true;
        }
    }
    false
}

pub(crate) fn _consume_period(iter: &mut TokenStream) -> bool {
    if let Some(x) = iter.peek() {
        if x.kind == TokenKind::Period {
            iter.next();
            return true;
        }
    }
    false
}

pub(crate) fn consume_string(iter: &mut TokenStream) -> Option<String> {
    if let Some(x) = iter.peek() {
        if let TokenKind::String(string) = x.kind {
            iter.next();
            return Some(string + "\0");
        }
    }
    None
}

pub(crate) fn consume_char(iter: &mut TokenStream) -> Option<char> {
    if let Some(x) = iter.peek() {
        if let TokenKind::Char(c) = x.kind {
            iter.next();
            return Some(c);
        }
    }
    None
}

pub(crate) fn consume_declarator(
    iter: &mut TokenStream,
    ctx: &mut Context,
    type_kind: Rc<RefCell<TypeKind>>,
    ident: &mut Ident,
) -> Option<Rc<RefCell<TypeKind>>> {
    match crate::ast::ast::declarator(
        &mut iter.clone(),
        &mut ctx.clone(),
        type_kind.clone(),
        &mut ident.clone(),
    ) {
        Ok(_) => Some(crate::ast::ast::declarator(iter, ctx, type_kind, ident).unwrap()),
        Err(_) => None,
    }
}

pub(crate) fn consume_type_kind(iter: &mut TokenStream) -> Option<base_types::TypeKind> {
    if let Some(x) = iter.peek() {
        if let TokenKind::TypeKind(bt) = x.kind {
            iter.next();
            return Some(bt);
        }
    }
    None
}

pub(crate) fn _consume_token_kind(iter: &mut TokenStream, kind: TokenKind) -> Option<TokenKind> {
    if let Some(x) = iter.peek() {
        if x.kind == kind {
            iter.next();
            return Some(x.kind);
        }
    }
    None
}

pub(crate) fn expect(iter: &mut TokenStream, op: Operator) -> Result<(), Error> {
    if let Some(x) = iter.peek() {
        if let TokenKind::Reserved(xx) = x.kind {
            if xx == op {
                iter.next();
                return Ok(());
            } else {
                return Err(Error::unexpected_token(
                    iter.filepath.clone(),
                    iter.input.clone(),
                    &x,
                    TokenKind::Reserved(op),
                ));
            }
        }
    }
    return Err(Error::eof(
        iter.filepath.clone(),
        iter.input.clone(),
        iter.pos,
        TokenKind::Reserved(op),
        None,
    ));
}

pub(crate) fn expect_num(iter: &mut TokenStream) -> Result<i64, Error> {
    if let Some(x) = iter.peek() {
        if let TokenKind::Num(xx) = x.kind {
            iter.next();
            return Ok(xx);
        } else {
            return Err(Error::unexpected_token(
                iter.filepath.clone(),
                iter.input.clone(),
                &x,
                TokenKind::Num(0),
            ));
        }
    }
    Err(Error::eof(
        iter.filepath.clone(),
        iter.input.clone(),
        iter.pos,
        TokenKind::Num(0),
        None,
    ))
}

pub(crate) fn expect_semi(iter: &mut TokenStream) -> Result<(), Error> {
    if let Some(x) = iter.peek() {
        if x.kind == TokenKind::SemiColon {
            iter.next();
            return Ok(());
        } else {
            return Err(Error::unexpected_token(
                iter.filepath.clone(),
                iter.input.clone(),
                &x,
                TokenKind::SemiColon,
            ));
        }
    }
    Err(Error::eof(
        iter.filepath.clone(),
        iter.input.clone(),
        iter.pos,
        TokenKind::SemiColon,
        None,
    ))
}

pub(crate) fn expect_colon(iter: &mut TokenStream) -> Result<(), Error> {
    if let Some(x) = iter.peek() {
        if x.kind == TokenKind::Colon {
            iter.next();
            return Ok(());
        } else {
            return Err(Error::unexpected_token(
                iter.filepath.clone(),
                iter.input.clone(),
                &x,
                TokenKind::Colon,
            ));
        }
    }
    Err(Error::eof(
        iter.filepath.clone(),
        iter.input.clone(),
        iter.pos,
        TokenKind::Colon,
        None,
    ))
}

pub(crate) fn expect_keyword(iter: &mut TokenStream, keyword: KeyWord) -> Result<(), Error> {
    if let Some(x) = iter.peek() {
        if x.kind == TokenKind::KeyWord(keyword) {
            iter.next();
            return Ok(());
        } else {
            return Err(Error::unexpected_token(
                iter.filepath.clone(),
                iter.input.clone(),
                &x,
                TokenKind::SemiColon,
            ));
        }
    }
    Err(Error::eof(
        iter.filepath.clone(),
        iter.input.clone(),
        iter.pos,
        TokenKind::SemiColon,
        None,
    ))
}
pub(crate) fn expect_comma(iter: &mut TokenStream) -> Result<(), Error> {
    expect_token_kind(iter, TokenKind::Comma)?;
    Ok(())
}

#[allow(dead_code)]
pub(crate) fn expect_token_kind(
    iter: &mut TokenStream,
    kind: TokenKind,
) -> Result<TokenKind, Error> {
    if let Some(x) = iter.peek() {
        if x.kind == kind {
            iter.next();
            return Ok(x.kind);
        } else {
            return Err(Error::unexpected_token(
                iter.filepath.clone(),
                iter.input.clone(),
                &x,
                TokenKind::SemiColon,
            ));
        }
    }
    Err(Error::eof(
        iter.filepath.clone(),
        iter.input.clone(),
        iter.pos,
        TokenKind::SemiColon,
        None,
    ))
}

pub(crate) fn expect_ident(iter: &mut TokenStream) -> Result<Ident, Error> {
    if let Some(x) = iter.peek() {
        if let TokenKind::Ident(id) = x.kind {
            iter.next();
            return Ok(Ident::new(id.name));
        } else {
            return Err(Error::unexpected_token(
                iter.filepath.clone(),
                iter.input.clone(),
                &x,
                TokenKind::Ident(crate::token::Ident::new("")),
            ));
        }
    }
    Err(Error::eof(
        iter.filepath.clone(),
        iter.input.clone(),
        iter.pos,
        TokenKind::Ident(crate::token::Ident::new("")),
        None,
    ))
}

pub(crate) fn expect_block(iter: &mut TokenStream, block: Block) -> Result<(), Error> {
    if let Some(x) = iter.peek() {
        if let TokenKind::Block(x) = x.kind {
            if x == block {
                iter.next();
                return Ok(());
            }
        } else {
            return Err(Error::unexpected_token(
                iter.filepath.clone(),
                iter.input.clone(),
                &x,
                TokenKind::Block(block),
            ));
        }
    }
    Err(Error::eof(
        iter.filepath.clone(),
        iter.input.clone(),
        iter.pos,
        TokenKind::Block(block),
        None,
    ))
}

/// if global var is already exist, then return error
pub(crate) fn check_g_var(
    iter: &mut TokenStream,
    g_var: &GvarMp,
    dec: Declaration,
    init: Vec<Initializer>,
) -> Result<Gvar, Error> {
    match g_var.get(&dec.ident.name) {
        Some(_) => {
            return Err(Error::re_declare(
                iter.filepath.clone().clone(),
                iter.input.clone().clone(),
                dec.ident,
                iter.pos,
                None,
            ))
        }
        None => {
            let size = dec.type_kind.size();
            return Ok(Gvar::new(dec, size, init));
        }
    }
}

pub(crate) fn check_func_prototype(
    iter: &TokenStream,
    func_prototype_mp: &FuncPrototypeMp,
    func_prototype: FuncPrototype,
) -> Result<FuncPrototype, Error> {
    match func_prototype_mp.get(&func_prototype.ident.name) {
        Some(_) => {
            return Err(Error::re_declare(
                iter.filepath.clone(),
                iter.input.clone(),
                func_prototype.ident,
                iter.pos,
                None,
            ))
        }
        None => return Ok(func_prototype),
    }
}

pub(crate) fn make_string_node(
    label: impl Into<String>,
    size: u64,
    init: Vec<Initializer>,
) -> NodeKind {
    NodeKind::Gvar(Rc::new(Gvar::new(
        Declaration::new(
            TypeKind::Array(size, Rc::new(RefCell::new(TypeKind::Char)), true),
            Ident::new(label),
        ),
        size,
        init,
    )))
}

pub(crate) fn is_typename(iter: &mut TokenStream, ctx: &Context) -> bool {
    if let Some(x) = iter.peek() {
        match x.kind {
            TokenKind::TypeKind(_) => return true,
            TokenKind::KeyWord(KeyWord::Struct)
            | TokenKind::KeyWord(KeyWord::Static)
            | TokenKind::KeyWord(KeyWord::Typedef)
            | TokenKind::KeyWord(KeyWord::Enum)
            | TokenKind::KeyWord(KeyWord::Extern) => return true,
            TokenKind::Ident(ident) => {
                let ident = Rc::new(Ident::from(ident.clone()));

                if let Some(_) = is_typedef_name(ident.clone(), ctx) {
                    return true;
                } else {
                    return false;
                }
            }
            _ => (),
        }
    }
    false
}

pub(crate) fn is_typedef_name(ident: Rc<Ident>, ctx: &Context) -> Option<Rc<Declaration>> {
    if let Some(tag) = ctx
        .s
        .find_upper_tag(Rc::new(ident.as_ref().clone().get_typedef_ident()))
    {
        if let TagTypeKind::Typedef(dec) = tag.as_ref() {
            return Some(dec.clone());
        }
    }
    None
}

pub(crate) fn peek_end(iter: &mut TokenStream) -> bool {
    let idx = iter.idx;
    let end = if consume_block(iter, Block::RParen)
        || (consume_comma(iter) && consume_block(iter, Block::RParen))
    {
        true
    } else {
        false
    };
    iter.idx = idx;
    end
}

pub(crate) fn expect_end(iter: &mut TokenStream) -> Result<(), Error> {
    if consume_comma(iter) && consume_block(iter, Block::RParen) {
        Ok(())
    } else {
        expect_block(iter, Block::RParen)?;
        Ok(())
    }
}

pub(crate) fn consume_end(iter: &mut TokenStream) -> bool {
    let idx = iter.idx;
    match expect_end(iter) {
        Ok(_) => true,
        Err(_) => {
            iter.idx = idx;
            false
        }
    }
}

pub(crate) fn new_desg_node2(var: Var, desg: &mut Option<Box<Designator>>) -> Result<Node, Error> {
    match desg {
        None => Ok(Node::new_var(var)),
        Some(desg) => {
            let mut node = new_desg_node2(var, &mut desg.next)?;
            if let Some(member) = &desg.member {
                node = Node::new_unary(
                    NodeKind::Member(member.as_ref().ident.clone(), member.clone()),
                    node,
                );
                return Ok(node);
            }
            node = Node::new(NodeKind::Add, node, Node::new_num(desg.idx as i64));
            Ok(Node::new_unary(NodeKind::Deref, node))
        }
    }
}

pub(crate) fn new_desg_node(
    var: Var,
    desg: &mut Option<Box<Designator>>,
    rhs: Node,
) -> Result<Node, Error> {
    let lhs = new_desg_node2(var, desg)?;
    let node = Node::new(NodeKind::Assign, lhs, rhs);
    Ok(Node::new_expr_stmt(node))
}

pub(crate) fn new_init_val(initializers: &mut Vec<Initializer>, size: u64, val: i64) {
    initializers.push(Initializer::Val(size, val))
}

pub(crate) fn new_init_label(initializers: &mut Vec<Initializer>, label: String, addend: i64) {
    initializers.push(Initializer::Label(label, addend));
}

pub(crate) fn gvar_init_string(initializers: &mut Vec<Initializer>, content: String) {
    for i in content.as_bytes() {
        new_init_val(initializers, 1, *i as i64);
    }
}

pub(crate) fn new_init_zero(initializers: &mut Vec<Initializer>, nbytes: u64) {
    for _ in 0..nbytes {
        new_init_val(initializers, 1, 0);
    }
}

pub(crate) fn emit_struct_padding(
    initializer: &mut Vec<Initializer>,
    size: u64,
    member: Rc<Member>,
    next: Option<Rc<Member>>,
) {
    let end = member.offset + member.type_kind.size();

    let padding = match next {
        Some(x) => x.offset - end,
        None => size - end,
    };
    if padding > 0 {
        new_init_zero(initializer, padding);
    }
}

pub(crate) fn skip_excess_element2(iter: &mut TokenStream, ctx: &mut Context) -> Result<(), Error> {
    loop {
        if consume_block(iter, Block::LParen) {
            skip_excess_element2(iter, ctx)?;
        } else {
            assign(iter, ctx)?;
        }
        if consume_end(iter) {
            return Ok(());
        }
        expect_comma(iter)?;
    }
}

pub(crate) fn skip_excess_elements(iter: &mut TokenStream, ctx: &mut Context) -> Result<(), Error> {
    expect_comma(iter)?;
    Warn::excess_initializer(iter.filepath.clone(), iter.input.clone(), iter.pos);
    skip_excess_element2(iter, ctx)
}
