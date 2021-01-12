use self::ErrorKind::*;
use std::fmt;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub enum ErrorKind {
    NoLVar,
    NoGvar,
    NotFound,
    StrayBreak,
    WriteError(String),
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct Error {
    kind: ErrorKind,
    msg: Option<String>,
}

impl Error {
    // pub fn not_lvar(input: impl Into<String>, token: Token) -> Error {
    pub fn not_lvar() -> Self {
        Self {
            kind: NoLVar,
            msg: None,
        }
    }
    pub fn not_gvar() -> Self {
        Self {
            kind: NoGvar,
            msg: None,
        }
    }
    pub fn not_found() -> Self {
        Self {
            kind: NotFound,
            msg: None,
        }
    }

    pub fn stray_break() -> Self {
        Self {
            kind: StrayBreak,
            msg: None,
        }
    }
}

impl<'a> fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            NoLVar => err_format(self, f),
            NoGvar => err_format(self, f),
            NotFound => err_format(self, f),
            StrayBreak => err_format(self, f),
            WriteError(_) => err_format(self, f),
        }
    }
}

fn err_format(err: &Error, f: &mut fmt::Formatter) -> fmt::Result {
    match &err.kind {
        NoLVar => writeln!(f, "Left Value is not substitutable"),
        NoGvar => writeln!(f, "not global value"),
        NotFound => writeln!(f, "Node not found"),
        StrayBreak => writeln!(f, "stray break"),
        WriteError(string) => writeln!(f, "{}", string),
    }
}

impl From<std::fmt::Error> for Error {
    fn from(error: std::fmt::Error) -> Self {
        Self {
            kind: WriteError(format!("{}", error)),
            msg: None,
        }
    }
}
