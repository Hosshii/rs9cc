use super::error::Error;
use crate::base_types::TypeKind;
use crate::preprocessor;
use std::fs;
use std::ops::{Add, AddAssign};
use std::rc::Rc;
use std::str::FromStr;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum TokenKind {
    Reserved(Operator),
    Ident(Ident),
    KeyWord(KeyWord),
    Block(Block),
    Num(i64),
    TypeKind(TypeKind),
    Comment(Comment),
    SemiColon,
    Colon,
    Comma,
    DoubleQuote,
    SingleQuote,
    Period,
    Question,
    String(String),
    Char(char),
    HashMark,
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
            Comment(x) => x.as_str().to_string(),
            SemiColon => ";".to_string(),
            Colon => ":".to_string(),
            Comma => ",".to_string(),
            DoubleQuote => "\"".to_string(),
            SingleQuote => "'".to_string(),
            Period => ".".to_string(),
            Question => "?".to_string(),
            String(s) => s.clone(),
            Char(c) => c.to_string(),
            HashMark => "#".to_string(),
            EOF => "EOF".to_string(),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub enum Operator {
    Assign,
    APlus,
    AMinus,
    AMul,
    ADiv,
    ALShift,
    ARShift,
    ABitAnd,
    ABitOr,
    ABitXor,
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
    Arrow,
    PlusPlus,
    MinusMinus,
    Not,
    BitNot,
    BitOr,
    BitXor,
    LogOr,
    LogAnd,
    RShift,
    LShift,
    ThreeDots,
}

impl Operator {
    pub fn as_str(&self) -> &'static str {
        use self::Operator::*;
        match self {
            Assign => "=",
            APlus => "+=",
            AMinus => "-=",
            AMul => "*=",
            ADiv => "/=",
            ALShift => "<<=",
            ARShift => ">>=",
            ABitAnd => "&=",
            ABitOr => "|=",
            ABitXor => "^=",
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
            Arrow => "->",
            PlusPlus => "++",
            MinusMinus => "--",
            Not => "!",
            BitNot => "~",
            BitOr => "|",
            BitXor => "^",
            LogOr => "||",
            LogAnd => "&&",
            RShift => ">>",
            LShift => "<<",
            ThreeDots => "...",
        }
    }

    /// sの最初がOperatorに一致していたらOperatorを返す
    pub fn from_starts(s: &str) -> Result<Operator, ()> {
        let op_lens = vec![6, 3, 2, 1];
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
            x if x == APlus.as_str() => Ok(APlus),
            x if x == AMinus.as_str() => Ok(AMinus),
            x if x == AMul.as_str() => Ok(AMul),
            x if x == ADiv.as_str() => Ok(ADiv),
            x if x == ALShift.as_str() => Ok(ALShift),
            x if x == ARShift.as_str() => Ok(ARShift),
            x if x == ABitAnd.as_str() => Ok(ABitAnd),
            x if x == ABitOr.as_str() => Ok(ABitOr),
            x if x == ABitXor.as_str() => Ok(ABitXor),
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
            x if x == Arrow.as_str() => Ok(Arrow),
            x if x == PlusPlus.as_str() => Ok(PlusPlus),
            x if x == MinusMinus.as_str() => Ok(MinusMinus),
            x if x == Not.as_str() => Ok(Not),
            x if x == BitNot.as_str() => Ok(BitNot),
            x if x == BitOr.as_str() => Ok(BitOr),
            x if x == BitXor.as_str() => Ok(BitXor),
            x if x == LogOr.as_str() => Ok(LogOr),
            x if x == LogAnd.as_str() => Ok(LogAnd),
            x if x == LShift.as_str() => Ok(LShift),
            x if x == RShift.as_str() => Ok(RShift),
            x if x == ThreeDots.as_str() => Ok(ThreeDots),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct Ident {
    pub name: String,
}

impl From<crate::ast::Ident> for Ident {
    fn from(ident: crate::ast::Ident) -> Self {
        Ident::new(ident.name)
    }
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
    Struct,
    Enum,
    Typedef,
    Static,
    Break,
    Continue,
    Goto,
    Switch,
    Case,
    Default,
    Extern,
    Do,
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
            Struct => "struct",
            Enum => "enum",
            Typedef => "typedef",
            Static => "static",
            Break => "break",
            Continue => "continue",
            Goto => "goto",
            Switch => "switch",
            Case => "case",
            Default => "default",
            Extern => "extern",
            Do => "do",
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
            x if x.starts_with(Struct.as_str()) => Ok(Struct),
            x if x.starts_with(Enum.as_str()) => Ok(Enum),
            x if x.starts_with(Typedef.as_str()) => Ok(Typedef),
            x if x.starts_with(Static.as_str()) => Ok(Static),
            x if x.starts_with(Break.as_str()) => Ok(Break),
            x if x.starts_with(Continue.as_str()) => Ok(Continue),
            x if x.starts_with(Goto.as_str()) => Ok(Goto),
            x if x.starts_with(Switch.as_str()) => Ok(Switch),
            x if x.starts_with(Case.as_str()) => Ok(Case),
            x if x.starts_with(Default.as_str()) => Ok(Default),
            x if x.starts_with(Extern.as_str()) => Ok(Extern),
            x if x.starts_with(Do.as_str()) => Ok(Do),
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
            x if x == Struct.as_str() => Ok(Struct),
            x if x == Enum.as_str() => Ok(Enum),
            x if x == Typedef.as_str() => Ok(Typedef),
            x if x == Static.as_str() => Ok(Static),
            x if x == Break.as_str() => Ok(Break),
            x if x == Continue.as_str() => Ok(Continue),
            x if x == Goto.as_str() => Ok(Goto),
            x if x == Switch.as_str() => Ok(Switch),
            x if x == Case.as_str() => Ok(Case),
            x if x == Default.as_str() => Ok(Default),
            x if x == Extern.as_str() => Ok(Extern),
            x if x == Do.as_str() => Ok(Do),
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub enum Comment {
    Single,
    MultiStart,
    MultiEnd,
}
impl Comment {
    fn as_str(&self) -> &'static str {
        use self::Comment::*;
        match self {
            Single => "//",
            MultiStart => "/*",
            MultiEnd => "*/",
        }
    }
}

impl FromStr for Comment {
    type Err = ();
    fn from_str(s: &str) -> Result<Comment, Self::Err> {
        use self::Comment::*;
        match s {
            x if x == Single.as_str() => Ok(Single),
            x if x == MultiStart.as_str() => Ok(MultiStart),
            x if x == MultiEnd.as_str() => Ok(MultiEnd),
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
    pub input: Rc<String>,
    pub filepath: Rc<String>,
    pub kind: TokenKind,
    pub pos: TokenPos,
    pub prev_pos: TokenPos,
    pub is_bol: bool,
}

impl Token {
    pub fn new(
        kind: TokenKind,
        pos: TokenPos,
        prev_pos: TokenPos,
        input: Rc<String>,
        filepath: Rc<String>,
        is_bol: bool,
    ) -> Self {
        Self {
            input,
            filepath,
            kind,
            pos,
            prev_pos,
            is_bol,
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct TokenStream {
    pub input: Rc<String>,
    pub filepath: Rc<String>,
    pub pos: TokenPos,
    pub idx: usize,
    pub tokens: Vec<Token>,
}

impl TokenStream {
    pub fn new(input: Rc<String>, filepath: Rc<String>, tokens: Vec<Token>) -> Self {
        Self {
            input,
            filepath,
            pos: TokenPos::new(0, 0),
            idx: 0,
            tokens,
        }
    }

    pub fn next(&mut self) -> Option<Token> {
        let result = self.tokens.get(self.idx);
        self.idx += 1;

        match result {
            Some(v) => {
                self.pos = v.pos;
                Some(v.clone())
            }
            None => None,
        }
    }

    pub fn peek(&mut self) -> Option<Token> {
        self.tokens.get(self.idx).and_then(|v| Some(v.clone()))
    }

    pub fn prev(&mut self) -> Option<Token> {
        if self.idx as isize - 1 >= 0 {
            let mut pos = self.pos;
            let result = self.tokens.get(self.idx - 1).and_then(|v| {
                pos = v.pos;
                Some(v.clone())
            });
            self.pos = pos;
            self.idx -= 1;
            result
        } else {
            None
        }
    }

    pub fn save(&self) -> (usize, TokenPos) {
        (self.idx, self.pos)
    }

    pub fn restore(&mut self, data: (usize, TokenPos)) {
        self.idx = data.0;
        self.pos = data.1;
    }
}

/// token iterator
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct TokenIter {
    pub input: Rc<String>,
    pub pos: TokenPos,
    pub prev_pos: TokenPos,
    pub filepath: Rc<String>,
}

pub fn tokenize(input: Rc<String>, filepath: Rc<String>) -> Result<TokenStream, Error> {
    let mut vec = Vec::new();
    let mut token_iter = TokenIter::new(input.clone(), filepath.clone());
    if let Some(mut x) = token_iter.next()? {
        x.is_bol = true;
        vec.push(x);
    }
    while let Some(x) = token_iter.next()? {
        vec.push(x);
    }

    vec = preprocessor::preprocessor(vec)?;

    Ok(TokenStream::new(input, filepath, vec))
}

pub fn tokenize_file(filepath: Rc<String>) -> Result<TokenStream, Error> {
    let mut content =
        fs::read_to_string(filepath.as_ref()).expect(&format!("{} is not exist", filepath));
    if content.len() == 0 || content.chars().last().unwrap() != '\n' {
        content += "\n";
    }
    let input = Rc::new(content);
    tokenize(input, filepath)
}

impl TokenIter {
    pub fn new(input: Rc<String>, filepath: Rc<String>) -> TokenIter {
        TokenIter {
            input,
            pos: TokenPos { tk: 0, bytes: 0 },
            prev_pos: TokenPos { tk: 0, bytes: 0 },
            filepath,
        }
    }

    pub fn next(&mut self) -> Result<Option<Token>, Error> {
        let is_bol;
        match calc_space_len(self.cur_str()) {
            Ok((sp, _is_bol)) => {
                self.pos.bytes += sp;
                is_bol = _is_bol;
            }
            Err(e) => return Err(self.error_at(&e)),
        }
        let s = self.cur_str();
        if s.is_empty() {
            return Ok(None);
        }

        if let Some((mut tk, pos)) = self.is_op(s) {
            self.prev_pos = self.pos;
            self.pos += pos;
            tk.is_bol = is_bol;
            return Ok(Some(tk));
        }

        if let Some((mut tk, pos)) = self.is_keyword(s) {
            self.prev_pos = self.pos;
            self.pos += pos;
            tk.is_bol = is_bol;
            tk.is_bol = is_bol;
            return Ok(Some(tk));
        }

        if let Some((mut tk, pos)) = self.is_num(s) {
            self.prev_pos = self.pos;
            self.pos += pos;
            tk.is_bol = is_bol;
            return Ok(Some(tk));
        }

        if let Some((mut tk, pos)) = self.is_semi(s) {
            self.prev_pos = self.pos;
            self.pos += pos;
            tk.is_bol = is_bol;
            return Ok(Some(tk));
        }

        if let Some((mut tk, pos)) = self.is_colon(s) {
            self.prev_pos = self.pos;
            self.pos += pos;
            tk.is_bol = is_bol;
            return Ok(Some(tk));
        }

        if let Some((mut tk, pos)) = self.is_block_paren(s) {
            self.prev_pos = self.pos;
            self.pos += pos;
            tk.is_bol = is_bol;
            return Ok(Some(tk));
        }

        if let Some((mut tk, pos)) = self.is_comma(s) {
            self.prev_pos = self.pos;
            self.pos += pos;
            tk.is_bol = is_bol;
            return Ok(Some(tk));
        }

        if let Some((mut tk, pos)) = self.is_period(s) {
            self.prev_pos = self.pos;
            self.pos += pos;
            tk.is_bol = is_bol;
            return Ok(Some(tk));
        }

        if let Some((mut tk, pos)) = self.is_question(s) {
            self.prev_pos = self.pos;
            self.pos += pos;
            tk.is_bol = is_bol;
            return Ok(Some(tk));
        }

        if let Some((mut tk, pos)) = self.is_hashmark(s) {
            self.prev_pos = self.pos;
            self.pos += pos;
            tk.is_bol = is_bol;
            return Ok(Some(tk));
        }

        if let Some((mut tk, pos)) = self.is_string(s)? {
            self.prev_pos = self.pos;
            self.pos += pos;
            tk.is_bol = is_bol;
            return Ok(Some(tk));
        }

        if let Some((mut tk, pos)) = self.is_char(s)? {
            self.prev_pos = self.pos;
            self.pos += pos;
            tk.is_bol = is_bol;
            return Ok(Some(tk));
        }

        if let Some((mut tk, pos)) = self.is_base_type(s) {
            self.prev_pos = self.pos;
            self.pos += pos;
            tk.is_bol = is_bol;
            return Ok(Some(tk));
        }

        // これ最後の方がいい
        // 最後にしないと予約後が変数として扱われちゃう
        if let Some((mut tk, pos)) = self.is_ident(s) {
            self.prev_pos = self.pos;
            self.pos += pos;
            tk.is_bol = is_bol;
            return Ok(Some(tk));
        }
        Err(self.error_at("トークナイズできません"))
    }

    // /// std::iter::Peekable.peek()に似てるけど、posを元に戻す
    // pub fn peek(&mut self) -> Option<Token> {
    //     let cur_pos = self.pos;
    //     let prev_pos = self.prev_pos;
    //     let result = self.next();
    //     self.pos = cur_pos;
    //     self.prev_pos = prev_pos;
    //     result
    // }

    pub fn back(&mut self, tk: usize, bytes: usize) {
        self.pos.tk -= tk;
        self.pos.bytes -= bytes;
    }

    pub fn prev(&mut self) {
        self.pos = self.prev_pos;
    }

    fn new_token(&self, kind: TokenKind) -> Token {
        Token::new(
            kind,
            self.pos,
            self.prev_pos,
            self.input.clone(),
            self.filepath.clone(),
            false,
        )
    }

    fn is_op(&self, s: &str) -> Option<(Token, TokenPos)> {
        use self::TokenKind::*;
        if let Ok(op) = Operator::from_starts(s) {
            return Some((
                self.new_token(Reserved(op)),
                TokenPos::new_bytes(op.as_str().len()),
            ));
        }
        None
    }

    fn is_keyword(&self, s: &str) -> Option<(Token, TokenPos)> {
        if let Ok(keyword) = KeyWord::from_starts(s) {
            let len = keyword.as_str().len();
            if !is_alnum(s.chars().nth(len).unwrap_or_else(|| ' ')) {
                let kind = TokenKind::KeyWord(keyword);
                return Some((self.new_token(kind), TokenPos::new_bytes(len)));
            }
        }
        None
    }

    fn is_num(&self, s: &str) -> Option<(Token, TokenPos)> {
        use self::TokenKind::*;
        let (digit, _, bytes) = split_digit(s);
        if !digit.is_empty() {
            return Some((
                self.new_token(Num(i64::from_str_radix(digit, 10).unwrap())),
                TokenPos::new_bytes(bytes),
            ));
        }
        return None;
    }

    fn is_semi(&self, s: &str) -> Option<(Token, TokenPos)> {
        use self::TokenKind::*;
        let ss = s.chars().nth(0).unwrap();
        if ss.to_string() == SemiColon.as_string() {
            return Some((self.new_token(SemiColon), TokenPos::new_bytes(1)));
        }
        None
    }

    fn is_colon(&self, s: &str) -> Option<(Token, TokenPos)> {
        use self::TokenKind::*;
        let ss = s.chars().nth(0).unwrap();
        if ss.to_string() == Colon.as_string() {
            return Some((self.new_token(Colon), TokenPos::new_bytes(1)));
        }
        None
    }

    fn is_question(&self, s: &str) -> Option<(Token, TokenPos)> {
        use self::TokenKind::*;
        let ss = s.chars().nth(0).unwrap();
        if ss.to_string() == Question.as_string() {
            return Some((self.new_token(Question), TokenPos::new_bytes(1)));
        }
        None
    }

    fn is_hashmark(&self, s: &str) -> Option<(Token, TokenPos)> {
        use self::TokenKind::*;
        let ss = s.chars().nth(0).unwrap();
        if ss.to_string() == HashMark.as_string() {
            return Some((self.new_token(HashMark), TokenPos::new_bytes(1)));
        }
        None
    }

    fn is_ident(&self, s: &str) -> Option<(Token, TokenPos)> {
        let (ident, _, first_non_num_idx) = split_ident(s);
        if !ident.is_empty() {
            return Some((
                self.new_token(TokenKind::Ident(Ident::new(ident))),
                TokenPos::new_bytes(first_non_num_idx),
            ));
        }
        None
    }

    fn is_block_paren(&self, s: &str) -> Option<(Token, TokenPos)> {
        let ss = s.chars().nth(0).unwrap();
        if let Ok(x) = Block::from_str(&ss.to_string()) {
            return Some((self.new_token(TokenKind::Block(x)), TokenPos::new(1, 1)));
        }
        None
    }

    fn is_comma(&self, s: &str) -> Option<(Token, TokenPos)> {
        use self::TokenKind::*;
        let ss = s.chars().nth(0).unwrap();
        if ss.to_string() == Comma.as_string() {
            return Some((self.new_token(Comma), TokenPos::new_bytes(1)));
        }
        None
    }

    fn is_period(&self, s: &str) -> Option<(Token, TokenPos)> {
        use self::TokenKind::*;
        let ss = s.chars().nth(0).unwrap();
        if ss.to_string() == Period.as_string() {
            return Some((self.new_token(Period), TokenPos::new_bytes(1)));
        }
        None
    }

    fn is_string(&self, s: &str) -> Result<Option<(Token, TokenPos)>, Error> {
        use TokenKind::DoubleQuote;
        let mut chars = s.chars();
        if let Some(x) = chars.next() {
            if x.to_string() == DoubleQuote.as_string() {
                let mut len = 1;
                let mut result = Vec::with_capacity(1024);
                loop {
                    if let Some(c) = chars.next() {
                        len += 1;
                        if c == '\"' {
                            break;
                        }

                        if c == '\\' {
                            if let Some(cc) = chars.next() {
                                len += 1;
                                result.push(get_escape_chars(cc));
                            } else {
                                return Err(self.error_at("reach EOF"));
                            }
                        } else {
                            result.push(c);
                        }
                    } else {
                        return Err(self.error_at("cannot find end of \""));
                    }
                }
                let result = result.into_iter().collect::<String>();

                return Ok(Some((
                    self.new_token(TokenKind::String(result)),
                    TokenPos::new_bytes(len),
                )));
            }
        }
        Ok(None)
    }

    fn is_char(&self, s: &str) -> Result<Option<(Token, TokenPos)>, Error> {
        use TokenKind::SingleQuote;
        let mut chars = s.chars();
        if chars
            .next()
            .map(|v| v.to_string() == SingleQuote.as_string())
            .unwrap_or(false)
        {
            let result: char;
            let mut len = 1;
            if let Some(c) = chars.next() {
                if c == '\\' {
                    if let Some(cc) = chars.next() {
                        result = get_escape_chars(cc);
                        len += 2;
                    } else {
                        return Err(self.error_at("reach EOF"));
                    }
                } else {
                    result = c;
                    len += 1;
                }
            } else {
                return Err(self.error_at("reach EOF"));
            }

            if chars.next() != Some('\'') {
                return Err(self.error_at("char length should be 1"));
            } else {
                len += 1;
            }

            return Ok(Some((
                self.new_token(TokenKind::Char(result)),
                TokenPos::new_bytes(len),
            )));
        }
        Ok(None)
    }

    fn is_base_type(&self, s: &str) -> Option<(Token, TokenPos)> {
        if let Ok(base_type) = TypeKind::from_starts(s) {
            let len = base_type.as_str().len();
            // specify ' ' in unwrap_of_else because !is_alnum(' ') is true at anytime
            if !is_alnum(s.chars().nth(len).unwrap_or_else(|| ' ')) {
                return Some((
                    self.new_token(TokenKind::TypeKind(base_type)),
                    TokenPos::new_bytes(len),
                ));
            }
        }
        None
    }

    fn error_at(&self, msg: impl Into<String>) -> Error {
        Error::invalid(
            self.filepath.clone(),
            self.input.clone(),
            self.pos,
            Some(msg.into()),
        )
    }

    /// ## warn
    /// you cannot change self.bytes_pos after using this method
    fn cur_str(&self) -> &str {
        let a = self.pos.bytes;
        &self.input[a..]
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
    for (i, c) in s.char_indices() {
        if let Ok(_) = Operator::from_starts(&s[i..]) {
            break;
        }
        if s[i..].starts_with(&TokenKind::SemiColon.as_string())
            || s[i..].starts_with(&TokenKind::Colon.as_string())
            || s[i..].starts_with(" ")
            || s[i..].starts_with(&TokenKind::Comma.as_string())
            || s[i..].starts_with(Block::LParen.as_str())
            || s[i..].starts_with(Block::RParen.as_str())
            || s[i..].starts_with(Comment::Single.as_str())
            || s[i..].starts_with(Comment::MultiStart.as_str())
            || s[i..].starts_with(&TokenKind::Period.as_string())
            || s[i..].starts_with(&TokenKind::Question.as_string())
            || s[i..].starts_with(&TokenKind::HashMark.as_string())
            || s[i..].starts_with(&TokenKind::DoubleQuote.as_string())
            || s[i..].starts_with(&TokenKind::SingleQuote.as_string())
            || c.is_ascii_whitespace()
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
/// 次のトークンが行頭のものだったtrueを返す
fn calc_space_len(s: &str) -> Result<(usize, bool), String> {
    let mut begin = s.char_indices().peekable();
    let mut is_bol = false;

    while let Some((mut pos, mut chars)) = begin.next() {
        // for comment //
        if chars == '/' {
            if let Some((_, x)) = begin.next() {
                if x == '/' {
                    chars = ' ';
                    while let Some((_pos, _chars)) = begin.next() {
                        pos = _pos;
                        if _chars == '\n' {
                            break;
                        }
                    }
                } else if x == '*' {
                    chars = ' ';
                    while let Some((_, c)) = begin.next() {
                        if c == '*' {
                            if let Some((_pos, _chars)) = begin.next() {
                                pos = _pos;
                                if _chars == '/' {
                                    break;
                                }
                            } else {
                                return Err("comment is not closed.".to_string());
                            }
                        }
                    }
                }
            }
        }

        if chars == '\n' {
            is_bol = true;
        }

        if !chars.is_whitespace() {
            return Ok((pos, is_bol));
        }
        if begin.peek() == None {
            return Ok((pos + 1, is_bol));
        }
    }
    Ok((0, true))
}

fn is_alnum(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn get_escape_chars(c: char) -> char {
    match c {
        'a' => 7 as char,
        'b' => 8 as char,
        't' => 9 as char,
        'n' => 10 as char,
        'v' => 11 as char,
        'f' => 12 as char,
        'r' => 13 as char,
        'e' => 27 as char,
        '0' => 0 as char,
        _ => c,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_iter() {
        use self::KeyWord::*;
        use self::Operator::*;
        use self::TokenKind::{KeyWord, Num, Reserved, SemiColon};
        let input =
            "== != = < <= > >= + - * / ( ) & sizeof [ ] -> ++ += -= *= /= ! ~ |  ^ || && >> << <<= >>= ... &= |= ^=";
        let expected = vec![
            Equal, Neq, Assign, Lesser, Leq, Greater, Geq, Plus, Minus, Mul, Div, LParen, RParen,
            Ampersand, Sizeof, LArr, RArr, Arrow, PlusPlus, APlus, AMinus, AMul, ADiv, Not, BitNot,
            BitOr, BitXor, LogOr, LogAnd, RShift, LShift, ALShift, ARShift, ThreeDots, ABitAnd,
            ABitOr, ABitXor,
        ];
        let mut iter = tokenize(Rc::new(input.to_string()), Rc::new(String::new())).unwrap();
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
        let mut iter = tokenize(Rc::new(input.to_string()), Rc::new(String::new())).unwrap();
        for i in expected {
            assert_eq!(i, iter.next().unwrap().kind);
        }
        assert_eq!(None, iter.next());

        let input = "return; returnx return1 return 1 for while if else force whilet ifelse elseif  struct . typedef enum static break continue goto : switch case default ? extern do #";

        let expected = vec![
            KeyWord(Return),
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
            KeyWord(Struct),
            TokenKind::Period,
            KeyWord(Typedef),
            KeyWord(Enum),
            KeyWord(Static),
            KeyWord(Break),
            KeyWord(Continue),
            KeyWord(Goto),
            TokenKind::Colon,
            KeyWord(Switch),
            KeyWord(Case),
            KeyWord(Default),
            TokenKind::Question,
            KeyWord(Extern),
            KeyWord(Do),
            TokenKind::HashMark,
        ];
        let mut iter = tokenize(Rc::new(input.to_string()), Rc::new(String::new())).unwrap();
        for i in expected {
            assert_eq!(i, iter.next().unwrap().kind);
        }
        assert_eq!(None, iter.next());

        let input = r###""hello" "hello\n" "hello\"" 'a' '\n' '\''"###;
        let expected = [
            TokenKind::String("hello".to_string()),
            TokenKind::String("hello\n".to_string()),
            TokenKind::String("hello\"".to_string()),
            TokenKind::Char('a'),
            TokenKind::Char('\n'),
            TokenKind::Char('\''),
        ];
        let mut iter = tokenize(Rc::new(input.to_string()), Rc::new(String::new())).unwrap();
        for i in &expected {
            assert_eq!(i, &iter.next().unwrap().kind);
        }
        assert_eq!(None, iter.next());

        let input = r###"
        // hello. this is comment 
        ;
        /*
        hello 
        this 
        is 
        multi 
        line
        comment */
        ; //
        "###;
        let expected = vec![TokenKind::SemiColon, TokenKind::SemiColon];
        let mut iter = tokenize(Rc::new(input.to_string()), Rc::new(String::new())).unwrap();
        for i in expected {
            assert_eq!(i, iter.next().unwrap().kind);
        }
        assert_eq!(None, iter.next());

        let input = "{ { } ,hoge, int int1 char char1 short long void _Bool";
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
            TokenKind::TypeKind(TypeKind::Short),
            TokenKind::TypeKind(TypeKind::Long),
            TokenKind::TypeKind(TypeKind::Void),
            TokenKind::TypeKind(TypeKind::_Bool),
        ];
        let mut iter = tokenize(Rc::new(input.to_string()), Rc::new(String::new())).unwrap();
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
            (">=", Ok(Geq)),
            (">=>", Ok(Geq)),
            ("+", Ok(Plus)),
            ("+=", Ok(APlus)),
            ("-", Ok(Minus)),
            ("-=", Ok(AMinus)),
            ("*", Ok(Mul)),
            ("*=", Ok(AMul)),
            ("/", Ok(Div)),
            ("//", Ok(Div)),
            ("/=", Ok(ADiv)),
            ("(", Ok(LParen)),
            ("(=", Ok(LParen)),
            (")", Ok(RParen)),
            ("))", Ok(RParen)),
            ("&", Ok(Ampersand)),
            ("sizeof", Ok(Sizeof)),
            ("++", Ok(PlusPlus)),
            ("--", Ok(MinusMinus)),
            ("!", Ok(Not)),
            ("~", Ok(BitNot)),
            ("|", Ok(BitOr)),
            ("^", Ok(BitXor)),
            ("||", Ok(LogOr)),
            ("&&", Ok(LogAnd)),
            (">>", Ok(RShift)),
            ("<<", Ok(LShift)),
            ("<<=", Ok(ALShift)),
            (">>=", Ok(ARShift)),
            ("...", Ok(ThreeDots)),
            ("&=", Ok(ABitAnd)),
            ("|=", Ok(ABitOr)),
            ("^=", Ok(ABitXor)),
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
        let tests = [("    a", 4), ("a", 0), ("// ", 3)];
        for (s, expected) in &tests {
            assert_eq!(expected, &calc_space_len(s).unwrap().0);
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
