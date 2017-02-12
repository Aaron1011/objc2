use std::fmt;
use std::mem;

use {Encoding, PointerEncoding, StructEncoding, FieldsComparator};
use descriptor::Descriptor;
use encodings::{Primitive, Never};

pub fn chomp(s: &str) -> (Option<&str>, &str) {
    let head_len = chomp_ptr(s)
        .or_else(|| chomp_struct(s))
        .or_else(|| {
            if let (Some(_), t) = chomp_primitive(s) {
                Some(s.len() - t.len())
            } else {
                None
            }
        });

    if let Some(head_len) = head_len {
        let (h, t) = s.split_at(head_len);
        (Some(h), t)
    } else {
        (None, s)
    }
}

fn chomp_ptr(s: &str) -> Option<usize> {
    if s.starts_with("^") {
        let (h, _) = chomp(&s[1..]);
        h.map(|h| h.len() + 1)
    } else {
        None
    }
}

fn chomp_struct(s: &str) -> Option<usize> {
    if !s.starts_with("{") {
        return None;
    }

    let mut depth = 1;
    for (i, b) in s.bytes().enumerate().skip(1) {
        if b == b'{' {
            depth += 1;
        } else if b == b'}' {
            depth -= 1;
        }

        if depth == 0 {
            return Some(i + 1);
        }
    }

    None
}

fn chomp_primitive(s: &str) -> (Option<Primitive>, &str) {
    if s.is_empty() {
        return (None, s);
    }

    let (h, t) = s.split_at(1);
    match h {
        "c" => (Some(Primitive::Char), t),
        "i" => (Some(Primitive::Int), t),
        _ => (None, s),
    }
}

enum ParseResult {
    Primitive(Primitive),
    Pointer,
    Struct,
    Error,
}

fn parse(s: &str) -> ParseResult {
    if s.starts_with('{') && s.ends_with('}') {
        ParseResult::Struct
    } else if s.starts_with('^') {
        ParseResult::Pointer
    } else {
        let (h, t) = chomp_primitive(s);
        if !t.is_empty() {
            ParseResult::Error
        } else if let Some(p) = h {
            ParseResult::Primitive(p)
        } else {
            ParseResult::Error
        }
    }
}

fn parse_struct(s: &str) -> Option<(&str, &str)> {
    if let Some(sep_pos) = s.find('=') {
        let name = &s[1..sep_pos];
        let fields = &s[sep_pos + 1..s.len() - 1];
        Some((name, fields))
    } else {
        None
    }
}

fn is_valid(s: &str) -> bool {
    match parse(s) {
        ParseResult::Primitive(_) => true,
        ParseResult::Pointer => {
            let pointee = &s[1..];
            is_valid(pointee)
        },
        ParseResult::Struct => {
            let mut fields = match parse_struct(s) {
                Some((_, fields)) => fields,
                _ => return false,
            };
            while !fields.is_empty() {
                let (h, t) = chomp(fields);
                if h.map_or(false, is_valid) {
                    return false;
                }
                fields = t;
            }
            true
        }
        ParseResult::Error => false,
    }
}

pub struct StrEncoding<S = str>(S) where S: ?Sized + AsRef<str>;

impl StrEncoding {
    fn from_str_unchecked(s: &str) -> &StrEncoding {
        unsafe { mem::transmute(s) }
    }
}

impl<S> StrEncoding<S> where S: AsRef<str> {
    pub fn new_unchecked(s: S) -> StrEncoding<S> {
        StrEncoding(s)
    }
}

impl<S> StrEncoding<S> where S: ?Sized + AsRef<str> {
    fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl<S> Encoding for StrEncoding<S> where S: ?Sized + AsRef<str> {
    type Pointer = StrPointerEncoding;
    type Struct = StrStructEncoding;

    fn descriptor(&self) -> Descriptor<StrPointerEncoding, StrStructEncoding> {
        let s = self.as_str();
        match parse(s) {
            ParseResult::Primitive(p) => Descriptor::Primitive(p),
            ParseResult::Pointer =>
                Descriptor::Pointer(StrPointerEncoding::from_str_unchecked(s)),
            ParseResult::Struct =>
                Descriptor::Struct(StrStructEncoding::from_str_unchecked(s)),
            ParseResult::Error => panic!(),
        }
    }

    fn eq_encoding<T: ?Sized + Encoding>(&self, other: &T) -> bool {
        self.descriptor().eq_encoding(other)
    }
}

impl<S> fmt::Display for StrEncoding<S> where S: ?Sized + AsRef<str> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), formatter)
    }
}

