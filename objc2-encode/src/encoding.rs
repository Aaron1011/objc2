use core::fmt;

use crate::parse;

/// An Objective-C type-encoding.
///
/// Can be retrieved in Objective-C for a type `T` using the `@encode(T)`
/// directive.
/// ```objective-c , ignore
/// NSLog(@"Encoding of NSException: %s", @encode(NSException));
/// ```
///
/// For more information, see [Apple's documentation][ocrtTypeEncodings] and
/// [`clang`'s source code for generating `@encode`][clang-src].
///
/// [ocrtTypeEncodings]: https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ObjCRuntimeGuide/Articles/ocrtTypeEncodings.html
/// [clang-src]: https://github.com/llvm/llvm-project/blob/fae0dfa6421ea6c02f86ba7292fa782e1e2b69d1/clang/lib/AST/ASTContext.cpp#L7500-L7850
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive] // Maybe we're missing some encodings?
pub enum Encoding<'a> {
    /// A C `char`. Corresponds to the `c` code.
    Char,
    /// A C `short`. Corresponds to the `s` code.
    Short,
    /// A C `int`. Corresponds to the `i` code.
    Int,
    /// A C `long`. Corresponds to the `l` code.
    Long,
    /// A C `long long`. Corresponds to the `q` code.
    LongLong,
    /// A C `unsigned char`. Corresponds to the `C` code.
    UChar,
    /// A C `unsigned short`. Corresponds to the `S` code.
    UShort,
    /// A C `unsigned int`. Corresponds to the `I` code.
    UInt,
    /// A C `unsigned long`. Corresponds to the `L` code.
    ULong,
    /// A C `unsigned long long`. Corresponds to the `Q` code.
    ULongLong,
    /// A C `float`. Corresponds to the `f` code.
    Float,
    /// A C `double`. Corresponds to the `d` code.
    Double,
    /// A C `long double`. Corresponds to the `D` code.
    LongDouble,
    /// A C `float _Complex`. Corresponds to the `jf` code.
    FloatComplex,
    /// A C `_Complex` or `double _Complex`. Corresponds to the `jd` code.
    DoubleComplex,
    /// A C `long double _Complex`. Corresponds to the `jD` code.
    LongDoubleComplex,
    // TODO: Complex(&Encoding) ???
    /// A C++ `bool` / C99 `_Bool`. Corresponds to the `B` code.
    Bool,
    /// A C `void`. Corresponds to the `v` code.
    Void,
    /// A C `char *`. Corresponds to the `*` code.
    String,
    /// An Objective-C object (`id`). Corresponds to the `@` code.
    Object,
    /// An Objective-C block. Corresponds to the `@?` code.
    Block,
    /// An Objective-C class (`Class`). Corresponds to the `#` code.
    Class,
    /// An Objective-C selector (`SEL`). Corresponds to the `:` code.
    Sel,
    /// An unknown type. Corresponds to the `?` code.
    ///
    /// This is usually used to encode functions.
    Unknown,
    /// A bitfield with the given number of bits.
    ///
    /// Corresponds to the `b`num code.
    BitField(u8),
    /// A pointer to the given type.
    ///
    /// Corresponds to the `^`type code.
    Pointer(&'a Encoding<'a>),
    /// An array with the given length and type.
    ///
    /// Corresponds to the `[len type]` code.
    Array(usize, &'a Encoding<'a>),
    /// A struct with the given name and fields.
    ///
    /// The order of the fields must match the order of the order in this.
    ///
    /// It is not uncommon for the name to be `"?"`.
    ///
    /// Corresponds to the `{name=fields...}` code.
    Struct(&'a str, &'a [Encoding<'a>]),
    /// A union with the given name and fields.
    ///
    /// The order of the fields must match the order of the order in this.
    ///
    /// Corresponds to the `(name=fields...)` code.
    Union(&'a str, &'a [Encoding<'a>]),
    // "Vector" types have the '!' encoding, but are not implemented in clang

    // TODO: Atomic, const and other such specifiers
    // typedef struct x {
    //     int a;
    //     void* b;
    // } x_t;
    // NSLog(@"Encoding: %s", @encode(_Atomic x_t)); // -> A{x}
    // NSLog(@"Encoding: %s", @encode(const int*)); // -> r^i
}

impl Encoding<'_> {
    /// Check if one encoding is equivalent to another.
    pub fn equivalent_to(&self, other: &Self) -> bool {
        // For now, because we don't allow representing qualifiers
        self == other
    }

    /// Check if an encoding is equivalent to the given string representation.
    pub fn equivalent_to_str(&self, s: &str) -> bool {
        // if the given encoding can be successfully removed from the start
        // and an empty string remains, they were fully equivalent!
        if let Some(res) = self.equivalent_to_start_of_str(s) {
            res.is_empty()
        } else {
            false
        }
    }

    /// Check if an encoding is equivalent to the start of the given string
    /// representation.
    ///
    /// If it is equivalent, the remaining part of the string is returned.
    /// Otherwise this returns [`None`].
    pub fn equivalent_to_start_of_str<'a>(&self, s: &'a str) -> Option<&'a str> {
        // strip leading qualifiers
        let s = s.trim_start_matches(parse::QUALIFIERS);

        // TODO: Allow missing/"?" names in structs and unions?

        parse::rm_enc_prefix(s, self)
    }
}

