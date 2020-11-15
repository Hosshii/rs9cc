use self::NodeKind::*;
use crate::token::{Operator, Token, TokenKind};
use std::iter::Peekable;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub enum AstError {
    Expr,
    Mul,
    Primary,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub enum NodeKind {
    Add,
    Sub,
    Mul,
    Div,
    Num(u64),
}

impl NodeKind {
    pub fn as_op(&self) -> Result<Operator, ()> {
        match self {
            Add => Ok(Operator::Plus),
            Sub => Ok(Operator::Minus),
            Mul => Ok(Operator::Mul),
            Div => Ok(Operator::Div),
            _ => Err(()),
        }
    }

    pub fn from_op(op: Operator) -> Result<NodeKind, ()> {
        match op {
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
    kind: NodeKind,
    lhs: Option<Box<Node>>,
    rhs: Option<Box<Node>>,
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

pub fn expr<T>(iter: &mut Peekable<T>) -> Result<Node, AstError>
where
    T: Iterator<Item = Token>,
{
    let mut node = mul(iter)?;

    while let Some(token) = iter.peek() {
        match token.kind {
            TokenKind::Reserved(op) => match op {
                Operator::Plus => {
                    iter.next();
                    node = Node::new(Add, node, mul(iter)?)
                }
                Operator::Minus => {
                    iter.next();
                    node = Node::new(Sub, node, mul(iter)?)
                }
                _ => return Ok(node),
            },
            _ => return Ok(node),
        }
    }
    Ok(node)
}

pub fn mul<T>(iter: &mut Peekable<T>) -> Result<Node, AstError>
where
    T: Iterator<Item = Token>,
{
    let mut node = primary(iter)?;
    while let Some(token) = iter.peek() {
        match token.kind {
            TokenKind::Reserved(op) => match op {
                Operator::Mul => {
                    iter.next();
                    node = Node::new(Mul, node, primary(iter)?)
                }
                Operator::Div => {
                    iter.next();
                    node = Node::new(Div, node, primary(iter)?)
                }
                _ => return Ok(node),
            },
            _ => return Ok(node),
        }
    }
    Ok(node)
}

pub fn primary<T>(iter: &mut Peekable<T>) -> Result<Node, AstError>
where
    T: Iterator<Item = Token>,
{
    // Node::new_num(1)
    while let Some(token) = iter.peek() {
        match token.kind {
            TokenKind::Reserved(op) => match op {
                Operator::LParen => {
                    iter.next();
                    let node = expr(iter);
                    if let Some(x) = iter.next() {
                        if x.kind == TokenKind::Reserved(Operator::RParen) {
                            return node;
                        }
                    }
                }
                _ => return Err(AstError::Primary),
            },
            TokenKind::Num(x) => {
                return {
                    iter.next();
                    Ok(Node::new_num(x))
                }
            }
            x => {
                println!("err {:?}", x);
                return Err(AstError::Primary);
            }
        }
    }
    Err(AstError::Primary)
}