pub struct StrPointerEncoding(StrEncoding);

impl StrPointerEncoding {
    fn from_str_unchecked(s: &str) -> &StrPointerEncoding {
        unsafe { mem::transmute(s) }
    }
}

impl Encoding for StrPointerEncoding {
    type Pointer = StrPointerEncoding;
    type Struct = Never;

    fn descriptor(&self) -> Descriptor<StrPointerEncoding, Never> {
        Descriptor::Pointer(self)
    }

    fn eq_encoding<T: ?Sized + Encoding>(&self, other: &T) -> bool {
        if let Descriptor::Pointer(p) = other.descriptor() {
            self.pointee().eq_encoding(p)
        } else {
            false
        }
    }
}

impl PointerEncoding for StrPointerEncoding {
    type Pointee = StrEncoding;

    fn pointee(&self) -> &StrEncoding {
        StrEncoding::from_str_unchecked(&self.0.as_str()[1..])
    }
}

impl fmt::Display for StrPointerEncoding {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, formatter)
    }
}

pub struct StrStructEncoding(StrEncoding);

impl StrStructEncoding {
    fn from_str_unchecked(s: &str) -> &StrStructEncoding {
        unsafe { mem::transmute(s) }
    }

    fn contents(&self) -> (&str, StrFields) {
        let s = self.0.as_str();
        let (name, fields) = parse_struct(s).unwrap();
        (name, StrFields { fields: fields })
    }
}

impl Encoding for StrStructEncoding {
    type Pointer = Never;
    type Struct = StrStructEncoding;

    fn descriptor(&self) -> Descriptor<Never, StrStructEncoding> {
        Descriptor::Struct(self)
    }

    fn eq_encoding<T: ?Sized + Encoding>(&self, other: &T) -> bool {
        if let Descriptor::Struct(s) = other.descriptor() {
            let (name, fields) = self.contents();
            s.eq_struct(name, fields)
        } else {
            false
        }
    }
}

impl StructEncoding for StrStructEncoding {
    fn name(&self) -> &str {
        self.contents().0
    }

    fn eq_struct<F: FieldsComparator>(&self, other_name: &str, other_fields: F) -> bool {
        let (name, fields) = self.contents();
        name == other_name && fields.eq(other_fields)
    }
}

impl fmt::Display for StrStructEncoding {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, formatter)
    }
}

struct StrFields<'a> {
    fields: &'a str,
}

impl<'a> StrFields<'a> {
    fn eq<F: FieldsComparator>(self, mut other: F) -> bool {
        for enc in self {
            if !other.eq_next(enc) {
                return false;
            }
        }
        other.is_finished()
    }
}

impl<'a> Iterator for StrFields<'a> {
    type Item = &'a StrEncoding;

    fn next(&mut self) -> Option<&'a StrEncoding> {
        if self.fields.is_empty() {
            None
        } else {
            let (h, t) = chomp(self.fields);
            self.fields = t;
            Some(StrEncoding::from_str_unchecked(h.unwrap()))
        }
    }
}

impl<'a> FieldsComparator for StrFields<'a> {
    fn eq_next<E: ?Sized + Encoding>(&mut self, other: &E) -> bool {
        self.next().map_or(false, |e| e.eq_encoding(other))
    }

    fn is_finished(&self) -> bool {
        self.fields.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chomp() {
        let (h, t) = chomp("{A={B=ci^{C=c}}ci}c^i{C=c}");
        assert_eq!(h, Some("{A={B=ci^{C=c}}ci}"));

        let (h, t) = chomp(t);
        assert_eq!(h, Some("c"));

        let (h, t) = chomp(t);
        assert_eq!(h, Some("^i"));

        let (h, t) = chomp(t);
        assert_eq!(h, Some("{C=c}"));

        let (h, _) = chomp(t);
        assert_eq!(h, None);
    }

    #[test]
    fn test_parsed_struct() {
        let s = StrStructEncoding::from_str_unchecked("{CGPoint=ci}");

        let (name, mut fields) = s.contents();
        assert_eq!(name, "CGPoint");
        assert_eq!(fields.next().unwrap().to_string(), "c");
        assert_eq!(fields.next().unwrap().to_string(), "i");
        assert!(fields.next().is_none());
    }
}