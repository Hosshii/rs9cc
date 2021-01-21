use self::ErrorKind::*;
use crate::token::TokenPos;
use crate::util;
use std::error::Error as StdError;
use std::fmt;
use std::rc::Rc;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub enum ErrorKind {
    Invalid(String),
    Preprocessor(String),
    Eof,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub pos: TokenPos,
    pub input: Rc<String>,
    pub filepath: Rc<String>,
    pub msg: Option<String>,
}

impl StdError for Error {}

impl Error {
    pub(crate) fn invalid(
        filepath: Rc<String>,
        input: Rc<String>,
        pos: TokenPos,
        msg: Option<String>,
    ) -> Error {
        Error {
            kind: Invalid(msg.unwrap_or("cannot tokenize".to_string())),
            pos,
            input,
            filepath,
            msg: None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            Invalid(x) | Preprocessor(x) => {
                let err = util::err_format(self.input.clone(), self.filepath.clone(), self.pos, x)?;
                writeln!(f, "{}", err)
            }

            Eof => writeln!(f, "reached EOF"),
        }
    }
}

impl From<crate::preprocessor::Error> for Error {
    fn from(from: crate::preprocessor::Error) -> Self {
        let err = format!("{}", from);
        Self {
            kind: Preprocessor(err),
            filepath: from.filepath,
            input: from.input,
            pos: from.pos,
            msg: None,
        }
    }
}
