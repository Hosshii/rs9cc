use self::ErrorKind::*;
use crate::token::TokenPos;
use std::error::Error as StdError;
use std::fmt;
use std::rc::Rc;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub enum ErrorKind {
    Invalid(String),
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
            Invalid(x) => {
                let mut line_num = 1;
                let mut bytes = 0;
                let mut err_input = String::new();
                for line in self.input.lines() {
                    let len = line.as_bytes().len();
                    if bytes + len >= self.pos.bytes {
                        err_input = line.to_string();
                        break;
                    }
                    line_num += 1;
                    bytes += len + 1;
                }

                let info = format!("{}: {}", self.filepath, line_num);
                let err_input = format!("{} {}", info, err_input);
                writeln!(f, "{}", err_input)?;
                let result = writeln!(
                    f,
                    "{number:>width$} {err_msg}",
                    number = '^',
                    width = self.pos.bytes + 1 + info.len() + 1 - bytes,
                    err_msg = x,
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
