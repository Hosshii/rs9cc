use self::NodeKind::*;
use super::error::Error;
use crate::token::{KeyWord, Operator, TokenIter, TokenKind};

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
    Num(u64),
    // Ident(Ident),
    Lvar(Lvar), // usize はベースポインタからのオフセット
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
        Node {
            kind,
            lhs: Some(Box::new(lhs)),
            rhs: Some(Box::new(rhs)),
            cond: None,
            then: None,
            els: None,
            init: None,
            inc: None,
        }
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
        Node {
            kind: Num(val),
            lhs: None,
            rhs: None,
            cond: None,
            then: None,
            els: None,
            init: None,
            inc: None,
        }
    }

    pub fn new_leaf(kind: NodeKind) -> Node {
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

    pub fn new_unary(kind: NodeKind, lhs: Node) -> Node {
        Node {
            kind,
            lhs: Some(Box::new(lhs)),
            rhs: None,
            cond: None,
            then: None,
            els: None,
            init: None,
            inc: None,
        }
    }

    pub fn new_cond(kind: NodeKind, cond: Node) -> Node {
        Node {
            kind,
            lhs: None,
            rhs: None,
            cond: Some(Box::new(cond)),
            then: None,
            els: None,
            init: None,
            inc: None,
        }
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
    next: Option<Box<Lvar>>,
    name: String,
    pub offset: usize,
}

impl Lvar {
    pub fn new(next: Lvar, name: impl Into<String>, offset: usize) -> Self {
        Self {
            next: Some(Box::new(next)),
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

    pub fn find_lvar(&self, name: &str) -> Option<&Lvar> {
        let name = name.into();
        if self.name == name {
            return Some(self);
        } else {
            if let Some(x) = &self.next {
                return x.find_lvar(name);
            } else {
                None
            }
        }
    }
}

pub struct Context {
    lvar: Option<Lvar>,
}

impl Context {
    pub fn new() -> Self {
        Self { lvar: None }
    }
}

pub type Program = Vec<Node>;

pub fn program(iter: &mut TokenIter, ctx: &mut Context) -> Result<Program, Error> {
    let mut program = Program::new();
    while iter.peek() != None {
        program.push(stmt(iter, ctx)?);
    }
    Ok(program)
}

pub fn stmt(iter: &mut TokenIter, ctx: &mut Context) -> Result<Node, Error> {
    if let Some(x) = iter.peek() {
        if let TokenKind::KeyWord(key) = x.kind {
            match key {
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
                // KeyWord::Else => {
                //     expr(iter, ctx);
                // }
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
            }
        }
    }
    let node = expr(iter, ctx);
    expect_semi(iter)?;
    node
}

pub fn expr(iter: &mut TokenIter, ctx: &mut Context) -> Result<Node, Error> {
    assign(iter, ctx)
}

pub fn assign(iter: &mut TokenIter, ctx: &mut Context) -> Result<Node, Error> {
    let mut node = equality(iter, ctx)?;
    if consume(iter, Operator::Assign) {
        node = Node::new(Assign, node, assign(iter, ctx)?);
    }
    return Ok(node);
}

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

pub fn unary(iter: &mut TokenIter, ctx: &mut Context) -> Result<Node, Error> {
    if consume(iter, Operator::Plus) {
        return primary(iter, ctx);
    } else if consume(iter, Operator::Minus) {
        return Ok(Node::new(Sub, Node::new_num(0), primary(iter, ctx)?));
    }
    return primary(iter, ctx);
}

pub fn primary(iter: &mut TokenIter, ctx: &mut Context) -> Result<Node, Error> {
    if consume(iter, Operator::LParen) {
        let node = expr(iter, ctx);
        expect(iter, Operator::RParen)?;
        return node;
    }

    if let Some(x) = consume_ident(iter) {
        if ctx.lvar == None {
            let lvar = Lvar::new_leaf(x.name.clone(), 8);
            ctx.lvar = Some(lvar)
        }
        if let Some(lvar) = ctx.lvar.as_ref().unwrap().find_lvar(&x.name) {
            return Ok(Node::new_leaf(Lvar(lvar.clone())));
        } else {
            let lvar = Lvar::new(
                ctx.lvar.as_ref().unwrap().clone(),
                x.name,
                ctx.lvar.as_ref().unwrap().offset + 8,
            );
            ctx.lvar = Some(lvar.clone());
            return Ok(Node::new_leaf(Lvar(lvar)));
        }
    }
    return Ok(Node::new_num(expect_num(iter)?));
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

fn _expect_ident(iter: &mut TokenIter) -> Result<Ident, Error> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr() {
        use crate::token;
        use NodeKind::*;
        // use TokenIter;
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
                        Node::new_leaf(Lvar(super::Lvar::new_leaf("a", 8))),
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
    fn test_program() {
        use crate::token;

        let lvar = Lvar::new_leaf("foo", 8);
        let lhs_f_node = Node::new_leaf(Lvar(lvar.clone()));
        let f_node = Node::new(Assign, lhs_f_node, Node::new_num(10));
        let lhs_s_node = Node::new_leaf(Lvar(Lvar::new(lvar, "bar", 16)));
        let s_node = Node::new(Assign, lhs_s_node, Node::new_num(20));
        let tests = [
            ("a=10;", vec![make_assign_node('a', 10, 8)]),
            ("foo=10;bar=20;", vec![f_node, s_node]),
        ];

        for (s, expected) in &tests {
            let actual = &program(&mut token::tokenize(s), &mut Context::new()).unwrap();
            assert_eq!(expected, actual);
            // for i in 0..expected.len() {
            //     assert_eq!()
            // }
        }
    }

    #[test]
    fn test_if() {
        use crate::token;
        let cond = Node::new(Equal, Node::new_num(10), Node::new_num(20));
        let then = Node::new_unary(Return, Node::new_num(15));
        let expected = vec![make_if_node(cond, then)];

        let input = "if ( 10 ==20 ) return 15;";
        let actual = program(&mut token::tokenize(input), &mut Context::new()).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_if_else() {
        use crate::token;
        let cond = Node::new(Equal, Node::new_num(10), Node::new_num(20));
        let then = Node::new_unary(Return, Node::new_num(15));
        let els = Node::new_unary(Return, Node::new(Add, Node::new_num(10), Node::new_num(30)));
        let expected = vec![make_if_else_node(cond, then, els)];

        let input = "if ( 10 ==20 ) return 15; else return 10+30;";
        let actual = program(&mut token::tokenize(input), &mut Context::new()).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_while() {
        use crate::token;
        // Geqは左右を入れ替えてLeq
        let cond = Node::new(Leq, Node::new_num(20), Node::new_num(32));
        let then = Node::new_unary(Return, Node::new_num(10));
        let expected = vec![make_while_node(cond, then)];

        let input = "while (32 >= 20 ) return 10;";
        let actual = program(&mut token::tokenize(input), &mut Context::new()).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_for() {
        use crate::token;

        let init = make_assign_node("i", 0, 8);
        let cond = Node::new(
            Lesser,
            Node::new_leaf(Lvar(Lvar::new_leaf("i", 8))),
            Node::new_num(10),
        );
        let tmp_inc = Node::new(
            Add,
            Node::new_leaf(Lvar(Lvar::new_leaf("i", 8))),
            Node::new_num(1),
        );
        let inc = Node::new(
            Assign,
            Node::new_leaf(Lvar(Lvar::new_leaf("i", 8))),
            tmp_inc,
        );

        let ret = Node::new(
            Add,
            Node::new_leaf(Lvar(Lvar::new_leaf("i", 8))),
            Node::new_num(2),
        );
        let then = Node::new_unary(Return, ret);

        let expected = vec![make_for_node(Some(init), Some(cond), Some(inc), then)];

        let input = "for(i=0;i<10;i=i+1)return i+2;";
        let actual = program(&mut token::tokenize(input), &mut Context::new()).unwrap();

        assert_eq!(expected, actual);
    }

    fn make_test_node(kind: NodeKind, lhs_num: u64, rhs_num: u64) -> Node {
        Node {
            kind,
            lhs: Some(Box::new(Node::new_num(lhs_num))),
            rhs: Some(Box::new(Node::new_num(rhs_num))),
            cond: None,
            then: None,
            els: None,
            init: None,
            inc: None,
        }
    }

    fn make_assign_node(lhs: impl Into<String>, rhs: u64, offset: usize) -> Node {
        Node {
            kind: Assign,
            lhs: Some(Box::new(Node::new_leaf(Lvar(super::Lvar::new_leaf(
                lhs.into(),
                offset,
            ))))),
            rhs: Some(Box::new(Node::new_num(rhs))),
            cond: None,
            then: None,
            els: None,
            init: None,
            inc: None,
        }
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
}
