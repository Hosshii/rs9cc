use super::error::Error;
use super::{
    Context, Declaration, Designator, FuncPrototype, FuncPrototypeMp, Gvar, GvarMp, Ident, Node,
    NodeKind, Var,
};
use crate::base_types::{self, TagTypeKind, TypeKind};

use crate::token::{Block, KeyWord, Operator, TokenIter, TokenKind};
use std::{cell::RefCell, rc::Rc};

pub(crate) fn consume(iter: &mut TokenIter, op: Operator) -> bool {
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

pub(crate) fn _consume_keyword(iter: &mut TokenIter, key: KeyWord) -> bool {
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

pub(crate) fn consume_semi(iter: &mut TokenIter) -> bool {
    if let Some(x) = iter.peek() {
        if x.kind == TokenKind::SemiColon {
            iter.next();
            return true;
        }
    }
    return false;
}

pub(crate) fn consume_colon(iter: &mut TokenIter) -> bool {
    if let Some(x) = iter.peek() {
        if x.kind == TokenKind::Colon {
            iter.next();
            return true;
        }
    }
    return false;
}

pub(crate) fn consume_question(iter: &mut TokenIter) -> bool {
    if let Some(x) = iter.peek() {
        if x.kind == TokenKind::Question {
            iter.next();
            return true;
        }
    }
    return false;
}

pub(crate) fn consume_ident(iter: &mut TokenIter) -> Option<Ident> {
    if let Some(x) = iter.peek() {
        if let TokenKind::Ident(x) = x.kind {
            iter.next();
            return Some(Ident::new(x.name));
        }
    }
    return None;
}

pub(crate) fn consume_block(iter: &mut TokenIter, block: Block) -> bool {
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

pub(crate) fn consume_comma(iter: &mut TokenIter) -> bool {
    if let Some(x) = iter.peek() {
        if x.kind == TokenKind::Comma {
            iter.next();
            return true;
        }
    }
    false
}

pub(crate) fn _consume_period(iter: &mut TokenIter) -> bool {
    if let Some(x) = iter.peek() {
        if x.kind == TokenKind::Period {
            iter.next();
            return true;
        }
    }
    false
}

pub(crate) fn consume_string(iter: &mut TokenIter) -> Option<String> {
    if let Some(x) = iter.peek() {
        if let TokenKind::String(string) = x.kind {
            iter.next();
            return Some(string);
        }
    }
    None
}

pub(crate) fn consume_char(iter: &mut TokenIter) -> Option<char> {
    if let Some(x) = iter.peek() {
        if let TokenKind::Char(c) = x.kind {
            iter.next();
            return Some(c);
        }
    }
    None
}

pub(crate) fn consume_declarator(
    iter: &mut TokenIter,
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

#[allow(dead_code)]
pub(crate) fn consume_type_kind(iter: &mut TokenIter) -> Option<base_types::TypeKind> {
    if let Some(x) = iter.peek() {
        if let TokenKind::TypeKind(bt) = x.kind {
            iter.next();
            return Some(bt);
        }
    }
    None
}

pub(crate) fn _consume_token_kind(iter: &mut TokenIter, kind: TokenKind) -> Option<TokenKind> {
    if let Some(x) = iter.peek() {
        if x.kind == kind {
            iter.next();
            return Some(x.kind);
        }
    }
    None
}

pub(crate) fn expect(iter: &mut TokenIter, op: Operator) -> Result<(), Error> {
    if let Some(x) = iter.peek() {
        if let TokenKind::Reserved(xx) = x.kind {
            if xx == op {
                iter.next();
                return Ok(());
            } else {
                return Err(Error::unexpected_token(
                    iter.filepath,
                    iter.s,
                    x.clone(),
                    TokenKind::Reserved(op),
                ));
            }
        }
    }
    return Err(Error::eof(
        iter.filepath,
        iter.s,
        iter.pos,
        TokenKind::Reserved(op),
        None,
    ));
}

pub(crate) fn expect_num(iter: &mut TokenIter) -> Result<i64, Error> {
    if let Some(x) = iter.peek() {
        if let TokenKind::Num(xx) = x.kind {
            iter.next();
            return Ok(xx);
        } else {
            return Err(Error::unexpected_token(
                iter.filepath,
                iter.s,
                x.clone(),
                TokenKind::Num(0),
            ));
        }
    }
    Err(Error::eof(
        iter.filepath,
        iter.s,
        iter.pos,
        TokenKind::Num(0),
        None,
    ))
}

pub(crate) fn expect_semi(iter: &mut TokenIter) -> Result<(), Error> {
    if let Some(x) = iter.peek() {
        if x.kind == TokenKind::SemiColon {
            iter.next();
            return Ok(());
        } else {
            return Err(Error::unexpected_token(
                iter.filepath,
                iter.s,
                x.clone(),
                TokenKind::SemiColon,
            ));
        }
    }
    Err(Error::eof(
        iter.filepath,
        iter.s,
        iter.pos,
        TokenKind::SemiColon,
        None,
    ))
}

pub(crate) fn expect_colon(iter: &mut TokenIter) -> Result<(), Error> {
    if let Some(x) = iter.peek() {
        if x.kind == TokenKind::Colon {
            iter.next();
            return Ok(());
        } else {
            return Err(Error::unexpected_token(
                iter.filepath,
                iter.s,
                x.clone(),
                TokenKind::Colon,
            ));
        }
    }
    Err(Error::eof(
        iter.filepath,
        iter.s,
        iter.pos,
        TokenKind::Colon,
        None,
    ))
}

pub(crate) fn expect_keyword(iter: &mut TokenIter, keyword: KeyWord) -> Result<(), Error> {
    if let Some(x) = iter.peek() {
        if x.kind == TokenKind::KeyWord(keyword) {
            iter.next();
            return Ok(());
        } else {
            return Err(Error::unexpected_token(
                iter.filepath,
                iter.s,
                x.clone(),
                TokenKind::SemiColon,
            ));
        }
    }
    Err(Error::eof(
        iter.filepath,
        iter.s,
        iter.pos,
        TokenKind::SemiColon,
        None,
    ))
}
pub(crate) fn _expect_comma(iter: &mut TokenIter) -> Result<(), Error> {
    expect_token_kind(iter, TokenKind::Comma)?;
    Ok(())
}

#[allow(dead_code)]
pub(crate) fn expect_token_kind(iter: &mut TokenIter, kind: TokenKind) -> Result<TokenKind, Error> {
    if let Some(x) = iter.peek() {
        if x.kind == kind {
            iter.next();
            return Ok(x.kind);
        } else {
            return Err(Error::unexpected_token(
                iter.filepath,
                iter.s,
                x.clone(),
                TokenKind::SemiColon,
            ));
        }
    }
    Err(Error::eof(
        iter.filepath,
        iter.s,
        iter.pos,
        TokenKind::SemiColon,
        None,
    ))
}

pub(crate) fn expect_ident(iter: &mut TokenIter) -> Result<Ident, Error> {
    if let Some(x) = iter.peek() {
        if let TokenKind::Ident(id) = x.kind {
            iter.next();
            return Ok(Ident::new(id.name));
        } else {
            return Err(Error::unexpected_token(
                iter.filepath,
                iter.s,
                x.clone(),
                TokenKind::Ident(crate::token::Ident::new("")),
            ));
        }
    }
    Err(Error::eof(
        iter.filepath,
        iter.s,
        iter.pos,
        TokenKind::Ident(crate::token::Ident::new("")),
        None,
    ))
}

pub(crate) fn expect_block(iter: &mut TokenIter, block: Block) -> Result<(), Error> {
    if let Some(x) = iter.peek() {
        if let TokenKind::Block(x) = x.kind {
            if x == block {
                iter.next();
                return Ok(());
            }
        } else {
            return Err(Error::unexpected_token(
                iter.filepath,
                iter.s,
                x.clone(),
                TokenKind::Block(block),
            ));
        }
    }
    Err(Error::eof(
        iter.filepath,
        iter.s,
        iter.pos,
        TokenKind::Block(block),
        None,
    ))
}

/// if global var is already exist, then return error
pub(crate) fn check_g_var(
    iter: &mut TokenIter,
    g_var: &GvarMp,
    type_kind: TypeKind,
    ident: Ident,
    init: Vec<Node>,
) -> Result<Gvar, Error> {
    match g_var.get(&ident.name) {
        Some(_) => {
            return Err(Error::re_declare(
                iter.filepath,
                iter.s,
                ident,
                iter.pos,
                None,
            ))
        }
        None => {
            let size = type_kind.size();
            let dec = Declaration::new(type_kind, ident);
            return Ok(Gvar::new(dec, size, init));
        }
    }
}

pub(crate) fn check_func_prototype(
    iter: &TokenIter,
    func_prototype_mp: &FuncPrototypeMp,
    func_prototype: FuncPrototype,
) -> Result<FuncPrototype, Error> {
    match func_prototype_mp.get(&func_prototype.ident.name) {
        Some(_) => {
            return Err(Error::re_declare(
                iter.filepath,
                iter.s,
                func_prototype.ident,
                iter.pos,
                None,
            ))
        }
        None => return Ok(func_prototype),
    }
}

pub(crate) fn make_string_node(label: impl Into<String>, size: u64, init: Vec<Node>) -> NodeKind {
    NodeKind::Gvar(Rc::new(Gvar::new(
        Declaration::new(
            TypeKind::Array(size, Rc::new(RefCell::new(TypeKind::Char)), true),
            Ident::new(label),
        ),
        size,
        init,
    )))
}

pub(crate) fn is_typename(iter: &mut TokenIter, ctx: &Context) -> bool {
    if let Some(x) = iter.peek() {
        match x.kind {
            TokenKind::TypeKind(_) => return true,
            TokenKind::KeyWord(KeyWord::Struct)
            | TokenKind::KeyWord(KeyWord::Static)
            | TokenKind::KeyWord(KeyWord::Typedef)
            | TokenKind::KeyWord(KeyWord::Enum) => return true,
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
    if let Some(tag) = ctx.s.find_upper_tag(ident) {
        if let TagTypeKind::Typedef(dec) = tag.as_ref() {
            return Some(dec.clone());
        }
    }
    None
}

pub(crate) fn peek_end(iter: &mut TokenIter) -> bool {
    let pos = iter.pos;
    let end = if consume_block(iter, Block::RParen)
        || (consume_comma(iter) && consume_block(iter, Block::RParen))
    {
        true
    } else {
        false
    };
    iter.pos = pos;
    end
}

pub(crate) fn expect_end(iter: &mut TokenIter) -> Result<(), Error> {
    if consume_comma(iter) && consume_block(iter, Block::RParen) {
        Ok(())
    } else {
        expect_block(iter, Block::RParen)?;
        Ok(())
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
