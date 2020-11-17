use self::NodeKind::*;
use super::error::Error;
use crate::token::{Operator, TokenIter, TokenKind};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub enum NodeKind {
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
    Num(u64),
}

impl NodeKind {
    /// convert NodeKind to token::Operator
    pub fn as_op(&self) -> Result<Operator, ()> {
        match self {
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
pub struct Node {
    pub kind: NodeKind,
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
}

impl Node {
    pub fn new(kind: NodeKind, lhs: Node, rhs: Node) -> Node {
        Node {
            kind,
            lhs: Some(Box::new(lhs)),
            rhs: Some(Box::new(rhs)),
        }
    }

    pub fn new_num(val: u64) -> Node {
        Node {
            kind: Num(val),
            lhs: None,
            rhs: None,
        }
    }
}

pub fn expr(iter: &mut TokenIter) -> Result<Node, Error> {
    equality(iter)
}

pub fn equality(iter: &mut TokenIter) -> Result<Node, Error> {
    let mut node = relational(iter)?;
    loop {
        if consume(iter, Operator::Equal) {
            node = Node::new(Equal, node, relational(iter)?);
        } else if consume(iter, Operator::Neq) {
            node = Node::new(Neq, node, relational(iter)?);
        } else {
            return Ok(node);
        }
    }
}

pub fn relational(iter: &mut TokenIter) -> Result<Node, Error> {
    let mut node = add(iter)?;
    loop {
        if consume(iter, Operator::Lesser) {
            node = Node::new(Lesser, node, add(iter)?);
        } else if consume(iter, Operator::Leq) {
            node = Node::new(Leq, node, add(iter)?);
        } else if consume(iter, Operator::Greater) {
            // 左右を入れ替えて読み変える
            node = Node::new(Lesser, add(iter)?, node);
        } else if consume(iter, Operator::Geq) {
            // 左右を入れ替えて読み変える
            node = Node::new(Leq, add(iter)?, node);
        } else {
            return Ok(node);
        }
    }
}

pub fn add(iter: &mut TokenIter) -> Result<Node, Error> {
    let mut node = mul(iter)?;
    loop {
        if consume(iter, Operator::Plus) {
            node = Node::new(Add, node, mul(iter)?)
        } else if consume(iter, Operator::Minus) {
            node = Node::new(Sub, node, mul(iter)?)
        } else {
            return Ok(node);
        }
    }
}

pub fn mul(iter: &mut TokenIter) -> Result<Node, Error> {
    let mut node = unary(iter)?;
    loop {
        if consume(iter, Operator::Mul) {
            node = Node::new(Mul, node, unary(iter)?)
        } else if consume(iter, Operator::Div) {
            node = Node::new(Div, node, unary(iter)?)
        } else {
            return Ok(node);
        }
    }
}

pub fn unary(iter: &mut TokenIter) -> Result<Node, Error> {
    if consume(iter, Operator::Plus) {
        return primary(iter);
    } else if consume(iter, Operator::Minus) {
        return Ok(Node::new(Sub, Node::new_num(0), primary(iter)?));
    }
    return primary(iter);
}

pub fn primary(iter: &mut TokenIter) -> Result<Node, Error> {
    if consume(iter, Operator::LParen) {
        let node = expr(iter);
        expect(iter, Operator::RParen)?;
        return node;
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
    return Err(Error::eof(iter.s, iter.pos, TokenKind::Num(0), None));
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