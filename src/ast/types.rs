use self::NodeKind::*;

use crate::base_types;
use crate::base_types::TypeKind;
use crate::token::Operator;
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
    Addr,
    Deref,
    Block(Vec<Node>),
    Func(String, Vec<Node>), // (func_name,args)
    Num(u64),
    // Ident(Ident),
    Lvar(Rc<Lvar>), // usize はベースポインタからのオフセット
    BaseType(base_types::BaseType),
    Declaration(Declaration),
}

impl NodeKind {
    // todo
    // String is ok?
    pub fn as_str(&self) -> String {
        match self {
            Assign => "=".to_string(),
            Equal => "==".to_string(),
            Neq => "!=".to_string(),
            Lesser => "<".to_string(),
            Leq => "<=".to_string(),
            Greater => ">".to_string(),
            Geq => ">=".to_string(),
            Add => "+".to_string(),
            Sub => "-".to_string(),
            Mul => "*".to_string(),
            Div => "/".to_string(),
            Return => "return".to_string(),
            If => "if".to_string(),
            Else => "else".to_string(),
            While => "while".to_string(),
            For => "for".to_string(),
            Addr => "&".to_string(),
            Deref => "*".to_string(),
            Block(_) => "block".to_string(),
            Func(name, _) => format!("function: {}", name), // (func_name,args)
            Num(num) => format!("{}", num),
            // Ident(Ident),
            Lvar(lvar) => format!("{:?}", lvar), // usize はベースポインタからのオフセット
            BaseType(b_type) => format!("{}", b_type.kind),
            Declaration(dec) => format!("{:?}", dec),
        }
    }
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
    pub name: String,
}

impl Ident {
    pub fn new(name: impl Into<String>) -> Self {
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

    pub fn get_type(&self) -> Result<TypeKind, &'static str> {
        match &self.kind {
            Assign | Add | Sub | Mul | Div => {
                if let Some(ref x) = self.lhs {
                    x.get_type()
                } else {
                    Err("assign add sub mul")
                }
            }
            Deref => {
                if let Some(ref lhs) = self.lhs {
                    Ok(lhs.get_type()?.get_deref_type())
                } else {
                    Err("deref")
                }
            }
            Addr => {
                if let Some(ref lhs) = self.lhs {
                    Ok(lhs.get_type()?.get_addr_type())
                } else {
                    Err("addr")
                }
            }
            Lvar(lvar) => Ok(lvar.get_type()),
            Num(_) => Ok(TypeKind::Int),
            _ => Err("err"),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct Lvar {
    next: Option<Rc<Lvar>>,
    pub dec: Declaration,
    pub offset: u64,
}

impl Lvar {
    pub fn new(next: Lvar, dec: Declaration, offset: u64) -> Self {
        Self {
            next: Some(Rc::new(next)),
            dec,
            offset,
        }
    }

    pub fn new_leaf(dec: Declaration, offset: u64) -> Self {
        Self {
            next: None,
            dec,
            offset,
        }
    }

    fn get_type(&self) -> TypeKind {
        self.dec.get_type()
    }
}

pub struct Context {
    pub lvar: Option<Rc<Lvar>>,
    pub(crate) count: usize,
}

impl Context {
    pub fn new() -> Self {
        Self {
            lvar: None,
            count: 0,
        }
    }

    pub fn push_front(&mut self, dec: Declaration, offset: u64) {
        self.count += 1;
        let offset = offset + dec.base_type.kind.eight_size();
        self.lvar = Some(Rc::new(Lvar {
            next: self.lvar.take(),
            dec,
            offset,
        }))
    }

    // pub fn push_front_param(&mut self, dec: Declaration, offset: u64) {
    //     self.count += 1;
    //     let offset = offset + dec.base_type.kind.eight_size();
    //     self.lvar = Some(Rc::new(Lvar {
    //         next: self.lvar.take(),
    //         dec,
    //         offset,
    //     }))
    // }

    pub fn find_lvar(&self, name: impl Into<String>) -> Option<Rc<Lvar>> {
        if let Some(ref lvar) = self.lvar {
            Self::_find_lvar(lvar, name)
        } else {
            None
        }
    }

    fn _find_lvar(lvar: &Rc<Lvar>, name: impl Into<String>) -> Option<Rc<Lvar>> {
        let name = name.into();
        if lvar.dec.ident.name == name {
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
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct Function {
    pub type_kind: TypeKind,
    pub name: String,
    pub all_vars: Option<Rc<Lvar>>,
    pub all_var_num: usize,
    pub params: Vec<Declaration>,
    pub param_num: usize,
    pub nodes: Vec<Node>,
}

impl Function {
    pub fn new(
        type_kind: TypeKind,
        name: impl Into<String>,
        all_vars: Option<Rc<Lvar>>,
        all_var_num: usize,
        params: Vec<Declaration>,
        param_num: usize,
        nodes: Vec<Node>,
    ) -> Self {
        Self {
            type_kind,
            name: name.into(),
            all_vars,
            all_var_num,
            params,
            param_num,
            nodes,
        }
    }

    /// return all variable size  
    /// `int: 4`  
    /// `ptr: 8`  
    /// `int x[10]: 4*10 = 40`
    pub fn get_all_var_size(&self) -> u64 {
        let mut result = 0;
        let mut lvar_ref = &self.all_vars;
        while let Some(ref lvar) = lvar_ref {
            result += lvar.dec.base_type.kind.size();
            lvar_ref = &lvar.next;
        }
        result
    }

    /// 配列以外は8バイト以下は8バイトにする
    pub fn _get_all_var_size(&self) -> u64 {
        let mut result = 0;
        let mut lvar_ref = &self.all_vars;

        while let Some(ref lvar) = lvar_ref {
            result += lvar.dec.base_type.kind.eight_size();
            lvar_ref = &lvar.next;
        }
        result
    }

    pub fn get_param_size(&self) -> u64 {
        let mut result = 0;
        for dec in &self.params {
            result += dec.base_type.kind.size();
        }
        result
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct Declaration {
    pub base_type: base_types::BaseType,
    pub ident: Ident,
}

impl Declaration {
    pub fn new(base_type: base_types::BaseType, ident: Ident) -> Self {
        Self { base_type, ident }
    }

    // todo: remove clone
    fn get_type(&self) -> TypeKind {
        self.base_type.kind.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_type() {
        use crate::ast::ast;
        use crate::base_types::{BaseType, TypeKind};
        use crate::token;

        let input = "&*1;";
        let node = ast::stmt(&mut token::tokenize(input), &mut Context::new()).unwrap();
        assert_eq!(TypeKind::Int, node.get_type().unwrap());

        let input = "*(y + 1);";
        let mut ctx = Context::new();
        ctx.push_front(
            Declaration::new(
                BaseType::new(TypeKind::Ptr(Rc::new(BaseType::new(TypeKind::Int)))),
                Ident::new("y"),
            ),
            8,
        );
        let node = ast::stmt(&mut token::tokenize(input), &mut ctx).unwrap();
        assert_eq!(TypeKind::Int, node.get_type().unwrap())
    }
}
