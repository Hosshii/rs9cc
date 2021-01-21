use self::ErrorKind::*;
use crate::token::{Token, TokenPos};
use std::fmt;
use std::rc::Rc;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum ErrorKind {
    InvalidPreprocessor(Token),
    Todo,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct Error {
    kind: ErrorKind,
    pub pos: TokenPos,
    pub input: Rc<String>,
    pub filepath: Rc<String>,
    msg: Option<String>,
}

impl Error {
    pub fn invalid_preprocessor(tk: Token) -> Self {
        Self {
            pos: tk.pos,
            input: tk.input.clone(),
            filepath: tk.filepath.clone(),
            kind: InvalidPreprocessor(tk),
            msg: None,
        }
    }

    pub fn todo(pos: TokenPos, input: Rc<String>, filepath: Rc<String>) -> Self {
        Self {
            kind: Todo,
            pos,
            input,
            filepath,
            msg: None,
        }
    }
}

impl<'a> fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            InvalidPreprocessor(_) => err_format(self, f),
            Todo => err_format(self, f),
        }
    }
}

fn err_format(err: &Error, f: &mut fmt::Formatter) -> fmt::Result {
    match &err.kind {
        InvalidPreprocessor(_) => writeln!(f, "invalid preprocessor"),
        Todo => writeln!(f, "todo"),
    }
}
