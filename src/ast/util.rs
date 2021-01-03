use super::error::Error;
use super::{
    Context, Declaration, FuncPrototype, FuncPrototypeMp, Gvar, GvarMp, Ident, Lvar, Node, NodeKind,
};
use crate::base_types;
use crate::base_types::TypeKind;
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

pub(crate) fn consume_declaration(iter: &mut TokenIter, ctx: &mut Context) -> Option<Declaration> {
    crate::ast::ast::declaration(iter, ctx).ok() // todo エラー握りつぶしてるので注意
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

pub(crate) fn expect_num(iter: &mut TokenIter) -> Result<u64, Error> {
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

pub(crate) fn make_array_idx_node(idx: u64, lvar: Rc<Lvar>) -> Node {
    Node::new(
        NodeKind::Add,
        Node::new_leaf(NodeKind::Lvar(lvar)),
        Node::new_num(idx),
    )
}

pub(crate) fn make_arr_init(
    lvar: Rc<Lvar>,
    dec: &Declaration,
    nodes: Vec<Node>,
) -> Result<Node, usize> {
    if let TypeKind::Array(size, _, _) = dec.type_kind {
        let mut assign_nodes = Vec::new();
        let len = nodes.len();
        let mut idx = 0;
        for node in nodes {
            assign_nodes.push(Node::new_expr_stmt(Node::new_assign(
                Node::new_unary(NodeKind::Deref, make_array_idx_node(idx, lvar.clone())),
                node,
            )));
            idx += 1;
        }
        if size < assign_nodes.len() as u64 {
            return Err(assign_nodes.len());
        }
        for _ in 0..(size - len as u64) {
            assign_nodes.push(Node::new_expr_stmt(Node::new_assign(
                Node::new_unary(NodeKind::Deref, make_array_idx_node(idx, lvar.clone())),
                Node::new_num(0),
            )));
            idx += 1;
        }
        return Ok(Node::new_init(
            NodeKind::Declaration(dec.clone()),
            assign_nodes,
        ));
    } else {
        unreachable!()
    }
}

pub(crate) fn make_unary_init(lvar: Rc<Lvar>, dec: &Declaration, node: Node) -> Result<Node, ()> {
    let node = Node::new_unary(
        NodeKind::ExprStmt,
        Node::new(NodeKind::Assign, Node::new_leaf(NodeKind::Lvar(lvar)), node),
    );
    Ok(Node::new_init(
        NodeKind::Declaration(dec.clone()),
        vec![node],
    ))
}
