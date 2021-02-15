use self::NodeKind::*;

use crate::base_types;
use crate::base_types::{Member, TagTypeKind, TypeKind};
use crate::token::Operator;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum NodeKind {
    Assign,
    AAdd,
    ASub,
    AMul,
    ADiv,
    ALShift,
    ARShift,
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
    Do,
    For,
    Addr,
    Deref,
    Block(Vec<Node>),
    Func(Rc<FuncPrototype>, Vec<Node>), // (func_name,args)
    Num(i64),
    // Ident(Ident),
    Lvar(Rc<RefCell<Lvar>>), // usize はベースポインタからのオフセット
    TypeKind(TypeKind),
    Declaration(Declaration),
    Gvar(Rc<Gvar>),
    TkString(Rc<String>),
    Char(char),
    StmtExpr(Vec<Node>),
    ExprStmt,
    Member(Ident, Rc<Member>), // member name, offset
    Cast(TypeKind),
    Comma,
    PreInc,
    PreDec,
    PostInc,
    PostDec,
    Not,
    BitNot,
    BitOr,
    BitAnd,
    BitXor,
    LogOr,
    LogAnd,
    Break,
    Continue,
    Goto(Ident),
    Label(Ident),
    Switch(Vec<Node>),
    Case(i64),
    DefaultCase,
    LShift,
    RShift,
    Ternary,
    Null,
}

