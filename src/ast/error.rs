use self::ErrorKind::*;
use crate::token::{Token, TokenKind, TokenPos};
use std::fmt;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub enum ErrorKind {
    UnexpectedToken {
        expected: TokenKind,
        actual: TokenKind,
    },
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
    // pub fn unexpected_token(
    //     input: &'a str,
    //     pos: TokenPos,
    //     expected: TokenKind,
    //     actual: TokenKind,
    //     msg: Option<&'a str>,
    // ) -> Error<'a> {
    //     Error {
    //         kind: UnexpectedToken { expected, actual },
    //         pos,
    //         input,
    //         msg,
    //     }
    // }

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
}

impl<'a> fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            UnexpectedToken { expected, actual } => {
                // let expected_string = match expected {
                //     TokenKind::Num(_) => "number".to_string(),
                //     x => x.as_string(),
                // };
                // writeln!(f, "{}", self.input)?;
                // let result = writeln!(
                //     f,
                //     "{number:>width$} {err_msg}",
                //     number = '^',
                //     width = self.pos.bytes + 1,
                //     err_msg = format!(
                //         "unexpected token. expected: {}, got: {}",
                //         expected_string,
                //         actual.as_string()
                //     )
                // );
                // if let Some(x) = &self.msg {
                //     writeln!(f, "{}", x)
                // } else {
                //     result
                // }
                err_format(expected, actual, &self, f)
            }
            EOF(expected) => err_format(expected, TokenKind::EOF, &self, f),
        }
    }
}

fn err_format(
    expected: TokenKind,
    actual: TokenKind,
    err: &Error,
    f: &mut fmt::Formatter,
) -> fmt::Result {
    let expected_string = match expected {
        TokenKind::Num(_) => "number".to_string(),
        x => x.as_string(),
    };
    writeln!(f, "{}", err.input)?;
    let result = writeln!(
        f,
        "{number:>width$} {err_msg}",
        number = '^',
        width = err.pos.bytes + 1,
        err_msg = format!(
            "unexpected token. expected: {}, got: {}",
            expected_string,
            actual.as_string()
        )
    );
    if let Some(x) = &err.msg {
        writeln!(f, "{}", x)
    } else {
        result
    }
}
