use self::TypeKind::*;
use std::fmt;
use std::rc::Rc;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct BaseType {
    pub kind: TypeKind,
}

impl BaseType {
    pub fn new(kind: TypeKind) -> Self {
        Self { kind }
    }

    /// count number of '*' of type
    /// ```
    /// use std::rc::Rc;
    /// use rs9cc::base_types::{BaseType, TypeKind};
    ///
    /// let b_type = BaseType::new(TypeKind::Ptr(Rc::new(BaseType::new(TypeKind::Int))));
    /// assert_eq!(b_type.count_deref(),(1,&BaseType { kind: TypeKind::Int }))
    /// ```
    /// if not pointer type, return 0 and same type
    pub fn count_deref(&self) -> (usize, &BaseType) {
        let mut count = 0;
        let mut ref_base_type = self;
        while let TypeKind::Ptr(ref x) = ref_base_type.kind {
            count += 1;
            ref_base_type = &x;
        }
        return (count, ref_base_type);
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub enum TypeKind {
    Int,
    Ptr(Rc<BaseType>),
}

impl fmt::Display for TypeKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Int => writeln!(f, "{}", self.as_str()),
            Ptr(x) => {
                let (count, b_type) = x.count_deref();
                writeln!(
                    f,
                    "{b_type} {:*<width$}",
                    b_type = b_type.kind.as_str(),
                    width = count + 1
                )
            }
        }
    }
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

    pub fn size(&self) -> usize {
        match self {
            Int => 4,
            Ptr(_) => 8,
        }
    }
}
