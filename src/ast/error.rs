use self::ErrorKind::*;
use super::{Ident, Lvar};
use crate::base_types::TypeKind;
use crate::token::{Token, TokenKind, TokenPos};
use std::fmt;
use std::rc::Rc;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum ErrorKind {
    UnexpectedToken {
        expected: TokenKind,
        actual: TokenKind,
    },
    UndefinedVariable(Ident),
    UndefinedFunction(Ident),
    UndefinedMember(Ident),
    UndefinedTag(Ident),
    ReDeclare(Ident),
    InvalidVariableDereference(Lvar, usize),
    InvalidValueDereference(String),
    InvalidAssignment(TypeKind, TypeKind),
    InvalidInitialization(Rc<Lvar>, String),
    InvalidStmtExpr,
    StrayCase,
    EOF(TokenKind),
    Todo,
    Unimplemented,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct Error {
    filepath: Rc<String>,
    kind: ErrorKind,
    pos: TokenPos,
    input: Rc<String>,
    msg: Option<String>,
}

impl Error {
    pub fn unexpected_token(
        filepath: Rc<String>,
        input: Rc<String>,
        token: &Token,
        expected: TokenKind,
    ) -> Error {
        Error {
            filepath,
            kind: UnexpectedToken {
                expected,
                actual: token.kind.clone(),
            },
            pos: token.pos,
            input,
            msg: None,
        }
    }

    pub fn eof(
        filepath: Rc<String>,
        input: Rc<String>,
        pos: TokenPos,
        expected: TokenKind,
        msg: Option<String>,
    ) -> Error {
        Error {
            filepath,
            kind: EOF(expected),
            pos,
            input,
            msg,
        }
    }

    pub fn undefined_variable(
        filepath: Rc<String>,
        input: Rc<String>,
        ident: Ident,
        pos: TokenPos,
        msg: Option<String>,
    ) -> Error {
        Error {
            filepath,
            kind: UndefinedVariable(ident),
            pos,
            input,
            msg,
        }
    }

    pub fn undefined_function(
        filepath: Rc<String>,
        input: Rc<String>,
        ident: Ident,
        pos: TokenPos,
        msg: Option<String>,
    ) -> Error {
        Error {
            filepath,
            kind: UndefinedFunction(ident),
            pos,
            input,
            msg,
        }
    }

    pub fn undefined_member(
        filepath: Rc<String>,
        input: Rc<String>,
        pos: TokenPos,
        ident: Ident,
        msg: Option<String>,
    ) -> Error {
        Error {
            filepath,
            kind: UndefinedMember(ident),
            pos,
            input,
            msg,
        }
    }

    pub fn undefined_tag(
        filepath: Rc<String>,
        input: Rc<String>,
        pos: TokenPos,
        ident: Ident,
        msg: Option<String>,
    ) -> Error {
        Error {
            filepath,
            kind: UndefinedTag(ident),
            pos,
            input,
            msg,
        }
    }

    pub fn re_declare(
        filepath: Rc<String>,
        input: Rc<String>,
        ident: Ident,
        pos: TokenPos,
        msg: Option<String>,
    ) -> Error {
        Error {
            filepath,
            kind: ReDeclare(ident),
            pos,
            input,
            msg,
        }
    }

    pub fn invalid_variable_dereference(
        filepath: Rc<String>,
        input: Rc<String>,
        pos: TokenPos,
        lvar: Lvar,
        actual_deref_count: usize,
    ) -> Error {
        Error {
            filepath,
            kind: InvalidVariableDereference(lvar, actual_deref_count),
            pos,
            input,
            msg: None,
        }
    }

    pub fn invalid_value_dereference(
        filepath: Rc<String>,
        input: Rc<String>,
        pos: TokenPos,
        type_name: String,
    ) -> Error {
        Error {
            filepath,
            kind: InvalidValueDereference(type_name),
            pos,
            input,
            msg: None,
        }
    }

    pub fn invalid_assignment(
        filepath: Rc<String>,
        input: Rc<String>,
        pos: TokenPos,
        lhs_type: TypeKind,
        rhs_type: TypeKind,
    ) -> Error {
        Error {
            filepath,
            kind: InvalidAssignment(lhs_type, rhs_type),
            pos,
            input,
            msg: None,
        }
    }

    pub fn invalid_initialization(
        filepath: Rc<String>,
        input: Rc<String>,
        pos: TokenPos,
        lhs: Rc<Lvar>,
        rhs: String,
    ) -> Error {
        Error {
            filepath,
            kind: InvalidInitialization(lhs, rhs),
            pos,
            input,
            msg: None,
        }
    }

    pub fn invalid_stmt_expr(filepath: Rc<String>, input: Rc<String>, pos: TokenPos) -> Error {
        Error {
            filepath,
            kind: InvalidStmtExpr,
            pos,
            input,
            msg: None,
        }
    }

    pub fn stray_case(filepath: Rc<String>, input: Rc<String>, pos: TokenPos) -> Error {
        Error {
            filepath,
            kind: StrayCase,
            pos,
            input,
            msg: None,
        }
    }

    pub fn todo(filepath: Rc<String>, input: Rc<String>, pos: TokenPos) -> Error {
        Error {
            filepath,
            kind: Todo,
            pos,
            input,
            msg: None,
        }
    }

    pub fn unimplemented(filepath: Rc<String>, input: Rc<String>, pos: TokenPos) -> Error {
        Error {
            filepath,
            kind: Unimplemented,
            pos,
            input,
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
            UndefinedVariable(ident) => undefined_variable_err_format(&self, ident, f),
            UndefinedFunction(ident) => undefined_function_err_format(&self, ident, f),
            UndefinedMember(ident) => undefined_member_err_format(&self, ident, f),
            UndefinedTag(ident) => undefined_tag_err_format(&self, ident, f),
            ReDeclare(ident) => re_declare_err_format(&self, ident, f),
            InvalidVariableDereference(lvar, actual_deref_count) => {
                invalid_variable_dereference_err_format(&self, lvar, *actual_deref_count, f)
            }
            InvalidValueDereference(type_name) => {
                invalid_value_dereference_err_format(&self, type_name.clone(), f)
            }
            InvalidAssignment(lhs_type, rhs_type) => {
                invalid_assignment_err_format(&self, lhs_type, rhs_type, f)
            }
            InvalidInitialization(lhs, rhs) => {
                invalid_initialization_err_format(&self, lhs, rhs, f)
            }
            InvalidStmtExpr => invalid_stmt_expr_err_format(&self, f),
            StrayCase => err_format(&self, "stray case", f),
            Todo => err_format(&self, "todo", f),
            Unimplemented => err_format(&self, "not yet implemented", f),
        }
    }
}

fn err_format(err: &Error, msg: impl Into<String>, f: &mut fmt::Formatter) -> fmt::Result {
    let mut line_num = 1;
    let mut bytes = 0;
    let mut err_input = String::new();
    for line in err.input.lines() {
        let len = line.as_bytes().len();
        if bytes + len >= err.pos.bytes {
            err_input = line.to_string();
            break;
        }
        line_num += 1;
        bytes += len + 1;
    }

    let info = format!("{}: {}", err.filepath, line_num);
    let err_input = format!("{} {}", info, err_input);
    writeln!(f, "{}", err_input)?;
    let result = writeln!(
        f,
        "{number:>width$} {err_msg}",
        number = '^',
        width = err.pos.bytes + 1 + info.len() + 1 - bytes,
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

fn undefined_variable_err_format(
    err: &Error,
    ident: &Ident,
    f: &mut fmt::Formatter,
) -> fmt::Result {
    err_format(err, format!("variable {} is not defined", ident.name), f)
}

fn undefined_function_err_format(
    err: &Error,
    ident: &Ident,
    f: &mut fmt::Formatter,
) -> fmt::Result {
    err_format(err, format!("function {} is not defined", ident.name), f)
}

fn undefined_member_err_format(err: &Error, ident: &Ident, f: &mut fmt::Formatter) -> fmt::Result {
    err_format(
        err,
        format!("struct member {} is not defined", ident.name),
        f,
    )
}

fn undefined_tag_err_format(err: &Error, ident: &Ident, f: &mut fmt::Formatter) -> fmt::Result {
    err_format(
        err,
        format!("the tag named {} is not defined", ident.name),
        f,
    )
}

fn re_declare_err_format(err: &Error, ident: &Ident, f: &mut fmt::Formatter) -> fmt::Result {
    err_format(
        err,
        format!("variable or tag \"{}\" is already defined", ident.name),
        f,
    )
}

fn invalid_variable_dereference_err_format(
    err: &Error,
    lvar: &Lvar,
    count: usize,
    f: &mut fmt::Formatter,
) -> fmt::Result {
    let ptr = format!("{:*<width$}", "*", width = count + 1);
    // writeln!(f, "{} {}", b_type.kind.as_str(), ptr);
    let msg = format!(
        "invalid pointer dereference
define: {}
actual: {}
   ",
        lvar.dec.type_kind, ptr
    );
    err_format(err, msg, f)
}

fn invalid_value_dereference_err_format(
    err: &Error,
    type_name: String,
    f: &mut fmt::Formatter,
) -> fmt::Result {
    let msg = format!("cannot take the address of an rvalue of type {}", type_name);
    err_format(err, msg, f)
}

fn invalid_assignment_err_format(
    err: &Error,
    lhs_type: &TypeKind,
    rhs_type: &TypeKind,
    f: &mut fmt::Formatter,
) -> fmt::Result {
    let msg = format!("invalid assignment. lhs: {}, rhs: {}", lhs_type, rhs_type);
    err_format(err, msg, f)
}

fn invalid_initialization_err_format(
    err: &Error,
    lvar: &Lvar,
    rhs: &String,
    f: &mut fmt::Formatter,
) -> fmt::Result {
    let msg = format!(
        "invalid initialization. lhs: {}, rhs: {}",
        lvar.dec.type_kind, rhs
    );
    err_format(err, msg, f)
}

fn invalid_stmt_expr_err_format(err: &Error, f: &mut fmt::Formatter) -> fmt::Result {
    err_format(err, "stmt expr return void is not supported", f)
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum WarnKind {
    ExcessInitializer,
}

pub struct Warn {
    filepath: Rc<String>,
    kind: WarnKind,
    pos: TokenPos,
    input: Rc<String>,
    msg: Option<String>,
}

impl Warn {
    pub fn excess_initializer(filepath: Rc<String>, input: Rc<String>, pos: TokenPos) {
        let warn = Warn {
            filepath,
            kind: WarnKind::ExcessInitializer,
            pos,
            input,
            msg: None,
        };
        eprintln!("{}", warn);
    }
}

impl fmt::Display for Warn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use WarnKind::*;
        match &self.kind {
            ExcessInitializer => warn_format(&self, "excess elements initializer", f),
        }
    }
}

fn warn_format(err: &Warn, msg: impl Into<String>, f: &mut fmt::Formatter) -> fmt::Result {
    let mut line_num = 1;
    let mut bytes = 0;
    let mut err_input = String::new();
    for line in err.input.lines() {
        let len = line.as_bytes().len();
        if bytes + len >= err.pos.bytes {
            err_input = line.to_string();
            break;
        }
        line_num += 1;
        bytes += len + 1;
    }

    let info = format!("{}: {}", err.filepath, line_num);
    let err_input = format!("{} {}", info, err_input);
    writeln!(f, "{}", err_input)?;
    let result = writeln!(
        f,
        "{number:>width$} {err_msg}",
        number = '^',
        width = err.pos.bytes + 1 + info.len() + 1 - bytes,
        err_msg = msg.into(),
    );
    if let Some(x) = &err.msg {
        writeln!(f, "{}", x)
    } else {
        result
    }
}
