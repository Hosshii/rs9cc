use self::TypeKind::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct BaseType {
    pub kind: TypeKind,
}

impl BaseType {
    pub fn new(kind: TypeKind) -> Self {
        Self { kind }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub enum TypeKind {
    Int,
    Ptr(Box<BaseType>),
}

impl TypeKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Int => "int",
            Ptr(_) => "Ptr",
        }
    }

    pub fn from_starts(s: &str) -> Result<TypeKind, ()> {
        match s {
            x if x.starts_with(Int.as_str()) => Ok(Int),
            _ => Err(()),
        }
    }
}
