use self::NodeKind::*;
use super::error::Error;
use crate::token::{Block, KeyWord, Operator, TokenIter, TokenKind};
use std::rc::Rc;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub enum NodeKind {
    Assign,
    Equal,
    Neq,
    Lesser,
    Leq,
    Greater,
    Geq,
    Add,
    Sub,
    Mul,
    Div,
    Return,
    If,
    Else,
    While,
    For,
    Block(Vec<Node>),
    Func(String, Vec<Node>), // (func_name,args)
    Num(u64),
    // Ident(Ident),
    Lvar(Rc<Lvar>), // usize はベースポインタからのオフセット
}

impl NodeKind {
    /// convert NodeKind to token::Operator
    pub fn as_op(&self) -> Result<Operator, ()> {
        match self {
            Assign => Ok(Operator::Assign),
            Equal => Ok(Operator::Equal),
            Neq => Ok(Operator::Neq),
            Lesser => Ok(Operator::Lesser),
            Leq => Ok(Operator::Leq),
            Greater => Ok(Operator::Greater),
            Geq => Ok(Operator::Geq),
            Add => Ok(Operator::Plus),
            Sub => Ok(Operator::Minus),
            Mul => Ok(Operator::Mul),
            Div => Ok(Operator::Div),
            _ => Err(()),
        }
    }

