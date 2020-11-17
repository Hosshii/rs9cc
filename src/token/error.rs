use self::ErrorKind::*;
use crate::token::TokenPos;
use std::error::Error as StdError;
use std::fmt;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub enum ErrorKind {
    Invalid(String),
    Eof,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub pos: TokenPos,
    pub input: String,
    pub msg: Option<String>,
}

impl StdError for Error {}

impl Error {
    pub(crate) fn invalid(
        input: impl Into<String>,
        s: impl Into<String>,
        pos: TokenPos,
        msg: Option<String>,
    ) -> Error {
        Error {
            kind: Invalid(s.into()),
            pos,
            input: input.into(),
            msg,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            Invalid(x) => {
                writeln!(f, "{}", self.input);
                let result = writeln!(
                    f,
                    "{number:>width$} {err_msg}",
                    number = '^',
                    width = self.pos.bytes + 1,
                    err_msg = format!("invalid token: {}", x)
                );
                if let Some(x) = &self.msg {
                    writeln!(f, "{}", x)
                } else {
                    result
                }
            }
            Eof => writeln!(f, "reached EOF"),
        }
    }
}
