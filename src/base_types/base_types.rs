use self::TypeKind::*;
use crate::ast::{Declaration, Ident};
use std::collections::HashMap;
use std::fmt;
use std::{cell::RefCell, rc::Rc};

// https://ja.wikipedia.org/wiki/データ構造アライメント#パディングの計算
pub fn align_to(offset: u64, align: u64) -> u64 {
    (offset + (align - 1)) & !(align - 1)
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct Member {
    pub type_kind: Rc<TypeKind>,
    pub offset: u64,
    pub ident: Ident,
}

impl fmt::Display for Member {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.ident.name, self.type_kind)
    }
}

impl Member {
    pub fn new(type_kind: Rc<TypeKind>, offset: u64, ident: Ident) -> Self {
        Self {
            type_kind,
            offset,
            ident,
        }
    }

    pub fn get_type(&self) -> Rc<TypeKind> {
        self.type_kind.clone()
    }
}

#[derive(Debug, Clone)]
pub enum TagTypeKind {
    Struct(Rc<Struct>),
    Enum,
    Typedef(Rc<Declaration>),
}

#[derive(Debug, Clone)]
pub struct TagContext {
    pub tag_list: HashMap<Rc<Ident>, Rc<TagTypeKind>>,
}

impl TagContext {
    pub fn new() -> Self {
        Self {
            tag_list: HashMap::new(),
        }
    }

    pub fn register(&mut self, dec: &Declaration) {
        if let TypeKind::Struct(_struct) = &dec.type_kind {
            if !_struct.is_anonymous {
                self.tag_list.insert(
                    _struct.ident.clone(),
                    Rc::new(TagTypeKind::Struct(_struct.clone())),
                );
            }
        }
    }