impl fmt::Display for Encoding<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Encoding::*;
        let code = match *self {
            Char => "c",
            Short => "s",
            Int => "i",
            Long => "l",
            LongLong => "q",
            UChar => "C",
            UShort => "S",
            UInt => "I",
            ULong => "L",
            ULongLong => "Q",
            Float => "f",
            Double => "d",
            LongDouble => "D",
            FloatComplex => "jf",
            DoubleComplex => "jd",
            LongDoubleComplex => "jD",
            Bool => "B",
            Void => "v",
            String => "*",
            Object => "@",
            Block => "@?",
            Class => "#",
            Sel => ":",
            Unknown => "?",
            BitField(b) => {
                return write!(formatter, "b{}", b);
            }
            Pointer(t) => {
                return write!(formatter, "^{}", t);
            }
            Array(len, item) => {
                return write!(formatter, "[{}{}]", len, item);
            }
            Struct(name, fields) => {
                write!(formatter, "{{{}=", name)?;
                for field in fields {
                    fmt::Display::fmt(field, formatter)?;
                }
                return formatter.write_str("}");
            }
            Union(name, members) => {
                write!(formatter, "({}=", name)?;
                for member in members {
                    fmt::Display::fmt(member, formatter)?;
                }
                return formatter.write_str(")");
            }
        };
        formatter.write_str(code)
    }
}

// TODO: Deprecate and remove these PartialEq impls

/// Partial equality between an [`Encoding`] and a [`str`].
///
/// Using this is heavily discouraged, since it is not transitive; use
/// [`Encoding::equivalent_to_str`] instead for more correct semantics.
impl PartialEq<str> for Encoding<'_> {
    /// Using this is discouraged.
    fn eq(&self, other: &str) -> bool {
        self.equivalent_to_str(other)
    }

    /// Using this is discouraged.
    fn ne(&self, other: &str) -> bool {
        !self.eq(other)
    }
}

/// Partial equality between an [`Encoding`] and a [`str`].
///
/// Using this is heavily discouraged, since it is not transitive; use
/// [`Encoding::equivalent_to_str`] instead for more correct semantics.
impl PartialEq<Encoding<'_>> for str {
    /// Using this is discouraged.
    fn eq(&self, other: &Encoding<'_>) -> bool {
        other.equivalent_to_str(self)
    }

    /// Using this is discouraged.
    fn ne(&self, other: &Encoding<'_>) -> bool {
        !self.eq(other)
    }
}

#[cfg(test)]
mod tests {
    use super::Encoding;
    use alloc::string::ToString;

    #[test]
    fn test_array_display() {
        let e = Encoding::Array(12, &Encoding::Int);
        assert_eq!(e.to_string(), "[12i]");
        assert!(e.equivalent_to_str("[12i]"));
    }

    #[test]
    fn test_pointer_display() {
        let e = Encoding::Pointer(&Encoding::Int);
        assert_eq!(e.to_string(), "^i");
        assert!(e.equivalent_to_str("^i"));
    }

    #[test]
    fn test_pointer_eq() {
        let i = Encoding::Int;
        let p = Encoding::Pointer(&Encoding::Int);

        assert_eq!(p, p);
        assert_ne!(p, i);
    }

    #[test]
    fn test_int_display() {
        assert_eq!(Encoding::Int.to_string(), "i");
        assert!(Encoding::Int.equivalent_to_str("i"));
    }

    #[test]
    fn test_eq() {
        let i = Encoding::Int;
        let c = Encoding::Char;

        assert_eq!(i, i);
        assert_ne!(i, c);
    }

    #[test]
    fn test_struct_display() {
        let s = Encoding::Struct("CGPoint", &[Encoding::Char, Encoding::Int]);
        assert_eq!(s.to_string(), "{CGPoint=ci}");
        assert!(s.equivalent_to_str("{CGPoint=ci}"));
    }

    #[test]
    fn test_struct_eq() {
        let s = Encoding::Struct("CGPoint", &[Encoding::Char, Encoding::Int]);
        assert_eq!(s, s);
        assert_ne!(s, Encoding::Int);
    }

    #[test]
    fn test_union_display() {
        let u = Encoding::Union("Onion", &[Encoding::Char, Encoding::Int]);
        assert_eq!(u.to_string(), "(Onion=ci)");
        assert!(u.equivalent_to_str("(Onion=ci)"));
    }

    #[test]
    fn test_union_eq() {
        let u = Encoding::Union("Onion", &[Encoding::Char, Encoding::Int]);
        assert_eq!(u, u);
        assert_ne!(u, Encoding::Int);
    }
}
