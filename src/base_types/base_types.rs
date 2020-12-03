use self::TypeKind::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub struct BaseType {
    pub kind: TypeKind,
    pub deref_num: usize,
}

impl BaseType {
    pub fn new(kind: TypeKind, deref_num: usize) -> Self {
        Self { kind, deref_num }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub enum TypeKind {
    Int,
}

impl TypeKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Int => "int",
        }
    }

    pub fn from_starts(s: &str) -> Result<TypeKind, ()> {
        match s {
            x if x.starts_with(Int.as_str()) => Ok(Int),
            _ => Err(()),
        }
    }
}
