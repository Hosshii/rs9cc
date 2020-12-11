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
    Array(u64, Rc<BaseType>),
}

// impl Default for TypeKind {
//     fn default() -> Self {
//         TypeKind::Int
//     }
// }

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
            Array(size, ptr_type) => writeln!(f, "{} [{}]", ptr_type.kind, size),
        }
    }
}

impl TypeKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Int => "int",
            Ptr(_) => "Ptr",
            Array(_, _) => "Array",
        }
    }

    pub fn from_starts(s: &str) -> Result<TypeKind, ()> {
        match s {
            x if x.starts_with(Int.as_str()) => Ok(Int),
            _ => Err(()),
        }
    }

    pub fn size(&self) -> u64 {
        match self {
            Int => 4,
            Ptr(_) => 8,
            Array(size, base_type) => size * base_type.kind.size(),
        }
    }

    /// return multiple of 8
    /// `int: 8 (not 4)`
    /// `ptr: 8`
    /// `int x[10]: 8 * 10 = 80`
    pub fn eight_size(&self) -> u64 {
        match self {
            Int => 8,
            Ptr(_) => 8,
            Array(size, base_type) => size * base_type.kind.size(),
        }
    }

    pub fn to_arr(&mut self, size: u64) {
        // unsafeをまだよくわかってないのでmem::replaceを使ってやる

        // unsafe {
        //     std::ptr::write(
        //         self,
        //         Array(size, Rc::new(BaseType::new(std::ptr::read(self)))),
        //     )
        // }

        *self = Array(size, Rc::new(BaseType::new(std::mem::replace(self, Int))));

        // use std::mem::take instead of std::mem::replace
        // *self = Array(size, Rc::new(BaseType::new(std::mem::take(self))));
    }
}