impl NodeKind {
    // todo
    // String is ok?
    pub fn as_str(&self) -> String {
        match self {
            Assign => "=".to_string(),
            AAdd => "+=".to_string(),
            ASub => "-=".to_string(),
            AMul => "*=".to_string(),
            ADiv => "/=".to_string(),
            ALShift => "<<=".to_string(),
            ARShift => ">>=".to_string(),
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
            Do => "do".to_string(),
            For => "for".to_string(),
            Addr => "&".to_string(),
            Deref => "*".to_string(),
            Block(_) => "block".to_string(),
            Func(func_prototype, _) => format!("function: {}", func_prototype.ident.name), // (func_name,args)
            Num(num) => format!("{}", num),
            // Ident(Ident),
            Lvar(lvar) => format!("{:?}", lvar), // usize はベースポインタからのオフセット
            TypeKind(type_kind) => format!("{}", type_kind),
            Declaration(dec) => format!("{:?}", dec),
            Gvar(x) => format!("{:?}", x),
            TkString(string) => string.to_string(),
            Char(c) => c.to_string(),
            StmtExpr(_) => "stmt expr".to_string(),
            ExprStmt => "expression statement".to_string(),
            Member(_, _) => "member".to_string(),
            Cast(_) => "cast".to_string(),
            Comma => ",".to_string(),
            PreInc => "pre ++".to_string(),
            PreDec => "pre --".to_string(),
            PostInc => "post ++".to_string(),
            PostDec => "post --".to_string(),
            Not => "!".to_string(),
            BitNot => "~".to_string(),
            BitOr => "|".to_string(),
            BitAnd => "&".to_string(),
            BitXor => "^".to_string(),
            LogOr => "||".to_string(),
            LogAnd => "&&".to_string(),
            Break => "break".to_string(),
            Continue => "continue".to_string(),
            Goto(_) => "goto".to_string(),
            Label(_) => "label".to_string(),
            Switch(_) => "switch".to_string(),
            Case(num) => format!("case{}", num).to_string(),
            DefaultCase => "default case".to_string(),
            LShift => "<<".to_string(),
            RShift => ">>".to_string(),
            Ternary => "ternary".to_string(),
            Null => "null".to_string(),
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

const IDENT_ANONYMOUS: &str = ".ident.anonymous";

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct Ident {
    pub name: String,
}

impl From<crate::token::Ident> for Ident {
    fn from(ident: crate::token::Ident) -> Self {
        Ident::new(ident.name)
    }
}

impl Ident {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    pub fn new_anonymous() -> Self {
        Self {
            name: IDENT_ANONYMOUS.to_string(),
        }
    }

    pub fn is_anonymous(&self) -> bool {
        self.name == IDENT_ANONYMOUS
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct Node {
    pub kind: NodeKind,
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
    pub cond: Option<Box<Node>>,
    pub then: Option<Box<Node>>,
    pub els: Option<Box<Node>>,
    pub init: Option<Vec<Node>>,
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
        init: Option<Vec<Node>>,
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

    pub fn new_num(val: i64) -> Node {
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

    pub fn new_init(kind: NodeKind, init: Vec<Node>) -> Node {
        let mut node = Node::new_leaf(kind);
        node.init = Some(init);
        node
    }

    pub fn new_expr_stmt(node: Node) -> Node {
        Node::new_unary(ExprStmt, node)
    }

    pub fn new_assign(lhs: Node, rhs: Node) -> Node {
        Node::new(Assign, lhs, rhs)
    }

    pub fn new_lvar(lvar: Rc<RefCell<Lvar>>) -> Node {
        Node::new_leaf(NodeKind::Lvar(lvar))
    }

    pub fn new_gvar(gvar: Rc<Gvar>) -> Node {
        Node::new_leaf(NodeKind::Gvar(gvar))
    }

    pub fn new_var(var: Var) -> Node {
        use Var::*;
        match var {
            L(lvar) => Node::new_lvar(lvar.clone()), // todo
            G(gvar) => Node::new_gvar(gvar),
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

    pub fn get_type(&self) -> Result<TypeKind, &'static str> {
        match &self.kind {
            Assign | Add | Sub | Mul | Div | PostDec | PostInc | PreDec | PreInc | AAdd | ASub
            | AMul | ADiv | Equal | Neq | Lesser | Leq | Greater | Geq => {
                if let Some(ref x) = self.lhs {
                    x.get_type()
                } else {
                    Err("assign add sub mul")
                }
            }
            Deref => {
                if let Some(ref lhs) = self.lhs {
                    Ok(lhs.get_type()?.get_deref_type().borrow().clone())
                } else {
                    Err("deref")
                }
            }
            Addr => {
                if let Some(ref lhs) = self.lhs {
                    Ok(lhs.get_type()?.get_addr_type().borrow().clone())
                } else {
                    Err("addr")
                }
            }
            Lvar(lvar) => Ok(lvar.borrow().get_type()),
            Gvar(gvar) => Ok(gvar.get_type()),
            Func(func_prototype, _) => Ok(func_prototype.type_kind.clone()),
            Num(num) => {
                if num > &(i32::MAX as i64) {
                    Ok(TypeKind::Long)
                } else {
                    Ok(TypeKind::Int)
                }
            }
            ExprStmt => {
                if let Some(ref lhs) = self.lhs {
                    Ok(lhs.get_type()?)
                } else {
                    Err("expr stmt")
                }
            }
            Member(_, member) => Ok(member.get_type().as_ref().clone()), // todo他のところもrcにしていく
            Cast(type_kind) => Ok(type_kind.clone()),
            _ => Err("err"),
        }
    }

    /// get gvar
    /// if NodeKind is ptr or addr, recursively search
    pub fn get_gvar(&self) -> Result<Rc<Gvar>, &'static str> {
        match &self.kind {
            Gvar(gvar) => Ok(gvar.clone()),
            Addr | Deref => {
                if let Some(ref lhs) = self.lhs {
                    Ok(lhs.get_gvar()?)
                } else {
                    Err("addr")
                }
            }
            _ => Err("not gvar"),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum Var {
    L(Rc<RefCell<Lvar>>),
    G(Rc<Gvar>),
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct Lvar {
    next: Option<Rc<RefCell<Lvar>>>,
    pub dec: Declaration,
    pub offset: u64,
}

impl Lvar {
    pub fn new(next: Lvar, dec: Declaration, offset: u64) -> Self {
        Self {
            next: Some(Rc::new(RefCell::new(next))),
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct Gvar {
    pub dec: Declaration,
    pub size: u64,
    pub init: Vec<Initializer>,
}

impl Gvar {
    pub fn new(dec: Declaration, size: u64, init: Vec<Initializer>) -> Self {
        Self { dec, size, init }
    }

    fn get_type(&self) -> TypeKind {
        self.dec.get_type()
    }
}

pub type GvarMp = HashMap<String, Rc<Gvar>>;

#[derive(Clone, Debug)]
pub struct Context {
    pub g: GlobalContext,
    pub l: LocalContext,
    pub s: Scope,
    pub static_counter: u32,
    pub cur_switch: Option<Vec<Node>>,
    pub cur_default: Option<Node>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            g: GlobalContext::new(),
            l: LocalContext::new(),
            s: Scope::new(),
            static_counter: 0,
            cur_switch: None,
            cur_default: None,
        }
    }

    pub fn push_front(&mut self, dec: Declaration, offset: u64) {
        self.l.push_front(dec.clone(), offset);
        let lvar = self.l.find_lvar(dec.ident.name.clone()).unwrap();
        self.push_scope(dec.ident, Rc::new(Var::L(lvar)));
    }

    pub fn push_scope(&mut self, ident: Ident, var: Rc<Var>) {
        self.s.insert_v(ident, var);
    }

    pub fn insert_g(&mut self, gvar: Rc<Gvar>) {
        // let size = dec.type_kind.size();
        // let ident = dec.ident.name.clone();
        // let gvar = Rc::new(Gvar::new(dec, size, init));
        self.g
            .gvar_mp
            .insert(gvar.dec.ident.name.clone(), gvar.clone());
        self.s
            .insert_v(gvar.dec.ident.clone(), Rc::new(Var::G(gvar)));
    }

    pub fn make_label(&mut self) -> String {
        self.static_counter += 1;
        format!(".static.data{}", self.static_counter)
    }
}

#[derive(Clone, Debug)]
pub struct GlobalContext {
    pub gvar_mp: GvarMp,
    pub func_prototype_mp: FuncPrototypeMp,
    pub tk_string: Vec<(Rc<String>, Rc<String>)>, // content, label
}

impl GlobalContext {
    pub fn new() -> Self {
        Self {
            gvar_mp: HashMap::new(),
            func_prototype_mp: HashMap::new(),
            tk_string: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct LocalContext {
    pub lvar: Option<Rc<RefCell<Lvar>>>,
    pub(crate) lvar_count: usize,
}

impl LocalContext {
    pub fn new() -> Self {
        Self {
            lvar: None,
            lvar_count: 0,
        }
    }

    pub fn push_front(&mut self, dec: Declaration, offset: u64) {
        self.lvar_count += 1;
        let offset = offset + dec.type_kind.size();
        let offset = base_types::align_to(offset, dec.type_kind.align());
        self.lvar = Some(Rc::new(RefCell::new(Lvar {
            next: self.lvar.take(),
            dec,
            offset,
        })))
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

    pub fn find_lvar(&self, name: impl Into<String>) -> Option<Rc<RefCell<Lvar>>> {
        if let Some(ref lvar) = self.lvar {
            Self::_find_lvar(lvar, name)
        } else {
            None
        }
    }

    fn _find_lvar(lvar: &Rc<RefCell<Lvar>>, name: impl Into<String>) -> Option<Rc<RefCell<Lvar>>> {
        let name = name.into();
        if lvar.borrow().dec.ident.name == name {
            Some(lvar.clone())
        } else {
            if let Some(ref next) = lvar.borrow().next {
                Self::_find_lvar(next, name)
            } else {
                None
            }
        }
    }
}

type VarMp = HashMap<(Ident, usize), Rc<Var>>;
type TagMp = HashMap<(Rc<Ident>, usize), Rc<TagTypeKind>>;
#[derive(Clone, Debug)]
pub struct Scope {
    v: VarMp,
    t: TagMp,
    depth: usize,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            v: HashMap::new(),
            t: HashMap::new(),
            depth: 0,
        }
    }

    pub fn enter(&mut self) -> Self {
        let tmp = self.clone();
        self.depth += 1;
        tmp
    }

    pub fn leave(&mut self, sc: Scope) {
        *self = sc;
    }

    pub fn find_cur_lvar(&self, ident: Ident) -> Option<Rc<RefCell<Lvar>>> {
        if let Some(vr) = self.v.get(&(ident, self.depth)) {
            match vr.as_ref() {
                Var::L(lvar) => Some(lvar.clone()),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn find_upper_lvar(&self, ident: Ident) -> Option<Rc<RefCell<Lvar>>> {
        self.find_lvar(ident, self.depth)
    }

    /// depthとそれより浅い範囲にある物を探す。深いものからみつかる
    pub fn find_lvar(&self, ident: Ident, depth: usize) -> Option<Rc<RefCell<Lvar>>> {
        for i in (0..=depth).rev() {
            if let Some(vr) = self.v.get(&(ident.clone(), i)) {
                // todo performance
                match vr.as_ref() {
                    Var::L(lvar) => return Some(lvar.clone()),
                    _ => (),
                }
            }
        }
        None
    }

    pub fn find_cur_gvar(&self, ident: Ident) -> Option<Rc<Gvar>> {
        if let Some(vr) = self.v.get(&(ident, self.depth)) {
            match vr.as_ref() {
                Var::G(gvar) => Some(gvar.clone()),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn find_upper_gvar(&self, ident: Ident) -> Option<Rc<Gvar>> {
        self.find_gvar(ident, self.depth)
    }

    pub fn find_gvar(&self, ident: Ident, depth: usize) -> Option<Rc<Gvar>> {
        for i in (0..=depth).rev() {
            if let Some(vr) = self.v.get(&(ident.clone(), i)) {
                match vr.as_ref() {
                    Var::G(gvar) => return Some(gvar.clone()),
                    _ => (),
                }
            }
        }
        None
    }

    pub fn insert_v(&mut self, ident: Ident, vr: Rc<Var>) -> Option<Rc<Var>> {
        self.v.insert((ident, self.depth), vr)
    }

    pub fn find_cur_tag(&self, ident: Rc<Ident>) -> Option<Rc<TagTypeKind>> {
        self.t.get(&(ident, self.depth)).map(|v| v.clone())
    }

    pub fn find_upper_tag(&self, ident: Rc<Ident>) -> Option<Rc<TagTypeKind>> {
        self.find_tag(ident, self.depth)
    }

    pub fn find_tag(&self, ident: Rc<Ident>, depth: usize) -> Option<Rc<TagTypeKind>> {
        for i in (0..=depth).rev() {
            match self.t.get(&(ident.clone(), i)).map(|v| v.clone()) {
                Some(x) => return Some(x),
                _ => (),
            }
        }
        None
    }

    pub fn find_upper_struct(&self, ident: Rc<Ident>) -> Option<Rc<TagTypeKind>> {
        if let Some(tag_type_kind) = self.find_upper_tag(ident) {
            if let TagTypeKind::Struct(_) = tag_type_kind.as_ref() {
                return Some(tag_type_kind);
            }
        }
        None
    }

    pub fn insert_t(&mut self, ident: Rc<Ident>, tag: TagTypeKind) -> Option<Rc<TagTypeKind>> {
        self.t.insert((ident, self.depth), Rc::new(tag))
    }
}

#[derive(Clone, Debug)]
pub struct Program {
    pub ctx: Context,
    pub functions: Vec<Function>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            ctx: Context::new(),
            functions: Vec::new(),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct Function {
    pub def: Rc<FuncPrototype>,
    pub all_vars: Option<Rc<RefCell<Lvar>>>,
    pub all_var_num: usize,
    pub nodes: Vec<Node>,
    pub is_static: bool,
    pub va_area: Option<Lvar>,
}

impl From<Function> for FuncPrototype {
    fn from(from: Function) -> FuncPrototype {
        FuncPrototype {
            type_kind: from.def.type_kind.clone(),
            ident: from.def.ident.clone(),
            params: from.def.params.clone(),
            param_num: from.def.param_num,
            is_static: from.is_static,
        }
    }
}

impl Function {
    pub fn new(
        def: Rc<FuncPrototype>,
        all_vars: Option<Rc<RefCell<Lvar>>>,
        all_var_num: usize,
        nodes: Vec<Node>,
        is_static: bool,
        va_area: Option<Lvar>,
    ) -> Self {
        Self {
            def,
            all_vars,
            all_var_num,
            nodes,
            is_static,
            va_area: va_area,
        }
    }

    /// パディング込みでのサイズを計算する
    /// 最後には8バイト境界になるようにパディングが追加される
    pub fn get_all_var_size(&self) -> u64 {
        if let Some(ref lvar) = self.all_vars {
            let size = lvar.borrow().offset;
            let size = base_types::align_to(size, 8);
            return size;
        } else {
            0
        }
    }

    pub fn get_param_size(&self) -> u64 {
        let mut result = 0;
        for dec in &self.def.params {
            result += dec.type_kind.size();
        }
        result
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct FuncPrototype {
    pub type_kind: TypeKind,
    pub ident: Ident,
    pub params: Vec<Declaration>,
    pub param_num: usize,
    pub is_static: bool,
}

impl FuncPrototype {
    pub fn new(
        type_kind: TypeKind,
        ident: Ident,
        params: Vec<Declaration>,
        is_static: bool,
    ) -> Self {
        let param_num = params.len();
        Self {
            type_kind,
            ident,
            params,
            param_num,
            is_static,
        }
    }
}
pub type FuncPrototypeMp = HashMap<String, Rc<FuncPrototype>>;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct Declaration {
    pub type_kind: TypeKind,
    pub ident: Ident,
    pub is_typedef: bool,
    pub is_static: bool,
    pub is_extern: bool,
    pub is_const: (bool, i64), // for enum
}

impl Declaration {
    pub fn new(type_kind: TypeKind, ident: Ident) -> Self {
        Self {
            type_kind,
            ident,
            is_typedef: false,
            is_static: false,
            is_extern: false,
            is_const: (false, 0),
        }
    }

    pub fn new_const(type_kind: TypeKind, ident: Ident, val: i64) -> Self {
        Self {
            type_kind,
            ident,
            is_typedef: false,
            is_static: false,
            is_extern: false,
            is_const: (true, val),
        }
    }

    // todo: remove clone
    fn get_type(&self) -> TypeKind {
        self.type_kind.clone()
    }
}
#[derive(Debug, Clone)]
pub struct Designator {
    pub idx: u64,
    pub next: Option<Box<Designator>>,
    pub member: Option<Rc<Member>>,
}

impl Designator {
    pub fn new(idx: u64, next: Option<Box<Designator>>, member: Option<Rc<Member>>) -> Self {
        Self { idx, next, member }
    }
}

/// global var initializer
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum Initializer {
    Val(u64, i64),      // size, value
    Label(String, i64), // pointer, addend
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_type() {
        use crate::ast::ast;
        use crate::base_types::TypeKind;
        use crate::token;
        use std::cell::RefCell;

        let input = Rc::new("&*1;".to_string());
        let mut ctx = Context::new();
        let mut token_stream =
            token::tokenize(Rc::new(input.to_string()), Rc::new("".to_string())).unwrap();
        let node = ast::stmt(&mut token_stream, &mut ctx).unwrap();
        assert_eq!(TypeKind::Int, node.get_type().unwrap());

        let input = "*(y + 1);";
        let mut ctx = Context::new();
        ctx.push_front(
            Declaration::new(
                TypeKind::Ptr(Rc::new(RefCell::new(TypeKind::Int))),
                Ident::new("y"),
            ),
            8,
        );
        let node = ast::stmt(
            &mut token::tokenize(Rc::new(input.to_string()), Rc::new("".to_string())).unwrap(),
            &mut ctx,
        )
        .unwrap();
        assert_eq!(TypeKind::Int, node.get_type().unwrap())
    }
}
