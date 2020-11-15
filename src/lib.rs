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
    pub fn as_char(&self) -> char {
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
    tk_pos: usize,
    bytes_pos: usize,
}

pub fn tokenize<'a>(s: &'a str) -> TokenIter {
    TokenIter {
        s,
        tk_pos: 0,
        bytes_pos: 0,
    }
}

impl Token {
    pub fn expect_num(&self) -> u64 {
        match self.kind {
            Num(n) => n,
            t => panic!("expected number. but got {:?}", t),
        }
    }
}

impl<'a> TokenIter<'a> {
    fn error_at(&self, msg: &str) -> ! {
        eprintln!("{}", self.s);
        eprintln!(
            "{number:>width$} {msg}",
            number = '^',
            width = self.bytes_pos + 1,
            msg = msg
        );
        panic!()
    }

    /// ## warn
    /// you cannot change self.bytes_pos after using this method
    fn cur_str(&self) -> &str {
        let a = self.bytes_pos;
        &self.s[a..]
    }
}

impl<'a> Iterator for TokenIter<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        let sp = calc_space_len(self.cur_str());
        self.bytes_pos += sp;
        let s = self.cur_str();
        if s.is_empty() {
            return None;
        }

        if s.as_bytes()[0] == Plus.as_char() as u8 {
            let tk = Some(Self::Item {
                kind: Reserved(Plus),
                pos: self.tk_pos,
            });
            self.bytes_pos += 1;
            self.tk_pos += 1;
            return tk;
        }

        if s.as_bytes()[0] == Minus.as_char() as u8 {
            let tk = Some(Self::Item {
                kind: Reserved(Minus),
                pos: self.tk_pos,
            });
            self.bytes_pos += 1;
            self.tk_pos += 1;
            return tk;
        }

        let (digit, _, first_non_num_idx) = split_digit(s);
        if !digit.is_empty() {
            let tk = Some(Self::Item {
                kind: Num(u64::from_str_radix(digit, 10).unwrap()),
                pos: self.tk_pos,
            });
            self.bytes_pos += first_non_num_idx;
            self.tk_pos += 1;
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