    pub fn find_tag(&self, ident: &Ident) -> Option<&Rc<TagTypeKind>> {
        self.tag_list.get(ident)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct Struct {
    ident: Rc<Ident>,
    members: Rc<Vec<Rc<Member>>>,
    is_anonymous: bool,
}

impl Struct {
    pub fn new(ident: Rc<Ident>, members: Rc<Vec<Rc<Member>>>) -> Self {
        Self {
            ident,
            members,
            is_anonymous: false,
        }
    }

    pub fn new_anonymous(members: Rc<Vec<Rc<Member>>>) -> Self {
        Self {
            ident: Rc::new(Ident::new(".struct.anonymous")),
            members,
            is_anonymous: true,
        }
    }

    pub fn find_field(&self, ident: &Ident) -> Option<Rc<Member>> {
        for member in &*self.members {
            if &member.ident == ident {
                return Some(member.clone());
            }
        }
        None
    }

    pub fn get_size(&self) -> u64 {
        if self.members.len() < 1 {
            return 0;
        }
        let last = self.members.last().unwrap();
        let mut size = last.offset + last.type_kind.size();
        size += (8 - size % 8) % 8;
        size
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum TypeKind {
    Char,
    Short,
    Int,
    Long,
    Ptr(Rc<RefCell<TypeKind>>),
    Array(u64, Rc<RefCell<TypeKind>>, bool), // bool is whether initialized or not
    Struct(Rc<Struct>),

    PlaceHolder, // virtual type
    /// this is virtual type for `get_deref_type`
    _Deref(Rc<RefCell<TypeKind>>),
    /// this is for err msg
    _Invalid(String),
}

// impl Default for TypeKind {
//     fn default() -> Self {
//         TypeKind::Int
//     }
// }

impl fmt::Display for TypeKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Char | Short | Int | Long => write!(f, "{}", self.as_str()),
            Ptr(x) => {
                let (count, type_kind) = x.borrow().count_deref();
                let ptr = format!("{:*<width$}", "*", width = count + 1);
                write!(f, "{} {}", type_kind.as_str(), ptr)
            }
            Array(size, type_kind, _) => write!(f, "{} [{}]", type_kind.borrow(), size),
            Struct(s) => {
                for member in &*s.members {
                    writeln!(f, "{}", member)?
                }
                Ok(())
            }
            _Deref(x) => {
                // todo
                // もう少しいい表示考える
                let (count, type_kind) = x.borrow().count_deref();
                let ptr = format!("{:*<width$}", "*", width = count + 1);
                write!(f, "{} {}", type_kind.as_str(), ptr)
            }
            _Invalid(msg) => write!(f, "{}", msg),
            PlaceHolder => write!(f, "placeholder"),
        }
    }
}

impl Default for TypeKind {
    fn default() -> Self {
        TypeKind::Int
    }
}

impl TypeKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Char => "char",
            Short => "short",
            Int => "int",
            Long => "long",
            Ptr(_) => "Ptr",
            Array(_, _, _) => "Array",
            Struct(_) => "struct",
            _ => unreachable!(),
        }
    }

    pub fn from_starts(s: &str) -> Result<TypeKind, ()> {
        match s {
            x if x.starts_with(Char.as_str()) => Ok(Char),
            x if x.starts_with(Short.as_str()) => Ok(Short),
            x if x.starts_with(Int.as_str()) => Ok(Int),
            x if x.starts_with(Long.as_str()) => Ok(Long),
            _ => Err(()),
        }
    }

    pub fn size(&self) -> u64 {
        match self {
            Char => 1,
            Short => 2,
            Int => 4,
            Long => 8,
            Ptr(_) => 8,
            Array(size, type_kind, _) => size * type_kind.borrow().size(),
            Struct(s) => s.get_size(),
            _ => unreachable!(),
        }
    }

    pub fn align(&self) -> u64 {
        match self {
            Char => 1,
            Short => 2,
            Int => 4,
            Long => 8,
            Ptr(_) => 8,
            Array(_, type_kind, _) => type_kind.borrow().align(),
            Struct(_) => 8,
            _ => unreachable!(),
        }
    }

    fn is_num_type(&self) -> bool {
        match self {
            Char | Short | Int | Long => true,
            _ => false,
        }
    }

    pub fn count_deref(&self) -> (usize, TypeKind) {
        let mut count = 0;

        let mut ref_type_kind = if let TypeKind::Ptr(x) = self {
            x.clone()
        } else {
            return (0, self.clone());
        };

        while let TypeKind::Ptr(ref x) = ref_type_kind.clone().borrow().clone() {
            count += 1;
            ref_type_kind = x.clone();
        }
        return (count, ref_type_kind.borrow().clone());
    }

    pub fn get_deref_type(&self) -> Rc<RefCell<Self>> {
        match self {
            Char | Short | Int | Long => Rc::new(RefCell::new(TypeKind::_Deref(Rc::new(
                RefCell::new(self.clone()),
            )))),
            Ptr(type_kind) => type_kind.clone(),
            Array(_, type_kind, _) => type_kind.clone(),
            Struct(_) => Rc::new(RefCell::new(TypeKind::_Deref(Rc::new(RefCell::new(
                self.clone(),
            ))))),
            _Deref(type_kind) => type_kind.clone(),
            _Invalid(msg) => Rc::new(RefCell::new(_Invalid(msg.clone()))),
            PlaceHolder => Rc::new(RefCell::new(PlaceHolder)),
        }
    }

    pub fn get_addr_type(&self) -> Rc<RefCell<Self>> {
        match self {
            _Deref(type_kind) => type_kind.clone(),
            other => Rc::new(RefCell::new(Ptr(Rc::new(RefCell::new(other.clone()))))),
        }
    }

    /// return multiple of 8
    /// `int: 8 (not 4)`
    /// `ptr: 8`
    /// `int x[10]: 4 * 10 = 40`
    pub fn eight_size(&self) -> u64 {
        match self {
            Char | Short | Int | Long => 8,
            Ptr(_) => 8,
            Array(_size, type_kind, _) => {
                let mut size = _size * type_kind.borrow().size();
                size += (8 - size % 8) % 8; // sizeを8の倍数にする
                size
            }
            Struct(_) => self.size(), // structは8の倍数にのともとパディングしてる
            _ => unreachable!(),
        }
    }

    pub fn to_arr(&mut self, size: u64, sized: bool) {
        // unsafeをまだよくわかってないのでmem::replaceを使ってやる

        // unsafe {
        //     std::ptr::write(
        //         self,
        //         Array(size, Rc::new(BaseType::new(std::ptr::read(self)))),
        //     )
        // }

        *self = Array(
            size,
            Rc::new(RefCell::new(std::mem::replace(self, Int))),
            sized,
        );

        // use std::mem::take instead of std::mem::replace
        // *self = Array(size, Rc::new(BaseType::new(std::mem::take(self))));
    }

    pub fn array_of(size: u64, base: Rc<RefCell<TypeKind>>, initialized: bool) -> TypeKind {
        TypeKind::Array(size, base, initialized)
    }

    /// `int [] == int *`
    /// array and pointer is same
    /// todo
    /// this can not compare `int **` and `int *a[]`
    pub fn partial_comp(lhs: &TypeKind, rhs: &TypeKind) -> bool {
        let a = if let Array(_, b_type, _) = lhs {
            Ptr(b_type.clone())
        } else {
            lhs.clone()
        };

        let b = if let Array(_, b_type, _) = rhs {
            Ptr(b_type.clone())
        } else {
            rhs.clone()
        };

        // if lhs and rhs is Int or Char,
        // return true
        if a.is_num_type() && b.is_num_type() {
            return true;
        }

        a == b
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_size() {
        let tests = [
            (Char, 1),
            (Short, 2),
            (Int, 4),
            (Long, 8),
            (Ptr(Rc::new(RefCell::new(Int))), 8),
            (make_array(5, Char, false), 5),
            (make_array(5, Short, false), 10),
            (make_array(5, Int, false), 20),
            (make_array(5, Long, false), 40),
            (make_array(4, make_array(4, Int, false), false), 64),
            (make_array(5, make_array(5, Int, false), false), 100),
        ];

        for (t, expected) in &tests {
            assert_eq!(t.size(), *expected);
        }
    }

    #[test]
    fn test_eight_size() {
        let tests = [
            (Char, 8),
            (Int, 8),
            (Ptr(Rc::new(RefCell::new(Int))), 8),
            (make_array(4, Char, false), 8),
            (make_array(4, Int, false), 16),
            (make_array(4, make_array(4, Int, false), false), 64),
            (make_array(4, make_array(5, Char, false), false), 24),
        ];

        for (t, expected) in &tests {
            assert_eq!(t.eight_size(), *expected);
        }
    }

    fn make_array(size: u64, type_kind: TypeKind, initialized: bool) -> TypeKind {
        TypeKind::Array(size, Rc::new(RefCell::new(type_kind)), initialized)
    }

    #[test]
    fn test_align_to() {
        let tests = [
            (0, Char, 0),
            (0, Short, 0),
            (0, Int, 0),
            (0, Long, 0),
            (3, Char, 3),
            (3, Short, 4),
            (3, Int, 4),
            (4, Int, 4),
            (3, Long, 8),
            (4, Char, 4),
            (4, Ptr(Rc::new(RefCell::new(Int))), 8),
        ];
        for (offset, type_kind, expected) in &tests {
            assert_eq!(align_to(*offset, type_kind.align()), *expected);
        }
    }
}
