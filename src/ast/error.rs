use self::ErrorKind::*;
use super::{Ident, Lvar};
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
    InvalidVariableDereference(Lvar, usize),
    InvalidValueDereference(String),
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

    pub fn invalid_variable_dereference(
        input: impl Into<String>,
        pos: TokenPos,
        lvar: Lvar,
        actual_deref_count: usize,
    ) -> Error {
        Error {
            kind: InvalidVariableDereference(lvar, actual_deref_count),
            pos,
            input: input.into(),
            msg: None,
        }
    }

    pub fn invalid_value_dereference(
        input: impl Into<String>,
        pos: TokenPos,
        type_name: impl Into<String>,
    ) -> Error {
        Error {
            kind: InvalidValueDereference(type_name.into()),
            pos,
            input: input.into(),
            msg: None,
        }
    }
}

impl fmt::Display for Error {
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
            InvalidVariableDereference(lvar, actual_deref_count) => {
                invalid_variable_dereference_err_format(&self, lvar, *actual_deref_count, f)
            }
            InvalidValueDereference(type_name) => {
                invalid_value_dereference_err_format(&self, type_name, f)
            }
        }
    }
}

fn err_format(err: &Error, msg: impl Into<String>, f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(f, "{}", err.input)?;
    let result = writeln!(
        f,
        "{number:>width$} {err_msg}",
        number = '^',
        width = err.pos.bytes + 1,
        err_msg = msg.into(),
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

fn invalid_variable_dereference_err_format(
    err: &Error,
    lvar: &Lvar,
    count: usize,
    f: &mut fmt::Formatter,
) -> fmt::Result {
    let msg = format!(
        "invalid pointer dereference
define: {}
actual: {:*<width$}
   ",
        lvar.dec.base_type.kind,
        width = count
    );
    err_format(err, msg, f)
}

fn invalid_value_dereference_err_format(
    err: &Error,
    type_name: impl Into<String>,
    f: &mut fmt::Formatter,
) -> fmt::Result {
    let msg = format!(
        "cannot take the address of an rvalue of type {}",
        type_name.into()
    );
    err_format(err, msg, f)
}
