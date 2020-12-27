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
    Char,
    Int,
    Ptr(Rc<BaseType>),
    Array(u64, Rc<BaseType>, bool), // bool is whether initialized or not

    /// this is virtual type for `get_deref_type`
    _Deref(Rc<BaseType>),
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
            Char => write!(f, "{}", self.as_str()),
            Int => write!(f, "{}", self.as_str()),
            Ptr(x) => {
                let (count, b_type) = x.count_deref();
                let ptr = format!("{:*<width$}", "*", width = count + 1);
                write!(f, "{} {}", b_type.kind.as_str(), ptr)
            }
            Array(size, ptr_type, _) => write!(f, "{} [{}]", ptr_type.kind, size),
            _Deref(x) => {
                // todo
                // もう少しいい表示考える
                let (count, b_type) = x.count_deref();
                let ptr = format!("{:*<width$}", "*", width = count + 1);
                write!(f, "{} {}", b_type.kind.as_str(), ptr)
            }
            _Invalid(msg) => write!(f, "{}", msg),
        }
    }
}

impl TypeKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Char => "char",
            Int => "int",
            Ptr(_) => "Ptr",
            Array(_, _, _) => "Array",
            _Deref(_) => unreachable!(),
            _Invalid(_) => unreachable!(),
        }
    }

    pub fn from_starts(s: &str) -> Result<TypeKind, ()> {
        match s {
            x if x.starts_with(Char.as_str()) => Ok(Char),
            x if x.starts_with(Int.as_str()) => Ok(Int),
            _ => Err(()),
        }
    }

    pub fn size(&self) -> u64 {
        match self {
            Char => 1,
            Int => 4,
            Ptr(_) => 8,
            Array(size, base_type, _) => size * base_type.kind.size(),
            _Deref(_) => unreachable!(),
            _Invalid(_) => unreachable!(),
        }
    }

    fn is_num_type(&self) -> bool {
        match self {
            Char | Int => true,
            _ => false,
        }
    }

    pub fn get_deref_type(&self) -> Self {
        match self {
            Char => TypeKind::_Deref(Rc::new(BaseType::new(self.clone()))),
            Int => TypeKind::_Deref(Rc::new(BaseType::new(self.clone()))),
            Ptr(b_type) => b_type.kind.clone(),
            Array(_, b_type, _) => b_type.kind.clone(),
            _Deref(b_type) => b_type.kind.clone(),
            _Invalid(msg) => _Invalid(msg.clone()),
        }
    }

    pub fn get_addr_type(&self) -> Self {
        match self {
            _Deref(b_type) => b_type.kind.clone(),
            other => Ptr(Rc::new(BaseType::new(other.clone()))),
        }
    }

    /// return multiple of 8
    /// `int: 8 (not 4)`
    /// `ptr: 8`
    /// `int x[10]: 4 * 10 = 40`
    pub fn eight_size(&self) -> u64 {
        match self {
            Char => 8,
            Int => 8,
            Ptr(_) => 8,
            Array(_size, base_type, _) => {
                let mut size = _size * base_type.kind.size();
                size += (8 - size % 8) % 8; // sizeを8の倍数にする
                size
            }
            _Deref(_) => unreachable!(),
            _Invalid(_) => unreachable!(),
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
            Rc::new(BaseType::new(std::mem::replace(self, Int))),
            sized,
        );

        // use std::mem::take instead of std::mem::replace
        // *self = Array(size, Rc::new(BaseType::new(std::mem::take(self))));
    }

    pub fn array_of(size: u64, base: &TypeKind, initialized: bool) -> TypeKind {
        TypeKind::Array(size, Rc::new(BaseType::new(base.clone())), initialized)
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
            (Int, 4),
            (Ptr(Rc::new(BaseType::new(Int))), 8),
            (make_array(5, Char, false), 5),
            (make_array(5, Int, false), 20),
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
            (Ptr(Rc::new(BaseType::new(Int))), 8),
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
        TypeKind::Array(size, Rc::new(BaseType::new(type_kind)), initialized)
    }
}
