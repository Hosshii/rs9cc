use self::ErrorKind::*;
use super::ast::Ident;
use crate::token::{Token, TokenKind, TokenPos};
use std::fmt;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub enum ErrorKind {
    UnexpectedToken {
        expected: TokenKind,
        actual: TokenKind,
    },
    Undefined(Ident),
    ReDeclare(Ident),
    EOF(TokenKind),
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct Error {
    kind: ErrorKind,
    pos: TokenPos,
    input: String,
    msg: Option<String>,
}

impl Error {
    pub fn unexpected_token(input: impl Into<String>, token: Token, expected: TokenKind) -> Error {
        Error {
            kind: UnexpectedToken {
                expected,
                actual: token.kind,
            },
            pos: token.pos,
            input: input.into(),
            msg: None,
        }
    }

    pub fn eof(
        input: impl Into<String>,
        pos: TokenPos,
        expected: TokenKind,
        msg: Option<String>,
    ) -> Error {
        Error {
            kind: EOF(expected),
            pos,
            input: input.into(),
            msg,
        }
    }

    pub fn undefined(
        input: impl Into<String>,
        ident: Ident,
        pos: TokenPos,
        msg: Option<String>,
    ) -> Error {
        Error {
            kind: Undefined(ident),
            pos,
            input: input.into(),
            msg,
        }
    }

    pub fn re_declare(
        input: impl Into<String>,
        ident: Ident,
        pos: TokenPos,
        msg: Option<String>,
    ) -> Error {
        Error {
            kind: ReDeclare(ident),
            pos,
            input: input.into(),
            msg,
        }
    }
}

impl<'a> fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            UnexpectedToken { expected, actual } => {
                unexpected_token_err_format(expected.clone(), actual.clone(), &self, f)
            }
            EOF(expected) => {
                unexpected_token_err_format(expected.clone(), TokenKind::EOF, &self, f)
            }
            Undefined(ident) => undefined_err_format(&self, ident, f),
            ReDeclare(ident) => re_declare_err_format(&self, ident, f),
        }
    }
}

fn err_format(err: &Error, msg: String, f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(f, "{}", err.input)?;
    let result = writeln!(
        f,
        "{number:>width$} {err_msg}",
        number = '^',
        width = err.pos.bytes + 1,
        err_msg = msg,
    );
    if let Some(x) = &err.msg {
        writeln!(f, "{}", x)
    } else {
        result
    }
}

fn unexpected_token_err_format(
    expected: TokenKind,
    actual: TokenKind,
    err: &Error,
    f: &mut fmt::Formatter,
) -> fmt::Result {
    let expected_string = match expected {
        TokenKind::Num(_) => "number".to_string(),
        x => x.as_string(),
    };
    let msg = format!(
        "unexpected token. expected: {}, got: {}",
        expected_string,
        actual.as_string()
    );
    err_format(err, msg, f)
}

fn undefined_err_format(err: &Error, ident: &Ident, f: &mut fmt::Formatter) -> fmt::Result {
    err_format(err, format!("variable {} is not defined", ident.name), f)
}

fn re_declare_err_format(err: &Error, ident: &Ident, f: &mut fmt::Formatter) -> fmt::Result {
    err_format(
        err,
        format!("variable {} is already defined", ident.name),
        f,
    )
}
