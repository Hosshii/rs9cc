use self::Operator::*;
use self::TokenKind::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub enum TokenKind {
    Reserved(Operator),
    Num(u64),
    TkEOF,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub enum Operator {
    Plus,
    Minus,
}

impl Operator {
    pub fn as_str(&self) -> char {
        match self {
            Plus => '+',
            Minus => '-',
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: usize,
}

/// token iterator
pub struct TokenIter<'a> {
    s: &'a str,
    pos: usize,
}

pub fn tokenize<'a>(s: &'a str) -> TokenIter {
    TokenIter { s, pos: 0 }
}

impl Token {
    pub fn expect_num(&self) -> u64 {
        match self.kind {
            Num(n) => n,
            t => panic!("expected number. but got {:?}", t),
        }
    }
}

impl<'a> Iterator for TokenIter<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        self.s = self.s.trim_start();

        if self.s.is_empty() {
            return None;
        }

        if self.s.as_bytes()[0] == Plus.as_str() as u8 {
            self.s = self.s.split_at(1).1;
            let tk = Some(Self::Item {
                kind: Reserved(Plus),
                pos: self.pos,
            });
            self.pos += 1;
            return tk;
        }

        if self.s.as_bytes()[0] == Minus.as_str() as u8 {
            self.s = self.s.split_at(1).1;
            let tk = Some(Self::Item {
                kind: Reserved(Minus),
                pos: self.pos,
            });
            self.pos += 1;
            return tk;
        }

        let (digit, remain) = split_digit(self.s);
        if !digit.is_empty() {
            self.s = remain;
            let tk = Some(Self::Item {
                kind: Num(u64::from_str_radix(digit, 10).unwrap()),
                pos: self.pos,
            });
            self.pos += 1;
            return tk;
        }

        None
    }
}

/// Rustにstrtol関数がないので同じような挙動をする関数を定義する。
fn split_digit(s: &str) -> (&str, &str) {
    let first_non_num_idx = s.find(|c| !char::is_numeric(c)).unwrap_or(s.len());
    s.split_at(first_non_num_idx)
}
