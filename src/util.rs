use std::fmt::{Error, Write};
use std::rc::Rc;

use crate::token::TokenPos;

pub fn err_format(
    input: Rc<String>,
    filepath: Rc<String>,
    pos: TokenPos,
    msg: impl Into<String>,
) -> Result<String, Error> {
    let mut line_num = 1;
    let mut bytes = 0;
    let mut err_input = String::new();
    let mut buf = String::new();
    for line in input.lines() {
        let len = line.as_bytes().len();
        if bytes + len >= pos.bytes {
            err_input = line.to_string();
            break;
        }
        line_num += 1;
        bytes += len + 1;
    }

    let info = format!("{}: {}", filepath, line_num);
    let err_input = format!("{} {}", info, err_input);
    writeln!(buf, "{}", err_input)?;
    writeln!(
        buf,
        "{number:>width$} {err_msg}",
        number = '^',
        width = pos.bytes + 1 + info.len() + 1 - bytes,
        err_msg = msg.into(),
    )?;
    Ok(buf)
}
