use self::Operator::*;
use self::TokenKind::*;
use super::error::{Error, ErrorKind};
use std::str::FromStr;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub enum TokenKind {
    Reserved(Operator),
    Num(u64),
    TkEOF,
}

impl TokenKind {
    pub fn as_string(&self) -> String {
        match self {
            Reserved(op) => op.as_str().to_string(),
            Num(x) => x.to_string(),
            TkEOF => "EOF".to_string(),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub enum Operator {
    Equal,
    Neq,
    Lesser,
    Leq,
    Greater,
    Geq,
    Plus,
    Minus,
    Mul,
    Div,
    LParen,
    RParen,
}

impl Operator {
    pub fn as_str(&self) -> &'static str {
        match self {
            Equal => "==",
            Neq => "!=",
            Lesser => "<",
            Leq => "<=",
            Greater => ">",
            Geq => ">=",
            Plus => "+",
            Minus => "-",
            Mul => "*",
            Div => "/",
            LParen => "(",
            RParen => ")",
        }
    }

    /// sの最初がOperatorに一致していたらOperatorを返す
    pub fn from_starts(s: &str) -> Result<Operator, ()> {
        let op_lens = vec![2, 1];
        for idx in op_lens {
            let x = s.chars().take(idx).collect::<String>();
            if let Ok(x) = Self::from_str(&s[..x.len()]) {
                return Ok(x);
            }
        }
        Err(())
    }
}

impl FromStr for Operator {
    type Err = ();
    fn from_str(s: &str) -> Result<Operator, Self::Err> {
        match s {
            x if x == Equal.as_str() => Ok(Equal),
            x if x == Neq.as_str() => Ok(Neq),
            x if x == Leq.as_str() => Ok(Leq),
            x if x == Geq.as_str() => Ok(Geq),
            x if x == Lesser.as_str() => Ok(Lesser),
            x if x == Greater.as_str() => Ok(Greater),
            x if x == Plus.as_str() => Ok(Plus),
            x if x == Minus.as_str() => Ok(Minus),
            x if x == Mul.as_str() => Ok(Mul),
            x if x == Div.as_str() => Ok(Div),
            x if x == LParen.as_str() => Ok(LParen),
            x if x == RParen.as_str() => Ok(RParen),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: TokenPos,
    pub len: usize,
}

impl Token {
    pub fn new_error(
        &self,
        input: impl Into<String>,
        s: impl Into<String>,
        msg: impl Into<String>,
    ) -> Error {
        Error {
            input: input.into(),
            kind: ErrorKind::Invalid(s.into()),
            pos: self.pos,
            msg: Some(msg.into()),
        }
    }
}

/// トークンの場所を保持する
/// エラーとかの表示のため
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub struct TokenPos {
    // tokenがイテレータのどこにあるか
    pub tk: usize,

    // 文字列のどこに位置しているか
    pub bytes: usize,
}

/// token iterator
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub struct TokenIter<'a> {
    pub s: &'a str,
    pub pos: TokenPos,
}

pub fn tokenize<'a>(s: &'a str) -> TokenIter {
    TokenIter {
        s,
        pos: TokenPos { tk: 0, bytes: 0 },
    }
}

impl<'a> TokenIter<'a> {
    /// std::iter::Peekable.peek()に似てるけど、nextを内部で呼ばない
    pub fn peek(&self) -> Option<Token> {
        let sp = calc_space_len(self.cur_str());
        let s = &self.s[self.pos.bytes + sp..];
        if s.is_empty() {
            return None;
        }

        if let Ok(op) = Operator::from_starts(s) {
            let tk = Some(Token {
                kind: Reserved(op),
                pos: self.pos,
                len: op.as_str().chars().count(),
            });
            return tk;
        }

        let (digit, _, first_non_num_idx) = split_digit(s);
        if !digit.is_empty() {
            let tk = Some(Token {
                kind: Num(u64::from_str_radix(digit, 10).unwrap()),
                pos: self.pos,
                len: digit.chars().count(),
            });
            return tk;
        }
        None
    }

    fn error_at(&self, msg: &str) -> ! {
        eprintln!("{}", self.s);
        eprintln!(
            "{number:>width$} {msg}",
            number = '^',
            width = self.pos.bytes + 1,
            msg = msg
        );
        panic!()
    }

    /// ## warn
    /// you cannot change self.bytes_pos after using this method
    fn cur_str(&self) -> &str {
        let a = self.pos.bytes;
        &self.s[a..]
    }
}

impl<'a> Iterator for TokenIter<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        let sp = calc_space_len(self.cur_str());
        self.pos.bytes += sp;
        let s = self.cur_str();
        if s.is_empty() {
            return None;
        }

        if let Ok(op) = Operator::from_starts(s) {
            let tk = Some(Self::Item {
                kind: Reserved(op),
                pos: self.pos,
                len: op.as_str().chars().count(),
            });
            self.pos.bytes += 1;
            self.pos.tk += 1;
            return tk;
        }

        let (digit, _, first_non_num_idx) = split_digit(s);
        if !digit.is_empty() {
            let tk = Some(Self::Item {
                kind: Num(u64::from_str_radix(digit, 10).unwrap()),
                pos: self.pos,
                len: digit.chars().count(),
            });
            self.pos.bytes += first_non_num_idx;
            self.pos.tk += 1;
            return tk;
        }

        self.error_at("トークナイズできません")
    }
}

/// Rustにstrtol関数がないので同じような挙動をする関数を定義する。
fn split_digit(s: &str) -> (&str, &str, usize) {
    let first_non_num_idx = s.find(|c| !char::is_numeric(c)).unwrap_or(s.len());
    let (f, s) = s.split_at(first_non_num_idx);
    (f, s, first_non_num_idx)
}

/// 入力の先頭から空白がなくなるところを探して
/// 最初に出てくる空白ではない文字の位置を返す
fn calc_space_len(s: &str) -> usize {
    let mut begin = s.char_indices().peekable();

    while let Some((pos, chars)) = begin.next() {
        if chars != ' ' {
            return pos;
        }
        if begin.peek() == None {
            return pos + 1;
        }
    }
    0
}
