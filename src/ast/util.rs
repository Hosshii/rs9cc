use super::error::Error;
use super::{Declaration, Gvar, GvarMp, Ident, Lvar, Node, NodeKind};
use crate::base_types;
use crate::base_types::{BaseType, TypeKind};
use crate::token::{Block, KeyWord, Operator, TokenIter, TokenKind};
use std::rc::Rc;

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

#[allow(dead_code)]
pub(crate) fn consume_base_type(iter: &mut TokenIter) -> Option<base_types::BaseType> {
    if let Some(kind) = consume_type_kind(iter) {
        let mut btype = BaseType::new(kind);
        loop {
            if consume(iter, Operator::Mul) {
                btype = BaseType::new(TypeKind::Ptr(Rc::new(btype)));
            } else {
                break;
            }
        }
        return Some(btype);
    }
    None
}

pub(crate) fn consume_declaration(iter: &mut TokenIter) -> Option<Declaration> {
    crate::ast::ast::declaration(iter).ok()
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
                    iter.s,
                    x.clone(),
                    TokenKind::Reserved(op),
                ));
            }
        }
    }
    return Err(Error::eof(iter.s, iter.pos, TokenKind::Reserved(op), None));
}

pub(crate) fn expect_num(iter: &mut TokenIter) -> Result<u64, Error> {
    if let Some(x) = iter.peek() {
        if let TokenKind::Num(xx) = x.kind {
            iter.next();
            return Ok(xx);
        } else {
            return Err(Error::unexpected_token(
                iter.s,
                x.clone(),
                TokenKind::Num(0),
            ));
        }
    }
    Err(Error::eof(iter.s, iter.pos, TokenKind::Num(0), None))
}

pub(crate) fn expect_semi(iter: &mut TokenIter) -> Result<(), Error> {
    if let Some(x) = iter.peek() {
        if x.kind == TokenKind::SemiColon {
            iter.next();
            return Ok(());
        } else {
            return Err(Error::unexpected_token(
                iter.s,
                x.clone(),
                TokenKind::SemiColon,
            ));
        }
    }
    Err(Error::eof(iter.s, iter.pos, TokenKind::SemiColon, None))
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
                iter.s,
                x.clone(),
                TokenKind::SemiColon,
            ));
        }
    }
    Err(Error::eof(iter.s, iter.pos, TokenKind::SemiColon, None))
}

pub(crate) fn expect_ident(iter: &mut TokenIter) -> Result<Ident, Error> {
    if let Some(x) = iter.peek() {
        if let TokenKind::Ident(id) = x.kind {
            iter.next();
            return Ok(Ident::new(id.name));
        } else {
            return Err(Error::unexpected_token(
                iter.s,
                x.clone(),
                TokenKind::Ident(crate::token::Ident::new("")),
            ));
        }
    }
    Err(Error::eof(
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
                iter.s,
                x.clone(),
                TokenKind::Block(block),
            ));
        }
    }
    Err(Error::eof(iter.s, iter.pos, TokenKind::Block(block), None))
}

pub(crate) fn expect_type_kind(iter: &mut TokenIter) -> Result<base_types::TypeKind, Error> {
    if let Some(x) = iter.peek() {
        if let TokenKind::TypeKind(bt) = x.kind {
            iter.next();
            return Ok(bt);
        } else {
            return Err(Error::unexpected_token(
                iter.s,
                x.clone(),
                TokenKind::TypeKind(base_types::TypeKind::Int),
            ));
        }
    }
    Err(Error::eof(
        iter.s,
        iter.pos,
        TokenKind::TypeKind(base_types::TypeKind::Int),
        None,
    ))
}

pub(crate) fn expect_base_type(iter: &mut TokenIter) -> Result<base_types::BaseType, Error> {
    let kind = expect_type_kind(iter)?;
    let mut btype = BaseType::new(kind);
    loop {
        if consume(iter, Operator::Mul) {
            btype = BaseType::new(TypeKind::Ptr(Rc::new(btype)));
        } else {
            break;
        }
    }
    Ok(btype)
}

/// `**x` returns 2
/// `*&x` returns 0
/// `&&x` returns -2
pub(crate) fn count_deref(node: &Node) -> (usize, Result<Rc<Lvar>, NodeKind>) {
    let mut count = 0;
    let mut ref_node = node;

    loop {
        if ref_node.kind == NodeKind::Deref {
            count += 1;
            ref_node = ref_node.lhs.as_ref().unwrap();
        }
        if ref_node.kind == NodeKind::Addr {
            count -= 1;
            ref_node = ref_node.lhs.as_ref().unwrap();
        } else {
            break;
        }
    }

    if let NodeKind::Lvar(ref lvar) = ref_node.kind {
        return (count, Ok(lvar.clone()));
    }
    (count, Err(ref_node.kind.clone()))
}

/// if global var is already exist, then return error
/// else add global bar to g_var
pub(crate) fn check_g_var(
    iter: &mut TokenIter,
    g_var: &GvarMp,
    b_type: BaseType,
    ident: Ident,
) -> Result<Gvar, Error> {
    match g_var.get(&ident.name) {
        Some(_) => return Err(Error::re_declare(iter.s, ident, iter.pos, None)),
        None => {
            let size = b_type.kind.eight_size();
            let dec = Declaration::new(b_type, ident);
            return Ok(Gvar::new(dec, size));
        }
    }
}
