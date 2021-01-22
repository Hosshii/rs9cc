use super::error::Error;
use crate::token::{self, Token, TokenKind};
use std::{iter::Peekable, rc::Rc, vec::IntoIter};

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

        // single #
        if let Some(x) = iter.peek() {
            if x.is_bol {
                continue;
            }
        }

        if let Some(token) = iter.peek() {
            if let TokenKind::Ident(ident) = &token.kind {
                match ident.name.as_str() {
                    "include" => {
                        iter.next();
                        result.append(&mut include(&mut iter)?)
                    }
                    _ => return Err(Error::invalid_preprocessor(token.clone())),
                }
            }
        }
    }
    Ok(result)
}

fn is_hash(token: &Token) -> bool {
    if let TokenKind::HashMark = &token.kind {
        token.is_bol
    } else {
        false
    }
}

fn include(iter: &mut Peekable<IntoIter<Token>>) -> Result<Vec<Token>, Error> {
    if let Some(x) = iter.next() {
        if let TokenKind::String(filepath) = x.kind {
            let filepath = Rc::new(filepath.trim().to_string());
            if let Some(x) = iter.peek() {
                if !x.is_bol {
                    return Err(Error::invalid_preprocessor(x.clone()));
                }
            }
            return match token::tokenize_file(filepath) {
                Ok(stream) => Ok(stream.tokens),
                Err(e) => Err(Error::todo(e.pos, e.input, e.filepath)),
            };
        }
    }
    todo!();
}
