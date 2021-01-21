use super::error::Error;
use crate::token::{Token, TokenKind};

pub fn preprocessor(tokens: Vec<Token>) -> Result<Vec<Token>, Error> {
    preprocessor_impl(tokens)
}

fn preprocessor_impl(tokens: Vec<Token>) -> Result<Vec<Token>, Error> {
    let mut result = Vec::with_capacity(tokens.len());
    let mut iter = tokens.into_iter().peekable();
    while let Some(token) = iter.next() {
        if !is_hash(&token) {
            result.push(token);
            continue;
        }
        if let Some(x) = iter.peek() {
            if x.is_bol {
                continue;
            } else {
                return Err(Error::invalid_preprocessor(x.clone()));
            }
        }
    }
    Ok(result)
}

pub fn is_hash(token: &Token) -> bool {
    if let TokenKind::Ident(ident) = &token.kind {
        token.is_bol && ident.name.starts_with("#")
    } else {
        false
    }
}
