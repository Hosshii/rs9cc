use super::error::{Error, ErrorKind};
use crate::base_types::TypeKind;
use std::ops::{Add, AddAssign};
use std::str::FromStr;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub enum TokenKind {
    Reserved(Operator),
    Ident(Ident),
    KeyWord(KeyWord),
    Block(Block),
    Num(u64),
    TypeKind(TypeKind),
    SemiColon,
    Comma,
    DoubleQuote,
    String(String),
    EOF,
}

impl TokenKind {
    pub fn as_string(&self) -> String {
        use self::TokenKind::*;
        match self {
            Reserved(op) => op.as_str().to_string(),
            Ident(ident) => ident.name.to_string(),
            KeyWord(keyword) => keyword.as_str().to_string(),
            Num(x) => x.to_string(),
            Block(x) => x.as_str().to_string(),
            TypeKind(x) => x.as_str().to_string(),
            SemiColon => ";".to_string(),
            Comma => ",".to_string(),
            DoubleQuote => "\"".to_string(),
            String(s) => s.clone(),
            EOF => "EOF".to_string(),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub enum Operator {
    Assign,
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
    // Asterisk,
    Ampersand,
    Sizeof,
    LArr,
    RArr,
}

impl Operator {
    pub fn as_str(&self) -> &'static str {
        use self::Operator::*;
        match self {
            Assign => "=",
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
            // Asterisk => "*",
            Ampersand => "&",
            Sizeof => "sizeof",
            LArr => "[",
            RArr => "]",
        }
    }

    /// sの最初がOperatorに一致していたらOperatorを返す
    pub fn from_starts(s: &str) -> Result<Operator, ()> {
        let op_lens = vec![6, 2, 1];
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
        use self::Operator::*;
        match s {
            x if x == Equal.as_str() => Ok(Equal),
            x if x == Neq.as_str() => Ok(Neq),
            x if x == Leq.as_str() => Ok(Leq),
            x if x == Geq.as_str() => Ok(Geq),
            x if x == Lesser.as_str() => Ok(Lesser),
            x if x == Greater.as_str() => Ok(Greater),
            x if x == Assign.as_str() => Ok(Assign),
            x if x == Plus.as_str() => Ok(Plus),
            x if x == Minus.as_str() => Ok(Minus),
            x if x == Mul.as_str() => Ok(Mul),
            x if x == Div.as_str() => Ok(Div),
            x if x == LParen.as_str() => Ok(LParen),
            x if x == RParen.as_str() => Ok(RParen),
            x if x == Ampersand.as_str() => Ok(Ampersand),
            x if x == Sizeof.as_str() => Ok(Sizeof),
            x if x == LArr.as_str() => Ok(LArr),
            x if x == RArr.as_str() => Ok(RArr),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct Ident {
    pub name: String,
}

impl Ident {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub enum KeyWord {
    Return,
    If,
    Else,
    While,
    For,
}

impl KeyWord {
    fn as_str(&self) -> &'static str {
        use KeyWord::*;
        match self {
            Return => "return",
            If => "if",
            Else => "else",
            While => "while",
            For => "for",
        }
    }

    fn from_starts(s: &str) -> Result<KeyWord, ()> {
        use self::KeyWord::*;
        match s {
            x if x.starts_with(Return.as_str()) => Ok(Return),
            x if x.starts_with(If.as_str()) => Ok(If),
            x if x.starts_with(Else.as_str()) => Ok(Else),
            x if x.starts_with(While.as_str()) => Ok(While),
            x if x.starts_with(For.as_str()) => Ok(For),
            _ => Err(()),
        }
    }
}

impl FromStr for KeyWord {
    type Err = ();
    fn from_str(s: &str) -> Result<KeyWord, Self::Err> {
        use self::KeyWord::*;
        match s {
            x if x == Return.as_str() => Ok(Return),
            x if x == If.as_str() => Ok(If),
            x if x == Else.as_str() => Ok(Else),
            x if x == While.as_str() => Ok(While),
            x if x == For.as_str() => Ok(For),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub enum Block {
    LParen,
    RParen,
}

impl Block {
    fn as_str(&self) -> &'static str {
        use self::Block::*;
        match self {
            LParen => "{",
            RParen => "}",
        }
    }
}

impl FromStr for Block {
    type Err = ();
    fn from_str(s: &str) -> Result<Block, Self::Err> {
        use self::Block::*;
        match s {
            x if x == LParen.as_str() => Ok(LParen),
            x if x == RParen.as_str() => Ok(RParen),
            _ => Err(()),
        }
    }
}

// impl Iterator for KeyWord {
//     type Item = Self;
//     fn next(&mut self) -> Option<Self::Item> {
//         use self::KeyWord::*;
//         match self {
//             Return => None,
//         }
//     }
// }

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: TokenPos,
}

impl Token {
    pub fn new(kind: TokenKind, pos: TokenPos) -> Self {
        Self { kind, pos }
    }

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

impl TokenPos {
    fn new(tk: usize, bytes: usize) -> Self {
        Self { tk, bytes }
    }

    // TokenPos { tk: 1, bytes }
    fn new_bytes(bytes: usize) -> Self {
        Self { tk: 1, bytes }
    }
}

impl Add for TokenPos {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            bytes: self.bytes + other.bytes,
            tk: self.tk + other.tk,
        }
    }
}

impl AddAssign for TokenPos {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other
    }
}

/// token iterator
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub struct TokenIter<'a> {
    pub s: &'a str,
    pub pos: TokenPos,
    pub filepath: &'a str,
}

pub fn tokenize<'a>(s: &'a str, filepath: &'a str) -> TokenIter<'a> {
    TokenIter {
        s,
        pos: TokenPos { tk: 0, bytes: 0 },
        filepath,
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

        if let Some((tk, _)) = self.is_op(s) {
            return Some(tk);
        }

        if let Some((tk, _)) = self.is_keyword(s) {
            return Some(tk);
        }

        if let Some((tk, _)) = self.is_num(s) {
            return Some(tk);
        }

        if let Some((tk, _)) = self.is_semi(s) {
            return Some(tk);
        }

        if let Some((tk, _)) = self.is_block_paren(s) {
            return Some(tk);
        }

        if let Some((tk, _)) = self.is_comma(s) {
            return Some(tk);
        }

        if let Some((tk, _)) = self.is_string(s) {
            return Some(tk);
        }

        if let Some((tk, _)) = self.is_base_type(s) {
            return Some(tk);
        }

        // これ最後の方がいい
        if let Some((tk, _)) = self.is_ident(s) {
            return Some(tk);
        }

        None
    }

    fn is_op(&self, s: &str) -> Option<(Token, TokenPos)> {
        use self::TokenKind::*;
        if let Ok(op) = Operator::from_starts(s) {
            return Some((
                Token::new(Reserved(op), self.pos),
                TokenPos::new_bytes(op.as_str().len()),
            ));
        }
        None
    }

    fn is_keyword(&self, s: &str) -> Option<(Token, TokenPos)> {
        if let Ok(keyword) = KeyWord::from_starts(s) {
            let len = keyword.as_str().len();
            if !is_alnum(s.chars().nth(len).unwrap_or_else(|| '1')) {
                let kind = TokenKind::KeyWord(keyword);
                return Some((Token::new(kind, self.pos), TokenPos::new_bytes(len)));
            }
        }
        None
    }

    fn is_num(&self, s: &str) -> Option<(Token, TokenPos)> {
        use self::TokenKind::*;
        let (digit, _, bytes) = split_digit(s);
        if !digit.is_empty() {
            return Some((
                Token::new(Num(u64::from_str_radix(digit, 10).unwrap()), self.pos),
                TokenPos::new_bytes(bytes),
            ));
        }
        return None;
    }

    fn is_semi(&self, s: &str) -> Option<(Token, TokenPos)> {
        use self::TokenKind::*;
        let ss = s.chars().nth(0).unwrap();
        if ss.to_string() == SemiColon.as_string() {
            return Some((Token::new(SemiColon, self.pos), TokenPos::new_bytes(1)));
        }
        None
    }

    fn is_ident(&self, s: &str) -> Option<(Token, TokenPos)> {
        let (ident, _, first_non_num_idx) = split_ident(s);
        if !ident.is_empty() {
            return Some((
                Token::new(TokenKind::Ident(Ident::new(ident)), self.pos),
                TokenPos::new_bytes(first_non_num_idx),
            ));
        }
        None
    }

    fn is_block_paren(&self, s: &str) -> Option<(Token, TokenPos)> {
        let ss = s.chars().nth(0).unwrap();
        if let Ok(x) = Block::from_str(&ss.to_string()) {
            return Some((
                Token::new(TokenKind::Block(x), self.pos),
                TokenPos::new(1, 1),
            ));
        }
        None
    }

    fn is_comma(&self, s: &str) -> Option<(Token, TokenPos)> {
        use self::TokenKind::*;
        let ss = s.chars().nth(0).unwrap();
        if ss.to_string() == Comma.as_string() {
            return Some((Token::new(Comma, self.pos), TokenPos::new_bytes(1)));
        }
        None
    }

    fn is_string(&self, s: &str) -> Option<(Token, TokenPos)> {
        use TokenKind::DoubleQuote;
        let mut chars = s.chars();
        if let Some(x) = chars.next() {
            if x.to_string() == DoubleQuote.as_string() {
                let result: String = chars.take_while(|c| c != &'\"').collect();
                let len = result.len();
                if len >= s.len() - 1 {
                    self.error_at("cannot find end of \"");
                }

                return Some((
                    Token::new(TokenKind::String(result), self.pos),
                    TokenPos::new_bytes(len + 2),
                ));
            }
        }
        None
    }

    fn is_base_type(&self, s: &str) -> Option<(Token, TokenPos)> {
        if let Ok(base_type) = TypeKind::from_starts(s) {
            let len = base_type.as_str().len();
            // specify '1' in unwrap_of_else because !is_alnum('1') is true at anytime
            if !is_alnum(s.chars().nth(len).unwrap_or_else(|| '1')) {
                return Some((
                    Token::new(TokenKind::TypeKind(base_type), self.pos),
                    TokenPos::new_bytes(len),
                ));
            }
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

        if let Some((tk, pos)) = self.is_op(s) {
            self.pos += pos;
            return Some(tk);
        }

        if let Some((tk, pos)) = self.is_keyword(s) {
            self.pos += pos;
            return Some(tk);
        }

        if let Some((tk, pos)) = self.is_num(s) {
            self.pos += pos;
            return Some(tk);
        }

        if let Some((tk, pos)) = self.is_semi(s) {
            self.pos += pos;
            return Some(tk);
        }

        if let Some((tk, pos)) = self.is_block_paren(s) {
            self.pos += pos;
            return Some(tk);
        }

        if let Some((tk, pos)) = self.is_comma(s) {
            self.pos += pos;
            return Some(tk);
        }

        if let Some((tk, pos)) = self.is_string(s) {
            self.pos += pos;
            return Some(tk);
        }

        if let Some((tk, pos)) = self.is_base_type(s) {
            self.pos += pos;
            return Some(tk);
        }

        // これ最後の方がいい
        // 最後にしないと予約後が変数として扱われちゃう
        if let Some((tk, pos)) = self.is_ident(s) {
            self.pos += pos;
            return Some(tk);
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

// fooo=1 のfoooをidentとして返す
fn split_ident(s: &str) -> (&str, &str, usize) {
    let mut first_non_ident_idx = 0;
    for (i, _) in s.char_indices() {
        if let Ok(_) = Operator::from_starts(&s[i..]) {
            break;
        }
        if s[i..].starts_with(&TokenKind::SemiColon.as_string())
            || s[i..].starts_with(" ")
            || s[i..].starts_with(&TokenKind::Comma.as_string())
            || s[i..].starts_with(Block::LParen.as_str())
            || s[i..].starts_with(Block::RParen.as_str())
        {
            break;
        }
        first_non_ident_idx += 1;
    }
    let (f, s) = s.split_at(first_non_ident_idx);
    (f, s, first_non_ident_idx)
}

/// 入力の先頭から空白がなくなるところを探して
/// 最初に出てくる空白ではない文字の位置を返す
fn calc_space_len(s: &str) -> usize {
    let mut begin = s.char_indices().peekable();

    while let Some((pos, chars)) = begin.next() {
        if !chars.is_whitespace() {
            return pos;
        }
        if begin.peek() == None {
            return pos + 1;
        }
    }
    0
}

fn is_alnum(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_iter() {
        use self::KeyWord::*;
        use self::Operator::*;
        use self::TokenKind::{KeyWord, Num, Reserved, SemiColon};
        let input = "== != = < <= > >= + - * / ( ) & sizeof [ ]";
        let expected = vec![
            Equal, Neq, Assign, Lesser, Leq, Greater, Geq, Plus, Minus, Mul, Div, LParen, RParen,
            Ampersand, Sizeof, LArr, RArr,
        ];
        let mut iter = tokenize(input, "");
        for i in expected {
            assert_eq!(Reserved(i), iter.next().unwrap().kind);
        }
        assert_eq!(None, iter.next());

        let input = "foo=1;bar=20;";
        let expected = vec![
            TokenKind::Ident(Ident::new("foo")),
            Reserved(Assign),
            Num(1),
            SemiColon,
            TokenKind::Ident(Ident::new("bar")),
            Reserved(Assign),
            Num(20),
            SemiColon,
        ];
        let mut iter = tokenize(input, "");
        for i in expected {
            assert_eq!(i, iter.next().unwrap().kind);
        }
        assert_eq!(None, iter.next());

        let input =
            "return; returnx return1 return 1 for while if else force whilet ifelse elseif \"aaaaa\"a";
        let expected = vec![
            TokenKind::KeyWord(Return),
            SemiColon,
            TokenKind::Ident(Ident::new("returnx")),
            TokenKind::Ident(Ident::new("return1")),
            KeyWord(Return),
            Num(1),
            KeyWord(For),
            KeyWord(While),
            KeyWord(If),
            KeyWord(Else),
            TokenKind::Ident(Ident::new("force")),
            TokenKind::Ident(Ident::new("whilet")),
            TokenKind::Ident(Ident::new("ifelse")),
            TokenKind::Ident(Ident::new("elseif")),
            TokenKind::String("aaaaa".to_string()),
            TokenKind::Ident(Ident::new("a")),
        ];
        let mut iter = tokenize(input, "");
        for i in expected {
            assert_eq!(i, iter.next().unwrap().kind);
        }
        assert_eq!(None, iter.next());

        let input = "{ { } ,hoge, int int1 char char1";
        let expected = vec![
            TokenKind::Block(Block::LParen),
            TokenKind::Block(Block::LParen),
            TokenKind::Block(Block::RParen),
            TokenKind::Comma,
            TokenKind::Ident(Ident::new("hoge")),
            TokenKind::Comma,
            TokenKind::TypeKind(TypeKind::Int),
            TokenKind::Ident(Ident::new("int1")),
            TokenKind::TypeKind(TypeKind::Char),
            TokenKind::Ident(Ident::new("char1")),
        ];
        let mut iter = tokenize(input, "");
        for i in expected {
            assert_eq!(i, iter.next().unwrap().kind);
        }
        assert_eq!(None, iter.next());
    }

    #[test]
    fn test_operator_from_starts() {
        use Operator::*;
        let tests = [
            ("==", Ok(Equal)),
            ("===", Ok(Equal)),
            ("!=", Ok(Neq)),
            ("!==", Ok(Neq)),
            ("<", Ok(Lesser)),
            ("<1", Ok(Lesser)),
            ("<=", Ok(Leq)),
            ("<==", Ok(Leq)),
            (">", Ok(Greater)),
            (">>", Ok(Greater)),
            (">=", Ok(Geq)),
            (">=>", Ok(Geq)),
            ("+", Ok(Plus)),
            ("++", Ok(Plus)),
            ("-", Ok(Minus)),
            ("-=", Ok(Minus)),
            ("*", Ok(Mul)),
            ("*=", Ok(Mul)),
            ("/", Ok(Div)),
            ("//", Ok(Div)),
            ("(", Ok(LParen)),
            ("(=", Ok(LParen)),
            (")", Ok(RParen)),
            ("))", Ok(RParen)),
            ("&", Ok(Ampersand)),
            ("sizeof", Ok(Sizeof)),
            ("foo", Err(())),
        ];
        for &(s, ref expected) in &tests {
            assert_eq!(expected, &Operator::from_starts(s));
        }
    }

    #[test]
    fn test_keyword_from_starts() {
        use self::KeyWord::*;
        let tests = [("return", Ok(Return)), ("noreturn", Err(()))];

        for (s, expected) in &tests {
            assert_eq!(expected, &KeyWord::from_starts(s));
        }
    }

    #[test]
    fn test_calc_space_len() {
        let tests = [("    a", 4), ("a", 0)];
        for (s, expected) in &tests {
            assert_eq!(expected, &calc_space_len(s));
        }
    }

    #[test]
    fn test_split_digit() {
        let tests = [("fff", ("", "fff", 0)), ("11fff", ("11", "fff", 2))];
        for (s, expected) in &tests {
            assert_eq!(expected, &split_digit(s));
        }
    }

    #[test]
    fn test_split_ident() {
        let tests = [
            ("foo1=", ("foo1", "=", 4)),
            ("=foo", ("", "=foo", 0)),
            ("bar;", ("bar", ";", 3)),
            ("a =1;", ("a", " =1;", 1)),
        ];
        for (s, expected) in &tests {
            let (f, s, n) = split_ident(s);
            assert_eq!(expected.0, f);
            assert_eq!(expected.1, s);
            assert_eq!(expected.2, n);
        }
    }
}