    pub fn from_op(op: Operator) -> Result<NodeKind, ()> {
        match op {
            x if x == Assign.as_op().unwrap() => Ok(Assign),
            x if x == Equal.as_op().unwrap() => Ok(Equal),
            x if x == Neq.as_op().unwrap() => Ok(Neq),
            x if x == Lesser.as_op().unwrap() => Ok(Lesser),
            x if x == Leq.as_op().unwrap() => Ok(Leq),
            x if x == Greater.as_op().unwrap() => Ok(Greater),
            x if x == Geq.as_op().unwrap() => Ok(Geq),
            x if x == Add.as_op().unwrap() => Ok(Add),
            x if x == Sub.as_op().unwrap() => Ok(Sub),
            x if x == Mul.as_op().unwrap() => Ok(Mul),
            x if x == Div.as_op().unwrap() => Ok(Div),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct Ident {
    name: String,
}

impl Ident {
    fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct Node {
    pub kind: NodeKind,
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
    pub cond: Option<Box<Node>>,
    pub then: Option<Box<Node>>,
    pub els: Option<Box<Node>>,
    pub init: Option<Box<Node>>,
    pub inc: Option<Box<Node>>,
}

impl Node {
    pub fn new(kind: NodeKind, lhs: Node, rhs: Node) -> Node {
        let mut node = Node::new_none(kind);
        node.lhs = Some(Box::new(lhs));
        node.rhs = Some(Box::new(rhs));
        node
    }

    pub fn _new(
        kind: NodeKind,
        lhs: Option<Box<Node>>,
        rhs: Option<Box<Node>>,
        cond: Option<Box<Node>>,
        then: Option<Box<Node>>,
        els: Option<Box<Node>>,
        init: Option<Box<Node>>,
        inc: Option<Box<Node>>,
    ) -> Node {
        Node {
            kind,
            lhs,
            rhs,
            cond,
            then,
            els,
            init,
            inc,
        }
    }

    pub fn new_num(val: u64) -> Node {
        Node::new_none(Num(val))
    }

    pub fn new_leaf(kind: NodeKind) -> Node {
        Node::new_none(kind)
    }

    pub fn new_unary(kind: NodeKind, lhs: Node) -> Node {
        let mut node = Node::new_none(kind);
        node.lhs = Some(Box::new(lhs));
        node
    }

    pub fn new_cond(kind: NodeKind, cond: Node) -> Node {
        let mut node = Node::new_none(kind);
        node.cond = Some(Box::new(cond));
        node
    }

    pub fn new_none(kind: NodeKind) -> Node {
        Node {
            kind,
            lhs: None,
            rhs: None,
            cond: None,
            then: None,
            els: None,
            init: None,
            inc: None,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct Lvar {
    next: Option<Rc<Lvar>>,
    name: String,
    pub offset: usize,
}

impl Lvar {
    pub fn new(next: Lvar, name: impl Into<String>, offset: usize) -> Self {
        Self {
            next: Some(Rc::new(next)),
            name: name.into(),
            offset,
        }
    }

    pub fn new_leaf(name: impl Into<String>, offset: usize) -> Self {
        Self {
            next: None,
            name: name.into(),
            offset,
        }
    }
}

pub struct Context {
    pub lvar: Option<Rc<Lvar>>,
    count: usize,
}

impl Context {
    pub fn new() -> Self {
        Self {
            lvar: None,
            count: 0,
        }
    }

    pub fn push_front(&mut self, name: impl Into<String>, offset: usize) {
        self.count += 1;
        self.lvar = Some(Rc::new(Lvar {
            next: self.lvar.take(),
            name: name.into(),
            offset: offset + 8,
        }))
    }

    pub fn find_lvar(&self, name: impl Into<String>) -> Option<Rc<Lvar>> {
        if let Some(ref lvar) = self.lvar {
            Self::_find_lvar(lvar, name)
        } else {
            None
        }
    }

    fn _find_lvar(lvar: &Rc<Lvar>, name: impl Into<String>) -> Option<Rc<Lvar>> {
        let name = name.into();
        if lvar.name == name {
            Some(lvar.clone())
        } else {
            if let Some(ref next) = lvar.next {
                Self::_find_lvar(next, name)
            } else {
                None
            }
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct Program {
    pub functions: Vec<Function>,
}

impl Program {
    fn new() -> Self {
        Self {
            functions: Vec::new(),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct Function {
    pub name: String,
    pub lvars: Option<Rc<Lvar>>,
    pub var_num: usize, // lvar + param
    pub params: Vec<Ident>,
    pub param_num: usize,
    pub nodes: Vec<Node>,
}

impl Function {
    fn new(
        name: impl Into<String>,
        lvars: Option<Rc<Lvar>>,
        var_num: usize,
        params: Vec<Ident>,
        param_num: usize,
        nodes: Vec<Node>,
    ) -> Self {
        Self {
            name: name.into(),
            lvars,
            var_num,
            params,
            param_num,
            nodes,
        }
    }
}

// program     = function*
pub fn program(iter: &mut TokenIter) -> Result<Program, Error> {
    let mut program = Program::new();
    while iter.peek() != None {
        program.functions.push(function(iter)?);
    }
    Ok(program)
}

// function    = ident "(" params? ")" "{" stmt* "}"
pub fn function(iter: &mut TokenIter) -> Result<Function, Error> {
    let ident = expect_ident(iter)?;
    expect(iter, Operator::LParen)?;

    let mut fn_params = Vec::new();
    if !consume(iter, Operator::RParen) {
        fn_params = params(iter)?;
        expect(iter, Operator::RParen)?;
    }
    expect_block(iter, Block::LParen)?;

    let param_len = fn_params.len();
    let mut lvars = Context::new();
    for fn_param in fn_params.clone() {
        lvars.push_front(
            fn_param.name,
            lvars.lvar.as_ref().map(|lvar| lvar.offset).unwrap_or(0),
        )
    }

    let mut stmt_vec = Vec::new();
    loop {
        if consume_block(iter, Block::RParen) {
            return Ok(Function::new(
                ident.name,
                lvars.lvar,
                lvars.count,
                fn_params,
                param_len,
                stmt_vec,
            ));
        }
        stmt_vec.push(stmt(iter, &mut lvars)?);
    }
}

// params      = ident ("," ident)*
pub fn params(iter: &mut TokenIter) -> Result<Vec<Ident>, Error> {
    let mut params = vec![expect_ident(iter)?];
    while consume_comma(iter) {
        params.push(expect_ident(iter)?);
    }
    Ok(params)
}

// stmt        = expr ";"
//             | "return" expr ";"
//             | "if" "(" expr ")" stmt
//             | "while" "(" expr ")" stmt
//             | "for" "(" expr? ";" expr? ";" expr? ")" stmt
//             | "{" stmt* "}"
pub fn stmt(iter: &mut TokenIter, ctx: &mut Context) -> Result<Node, Error> {
    if let Some(x) = iter.peek() {
        match x.kind {
            TokenKind::KeyWord(key) => match key {
                KeyWord::Return => {
                    iter.next();
                    let node = Node::new_unary(Return, expr(iter, ctx)?);
                    expect_semi(iter)?;
                    return Ok(node);
                }
                KeyWord::If => {
                    iter.next();
                    expect(iter, Operator::LParen)?;
                    let mut node = Node::new_cond(If, expr(iter, ctx)?);
                    expect(iter, Operator::RParen)?;
                    node.then = Some(Box::new(stmt(iter, ctx)?));
                    if let Some(x) = iter.peek() {
                        if x.kind == TokenKind::KeyWord(KeyWord::Else) {
                            iter.next();
                            node.els = Some(Box::new(stmt(iter, ctx)?));
                        }
                    }
                    return Ok(node);
                }
                KeyWord::While => {
                    iter.next();
                    expect(iter, Operator::LParen)?;
                    let mut node = Node::new_cond(While, expr(iter, ctx)?);
                    expect(iter, Operator::RParen)?;
                    node.then = Some(Box::new(stmt(iter, ctx)?));
                    return Ok(node);
                }
                KeyWord::For => {
                    iter.next();
                    expect(iter, Operator::LParen)?;
                    let mut node = Node::new_none(For);
                    if !consume_semi(iter) {
                        node.init = Some(Box::new(expr(iter, ctx)?));
                        expect_semi(iter)?;
                    }
                    if !consume_semi(iter) {
                        node.cond = Some(Box::new(expr(iter, ctx)?));
                        expect_semi(iter)?;
                    }
                    if !consume_semi(iter) {
                        node.inc = Some(Box::new(expr(iter, ctx)?));
                    }
                    expect(iter, Operator::RParen)?;
                    node.then = Some(Box::new(stmt(iter, ctx)?));
                    return Ok(node);
                }
                _ => (),
            },
            TokenKind::Block(block) => match block {
                Block::LParen => {
                    iter.next();
                    let mut stmt_vec = Vec::new();
                    loop {
                        if consume_block(iter, Block::RParen) {
                            return Ok(Node::new_none(Block(stmt_vec)));
                        }
                        stmt_vec.push(stmt(iter, ctx)?);
                    }
                }
                _ => {
                    return Err(Error::unexpected_token(
                        iter.s,
                        x.clone(),
                        TokenKind::Block(Block::LParen),
                    ))
                }
            },
            _ => (),
        }
    }
    let node = expr(iter, ctx)?;
    expect_semi(iter)?;
    Ok(node)
}

// expr        = assig
pub fn expr(iter: &mut TokenIter, ctx: &mut Context) -> Result<Node, Error> {
    assign(iter, ctx)
}

// assign      = equality ("=" assign)?
pub fn assign(iter: &mut TokenIter, ctx: &mut Context) -> Result<Node, Error> {
    let mut node = equality(iter, ctx)?;
    if consume(iter, Operator::Assign) {
        node = Node::new(Assign, node, assign(iter, ctx)?);
    }
    return Ok(node);
}

// equality    = relational ("==" relational | "!=" relational)*
pub fn equality(iter: &mut TokenIter, ctx: &mut Context) -> Result<Node, Error> {
    let mut node = relational(iter, ctx)?;
    loop {
        if consume(iter, Operator::Equal) {
            node = Node::new(Equal, node, relational(iter, ctx)?);
        } else if consume(iter, Operator::Neq) {
            node = Node::new(Neq, node, relational(iter, ctx)?);
        } else {
            return Ok(node);
        }
    }
}

// relational  = add ("<" add | "<=" | ">" add | ">=" add)*
pub fn relational(iter: &mut TokenIter, ctx: &mut Context) -> Result<Node, Error> {
    let mut node = add(iter, ctx)?;
    loop {
        if consume(iter, Operator::Lesser) {
            node = Node::new(Lesser, node, add(iter, ctx)?);
        } else if consume(iter, Operator::Leq) {
            node = Node::new(Leq, node, add(iter, ctx)?);
        } else if consume(iter, Operator::Greater) {
            // 左右を入れ替えて読み変える
            node = Node::new(Lesser, add(iter, ctx)?, node);
        } else if consume(iter, Operator::Geq) {
            // 左右を入れ替えて読み変える
            node = Node::new(Leq, add(iter, ctx)?, node);
        } else {
            return Ok(node);
        }
    }
}

// add         = mul ("+" mul | "-" mul)*
pub fn add(iter: &mut TokenIter, ctx: &mut Context) -> Result<Node, Error> {
    let mut node = mul(iter, ctx)?;
    loop {
        if consume(iter, Operator::Plus) {
            node = Node::new(Add, node, mul(iter, ctx)?)
        } else if consume(iter, Operator::Minus) {
            node = Node::new(Sub, node, mul(iter, ctx)?)
        } else {
            return Ok(node);
        }
    }
}

// mul         = unary ("*" unary | "/" unary)*
pub fn mul(iter: &mut TokenIter, ctx: &mut Context) -> Result<Node, Error> {
    let mut node = unary(iter, ctx)?;
    loop {
        if consume(iter, Operator::Mul) {
            node = Node::new(Mul, node, unary(iter, ctx)?)
        } else if consume(iter, Operator::Div) {
            node = Node::new(Div, node, unary(iter, ctx)?)
        } else {
            return Ok(node);
        }
    }
}

// unary       = ("+" | "-")? primary
pub fn unary(iter: &mut TokenIter, ctx: &mut Context) -> Result<Node, Error> {
    if consume(iter, Operator::Plus) {
        return primary(iter, ctx);
    } else if consume(iter, Operator::Minus) {
        return Ok(Node::new(Sub, Node::new_num(0), primary(iter, ctx)?));
    }
    return primary(iter, ctx);
}

// primary     = num | ident func-args? | "(" expr ")"
pub fn primary(iter: &mut TokenIter, ctx: &mut Context) -> Result<Node, Error> {
    if consume(iter, Operator::LParen) {
        let node = expr(iter, ctx)?;
        expect(iter, Operator::RParen)?;
        return Ok(node);
    }

    if let Some(ident) = consume_ident(iter) {
        if consume(iter, Operator::LParen) {
            return Ok(Node::new_leaf(Func(ident.name, func_args(iter, ctx)?)));
        }
        if let Some(lvar) = ctx.find_lvar(&ident.name) {
            return Ok(Node::new_leaf(Lvar(lvar)));
        } else {
            ctx.push_front(
                ident.name,
                ctx.lvar.as_ref().map(|lvar| lvar.offset).unwrap_or(0), // if ctx.lvar == None {return 0} else {return ctx.lvar.offset}
            );
            return Ok(Node::new_leaf(Lvar(ctx.lvar.as_ref().unwrap().clone())));
        }
    }
    return Ok(Node::new_num(expect_num(iter)?));
}

// func-args   = "(" (assign ("," assign)*)? ")"
fn func_args(iter: &mut TokenIter, ctx: &mut Context) -> Result<Vec<Node>, Error> {
    if consume(iter, Operator::RParen) {
        return Ok(vec![]);
    }
    let mut args = vec![assign(iter, ctx)?];
    while consume_comma(iter) {
        args.push(assign(iter, ctx)?);
    }
    expect(iter, Operator::RParen)?;
    Ok(args)
}

fn consume(iter: &mut TokenIter, op: Operator) -> bool {
    if let Some(x) = iter.peek() {
        if let TokenKind::Reserved(x) = x.kind {
            if x == op {
                iter.next();
                return true;
            }
        }
    }
    return false;
}

fn _consume_keyword(iter: &mut TokenIter, key: KeyWord) -> bool {
    if let Some(x) = iter.peek() {
        if let TokenKind::KeyWord(x) = x.kind {
            if x == key {
                iter.next();
                return true;
            }
        }
    }
    false
}

fn consume_semi(iter: &mut TokenIter) -> bool {
    if let Some(x) = iter.peek() {
        if x.kind == TokenKind::SemiColon {
            iter.next();
            return true;
        }
    }
    return false;
}

fn consume_ident(iter: &mut TokenIter) -> Option<Ident> {
    if let Some(x) = iter.peek() {
        if let TokenKind::Ident(x) = x.kind {
            iter.next();
            return Some(Ident::new(x.name));
        }
    }
    return None;
}

fn consume_block(iter: &mut TokenIter, block: Block) -> bool {
    if let Some(x) = iter.peek() {
        if let TokenKind::Block(x) = x.kind {
            if x == block {
                iter.next();
                return true;
            }
        }
    }
    return false;
}

fn consume_comma(iter: &mut TokenIter) -> bool {
    if let Some(x) = iter.peek() {
        if x.kind == TokenKind::Comma {
            iter.next();
            return true;
        }
    }
    false
}

fn _consume_token_kind(iter: &mut TokenIter, kind: TokenKind) -> Option<TokenKind> {
    if let Some(x) = iter.peek() {
        if x.kind == kind {
            iter.next();
            return Some(x.kind);
        }
    }
    None
}

fn expect(iter: &mut TokenIter, op: Operator) -> Result<(), Error> {
    if let Some(x) = iter.peek() {
        if let TokenKind::Reserved(xx) = x.kind {
            if xx == op {
                iter.next();
                return Ok(());
            } else {
                return Err(Error::unexpected_token(
                    iter.s,
                    x.clone(),
                    TokenKind::Reserved(op),
                ));
            }
        }
    }
    return Err(Error::eof(iter.s, iter.pos, TokenKind::Reserved(op), None));
}

fn expect_num(iter: &mut TokenIter) -> Result<u64, Error> {
    if let Some(x) = iter.peek() {
        if let TokenKind::Num(xx) = x.kind {
            iter.next();
            return Ok(xx);
        } else {
            return Err(Error::unexpected_token(
                iter.s,
                x.clone(),
                TokenKind::Num(0),
            ));
        }
    }
    Err(Error::eof(iter.s, iter.pos, TokenKind::Num(0), None))
}

fn expect_semi(iter: &mut TokenIter) -> Result<(), Error> {
    if let Some(x) = iter.peek() {
        if x.kind == TokenKind::SemiColon {
            iter.next();
            return Ok(());
        } else {
            return Err(Error::unexpected_token(
                iter.s,
                x.clone(),
                TokenKind::SemiColon,
            ));
        }
    }
    Err(Error::eof(iter.s, iter.pos, TokenKind::SemiColon, None))
}

fn _expect_comma(iter: &mut TokenIter) -> Result<(), Error> {
    expect_token_kind(iter, TokenKind::Comma)?;
    Ok(())
}

fn expect_token_kind(iter: &mut TokenIter, kind: TokenKind) -> Result<TokenKind, Error> {
    if let Some(x) = iter.peek() {
        if x.kind == kind {
            iter.next();
            return Ok(x.kind);
        } else {
            return Err(Error::unexpected_token(
                iter.s,
                x.clone(),
                TokenKind::SemiColon,
            ));
        }
    }
    Err(Error::eof(iter.s, iter.pos, TokenKind::SemiColon, None))
}

fn expect_ident(iter: &mut TokenIter) -> Result<Ident, Error> {
    if let Some(x) = iter.peek() {
        if let TokenKind::Ident(id) = x.kind {
            iter.next();
            return Ok(Ident::new(id.name));
        } else {
            return Err(Error::unexpected_token(
                iter.s,
                x.clone(),
                TokenKind::Ident(crate::token::Ident::new("")),
            ));
        }
    }
    Err(Error::eof(
        iter.s,
        iter.pos,
        TokenKind::Ident(crate::token::Ident::new("")),
        None,
    ))
}

fn expect_block(iter: &mut TokenIter, block: Block) -> Result<(), Error> {
    if let Some(x) = iter.peek() {
        if let TokenKind::Block(x) = x.kind {
            if x == block {
                iter.next();
                return Ok(());
            }
        } else {
            return Err(Error::unexpected_token(
                iter.s,
                x.clone(),
                TokenKind::Block(block),
            ));
        }
    }
    Err(Error::eof(iter.s, iter.pos, TokenKind::Block(block), None))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr() {
        use crate::token;
        use NodeKind::*;
        let tests = [
            ("1==10", make_test_node(Equal, 1, 10)),
            ("1 != 10", make_test_node(Neq, 1, 10)),
            ("1  <10", make_test_node(Lesser, 1, 10)),
            ("1<=10", make_test_node(Leq, 1, 10)),
            ("1>10", make_test_node(Lesser, 10, 1)), // Lesser,LeqはGreater.Geqを使って実装されてる
            ("1>=10", make_test_node(Leq, 10, 1)),
            ("1+10", make_test_node(Add, 1, 10)),
            ("1-10", make_test_node(Sub, 1, 10)),
            ("1*10", make_test_node(Mul, 1, 10)),
            ("1/10", make_test_node(Div, 1, 10)),
            ("+1", Node::new_num(1)),
            ("-1", make_test_node(Sub, 0, 1)),
            (
                "2 * ( 3 + 4)",
                Node::new(Mul, Node::new_num(2), make_test_node(Add, 3, 4)),
            ),
            ("42", Node::new_num(42)),
            (
                "a+1=5",
                Node::new(
                    Assign,
                    Node::new(
                        Add,
                        Node::new_leaf(Lvar(Rc::new(super::Lvar::new_leaf("a", 8)))),
                        Node::new_num(1),
                    ),
                    Node::new_num(5),
                ),
            ),
        ];

        for (s, expected) in &tests {
            assert_eq!(
                expected,
                &expr(&mut token::tokenize(s), &mut Context::new()).unwrap()
            )
        }
    }

    #[test]
    fn test_stmt() {
        use crate::token;

        let lvar = Lvar::new_leaf("foo", 8);
        let lhs_f_node = Node::new_leaf(Lvar(Rc::new(lvar.clone())));
        let f_node = Node::new(Assign, lhs_f_node, Node::new_num(10));
        let lhs_s_node = Node::new_leaf(Lvar(Rc::new(Lvar::new(lvar, "bar", 16))));
        let s_node = Node::new(Assign, lhs_s_node, Node::new_num(20));
        let tests = [
            ("a=10;", vec![make_assign_node('a', 10, 8)]),
            ("foo=10;bar=20;", vec![f_node, s_node]),
        ];

        for (s, expected) in &tests {
            let mut iter = token::tokenize(s);
            let mut actual = Vec::new();
            let ctx = &mut Context::new();
            while iter.peek() != None {
                actual.push(stmt(&mut iter, ctx).unwrap());
            }
            assert_eq!(expected, &actual);
        }
    }

    #[test]
    fn test_if() {
        use crate::token;
        let cond = Node::new(Equal, Node::new_num(10), Node::new_num(20));
        let then = Node::new_unary(Return, Node::new_num(15));
        let expected = make_if_node(cond, then);

        let input = "if ( 10 ==20 ) return 15;";
        let actual = stmt(&mut token::tokenize(input), &mut Context::new()).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_if_else() {
        use crate::token;
        let cond = Node::new(Equal, Node::new_num(10), Node::new_num(20));
        let then = Node::new_unary(Return, Node::new_num(15));
        let els = Node::new_unary(Return, Node::new(Add, Node::new_num(10), Node::new_num(30)));
        let expected = make_if_else_node(cond, then, els);

        let input = "if ( 10 ==20 ) return 15; else return 10+30;";
        let actual = stmt(&mut token::tokenize(input), &mut Context::new()).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_while() {
        use crate::token;
        // Geqは左右を入れ替えてLeq
        let cond = Node::new(Leq, Node::new_num(20), Node::new_num(32));
        let then = Node::new_unary(Return, Node::new_num(10));
        let expected = make_while_node(cond, then);

        let input = "while (32 >= 20 ) return 10;";
        let actual = stmt(&mut token::tokenize(input), &mut Context::new()).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_for() {
        use crate::token;

        let init = make_assign_node("i", 0, 8);
        let cond = Node::new(
            Lesser,
            Node::new_leaf(Lvar(Rc::new(Lvar::new_leaf("i", 8)))),
            Node::new_num(10),
        );
        let tmp_inc = Node::new(
            Add,
            Node::new_leaf(Lvar(Rc::new(Lvar::new_leaf("i", 8)))),
            Node::new_num(1),
        );
        let inc = Node::new(
            Assign,
            Node::new_leaf(Lvar(Rc::new(Lvar::new_leaf("i", 8)))),
            tmp_inc,
        );

        let ret = Node::new(
            Add,
            Node::new_leaf(Lvar(Rc::new(Lvar::new_leaf("i", 8)))),
            Node::new_num(2),
        );
        let then = Node::new_unary(Return, ret);

        let expected = make_for_node(Some(init), Some(cond), Some(inc), then);

        let input = "for(i=0;i<10;i=i+1)return i+2;";
        let actual = stmt(&mut token::tokenize(input), &mut Context::new()).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_block() {
        use crate::token;
        let input = "{1; 2; hoge=4;}";
        let expected = vec![
            Node::new_num(1),
            Node::new_num(2),
            make_assign_node("hoge", 4, 8),
        ];
        let expected = vec![Node::new_none(Block(expected))];
        let mut iter = token::tokenize(input);
        let mut actual = Vec::new();
        while iter.peek() != None {
            actual.push(stmt(&mut iter, &mut Context::new()).unwrap());
        }
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_func() {
        use crate::token;
        let input = "add();";
        let expected_name = "add";
        let expected_args = vec![];
        let expected = make_fn_node(expected_name, expected_args);

        let mut iter = token::tokenize(input);
        let actual = stmt(&mut iter, &mut Context::new()).unwrap();
        assert_eq!(expected, actual);

        let input = "three(1,2,3);";
        let expected_name = "three";
        let expected_args = vec![Node::new_num(1), Node::new_num(2), Node::new_num(3)];
        let expected = make_fn_node(expected_name, expected_args);

        let mut iter = token::tokenize(input);
        let actual = stmt(&mut iter, &mut Context::new()).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_func_def() {
        use crate::token;
        let expected_fn_name = "main";
        let expected_nodes = vec![Node::new_unary(Return, Node::new_num(1))];
        let expected = Function::new(expected_fn_name, None, 0, Vec::new(), 0, expected_nodes);

        let input = "main(){return 1;}";
        let iter = &mut token::tokenize(input);
        let actual = function(iter).unwrap();

        assert_eq!(expected, actual);

        let expected_fn_name = "main";
        let lvar1 = Lvar::new_leaf("foo", 8);
        let lvar2 = Lvar::new(lvar1.clone(), "bar", 16);
        let expected_lvar = Rc::new(lvar2.clone());
        let node1 = make_assign_node("foo", 1, 8);
        let node2 = Node::new(
            Assign,
            Node::new_leaf(Lvar(Rc::new(lvar2.clone()))),
            Node::new_num(2),
        );
        let node3 = Node::new_unary(
            Return,
            Node::new(
                Add,
                Node::new_leaf(Lvar(Rc::new(lvar1))),
                Node::new_leaf(Lvar(Rc::new(lvar2))),
            ),
        );
        let expected_nodes = vec![node1, node2, node3];
        let expected = Function::new(
            expected_fn_name,
            Some(expected_lvar),
            2,
            Vec::new(),
            0,
            expected_nodes,
        );

        let input = "main(){foo = 1; bar = 2; return foo+bar;}";
        let iter = &mut token::tokenize(input);
        let actual = function(iter).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_param() {
        use crate::token;

        let expected = vec![Ident::new("hoge")];

        let input = "hoge";
        let iter = &mut token::tokenize(input);
        let actual = params(iter).unwrap();

        assert_eq!(expected, actual);

        let expected = vec![Ident::new("foo"), Ident::new("bar"), Ident::new("hoge")];
        let input = "foo,bar,hoge";
        let iter = &mut token::tokenize(input);
        let actual = params(iter).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_fn_with_args() {
        use crate::token;

        let expected_fn_name = "main";
        let expected_nodes = vec![Node::new_unary(Return, Node::new_num(0))];
        let expected = Function::new(expected_fn_name, None, 0, Vec::new(), 0, expected_nodes);

        let input = "main(){return 0;}";
        let iter = &mut token::tokenize(input);
        let actual = function(iter).unwrap();

        assert_eq!(expected, actual);

        let expected_fn_name = "main";
        let expected_nodes = vec![Node::new_unary(Return, Node::new_num(0))];
        let expected_param = vec![Ident::new("foo")];
        let expected_lvar = Lvar::new_leaf("foo", 8);
        let expected = Function::new(
            expected_fn_name,
            Some(Rc::new(expected_lvar)),
            1,
            expected_param,
            1,
            expected_nodes,
        );

        let input = "main(foo){return 0;}";
        let iter = &mut token::tokenize(input);
        let actual = function(iter).unwrap();

        assert_eq!(expected, actual);

        let expected_fn_name = "main";
        let expected_nodes = vec![Node::new_unary(Return, Node::new_num(0))];
        let expected_param = vec![
            Ident::new("foo"),
            Ident::new("bar"),
            Ident::new("hoge"),
            Ident::new("hey"),
        ];
        let expected_lvar = Lvar::new(
            Lvar::new(Lvar::new(Lvar::new_leaf("foo", 8), "bar", 16), "hoge", 24),
            "hey",
            32,
        );
        let expected = Function::new(
            expected_fn_name,
            Some(Rc::new(expected_lvar)),
            4,
            expected_param,
            4,
            expected_nodes,
        );

        let input = "main(foo,bar,hoge,hey){return 0;}";
        let iter = &mut token::tokenize(input);
        let actual = function(iter).unwrap();

        assert_eq!(expected, actual);
    }

    fn make_test_node(kind: NodeKind, lhs_num: u64, rhs_num: u64) -> Node {
        Node::new(kind, Node::new_num(lhs_num), Node::new_num(rhs_num))
    }

    fn make_assign_node(lhs: impl Into<String>, rhs: u64, offset: usize) -> Node {
        let mut node = Node::new_none(Assign);
        node.lhs = Some(Box::new(Node::new_leaf(Lvar(Rc::new(
            super::Lvar::new_leaf(lhs.into(), offset),
        )))));
        node.rhs = Some(Box::new(Node::new_num(rhs)));
        node
    }

    fn make_if_node(cond: Node, then: Node) -> Node {
        let mut node = Node::new_none(If);
        node.cond = Some(Box::new(cond));
        node.then = Some(Box::new(then));
        node
    }

    fn make_if_else_node(cond: Node, then: Node, els: Node) -> Node {
        let mut node = make_if_node(cond, then);
        node.els = Some(Box::new(els));
        node
    }

    fn make_while_node(cond: Node, then: Node) -> Node {
        let mut node = Node::new_none(While);
        node.cond = Some(Box::new(cond));
        node.then = Some(Box::new(then));
        node
    }

    fn make_for_node(
        init: Option<Node>,
        cond: Option<Node>,
        inc: Option<Node>,
        then: Node,
    ) -> Node {
        let mut node = Node::new_none(For);
        node.init = init.map(|c| Box::new(c));
        node.cond = cond.map(|c| Box::new(c));
        node.inc = inc.map(|c| Box::new(c));
        node.then = Some(Box::new(then));
        node
    }

    fn make_fn_node(name: impl Into<String>, args: Vec<Node>) -> Node {
        Node::new_none(Func(name.into(), args))
    }
}
