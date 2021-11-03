use super::static_int_str;
#[cfg(feature = "unstable_static_encoding_str")]
use super::Encode;
use super::Encoding;

impl<'a> Encoding<'a> {
    #[doc(hidden)]
    pub const fn end_str_len(self) -> usize {
        use Encoding::*;

        match self {
            Char | Short | Int | Long | LongLong | UChar | UShort | UInt | ULong | ULongLong
            | Float | Double | Bool | Void | String | Object | Class | Sel | Unknown => 1,
            Block => 2,
            BitField(b) => 1 + static_int_str::end_str_len(b as u128),
            Pointer(t) => 1 + t.end_str_len(),
            Array(len, item) => {
                1 + static_int_str::end_str_len(len as u128) + item.end_str_len() + 1
            }
            Struct(name, items) | Union(name, items) => {
                let mut res = 1 + name.len() + 1;
                let mut i = 0;
                while i < items.len() {
                    res += items[i].end_str_len();
                    i += 1;
                }
                res + 1
            }
        }
    }

    #[doc(hidden)]
    pub const fn get_str_array<const LEN: usize>(self) -> [u8; LEN] {
        use Encoding::*;

        let mut res: [u8; LEN] = [0; LEN];

        match self {
            Char => res[0] = 'c' as u8,
            Short => res[0] = 's' as u8,
            Int => res[0] = 'i' as u8,
            Long => res[0] = 'l' as u8,
            LongLong => res[0] = 'q' as u8,
            UChar => res[0] = 'C' as u8,
            UShort => res[0] = 'S' as u8,
            UInt => res[0] = 'I' as u8,
            ULong => res[0] = 'L' as u8,
            ULongLong => res[0] = 'Q' as u8,
            Float => res[0] = 'f' as u8,
            Double => res[0] = 'd' as u8,
            Bool => res[0] = 'B' as u8,
            Void => res[0] = 'v' as u8,
            Block => {
                res[0] = '@' as u8;
                res[1] = '?' as u8;
            }
            String => res[0] = '*' as u8,
            Object => res[0] = '@' as u8,
            Class => res[0] = '#' as u8,
            Sel => res[0] = ':' as u8,
            Unknown => res[0] = '?' as u8,
            BitField(b) => {
                let mut res_i = 0;

                res[res_i] = 'b' as u8;
                res_i += 1;

                let mut i = 0;
                // We use 3 even though it creates an oversized array
                let arr = static_int_str::get_str_array::<3>(b as u128);
                while i < static_int_str::end_str_len(b as u128) {
                    res[res_i] = arr[i];
                    res_i += 1;
                    i += 1;
                }
            }
            Pointer(t) => {
                let mut res_i = 0;

                res[res_i] = '^' as u8;
                res_i += 1;

                let mut i = 0;
                // We use LEN even though it creates an oversized array
                let arr = t.get_str_array::<LEN>();
                while i < t.end_str_len() {
                    res[res_i] = arr[i];
                    res_i += 1;
                    i += 1;
                }
            }
            Array(len, item) => {
                let mut res_i = 0;

                res[res_i] = '[' as u8;
                res_i += 1;

                let mut i = 0;
                // We use 20 even though it creates an oversized array
                let arr = static_int_str::get_str_array::<20>(len as u128);
                while i < static_int_str::end_str_len(len as u128) {
                    res[res_i] = arr[i];
                    res_i += 1;
                    i += 1;
                }

                let mut i = 0;
                // We use LEN even though it creates an oversized array
                let arr = item.get_str_array::<LEN>();
                while i < item.end_str_len() {
                    res[res_i] = arr[i];
                    res_i += 1;
                    i += 1;
                }

                res[res_i] = ']' as u8;
            }
            Struct(name, items) | Union(name, items) => {
                let mut res_i = 0;

                match self {
                    Struct(_, _) => res[res_i] = '{' as u8,
                    Union(_, _) => res[res_i] = '(' as u8,
                    _ => {}
                };
                res_i += 1;

                let mut name_i = 0;
                let name = name.as_bytes();
                while name_i < name.len() {
                    res[res_i] = name[name_i];
                    res_i += 1;
                    name_i += 1;
                }

                res[res_i] = '=' as u8;
                res_i += 1;

                let mut items_i = 0;
                while items_i < items.len() {
                    // We use LEN even though it creates an oversized array
                    let field_res = items[items_i].get_str_array::<LEN>();

                    let mut item_res_i = 0;
                    while item_res_i < items[items_i].end_str_len() {
                        res[res_i] = field_res[item_res_i];
                        res_i += 1;
                        item_res_i += 1;
                    }
                    items_i += 1;
                }

                match self {
                    Struct(_, _) => res[res_i] = '}' as u8,
                    Union(_, _) => res[res_i] = ')' as u8,
                    _ => {}
                };
            }
        };
        res
    }

    #[doc(hidden)]
    pub const fn end_cstr_len(self) -> usize {
        self.end_str_len() + 1
    }

    #[doc(hidden)]
    pub const fn get_cstr_array<const RES: usize>(self) -> [u8; RES] {
        // Contains nul byte at the end
        self.get_str_array()
    }
}

/// Workaround since we can't specify the correct `where` bound on `Encode`.
#[cfg(feature = "unstable_static_encoding_str")]
pub struct EncodingHelper<T>(T);

#[cfg(feature = "unstable_static_encoding_str")]
impl<T: super::Encode> EncodingHelper<T>
where
    [u8; T::ENCODING.end_cstr_len()]: Sized,
{
    #[doc(hidden)]
    const __ENCODING_CSTR_BYTES: [u8; T::ENCODING.end_cstr_len()] = T::ENCODING.get_cstr_array();

    /// TODO
    pub const ENCODING_CSTR: *const u8 = Self::__ENCODING_CSTR_BYTES.as_ptr();
}

#[cfg(feature = "unstable_static_encoding_str")]
impl<T: Encode> EncodingHelper<T>
where
    [u8; T::ENCODING.end_str_len()]: Sized,
{
    #[doc(hidden)]
    const __ENCODING_STR_BYTES: [u8; T::ENCODING.end_str_len()] = T::ENCODING.get_str_array();

    /// TODO
    pub const ENCODING_STR: &'static str =
        unsafe { core::mem::transmute::<&[u8], &str>(&Self::__ENCODING_STR_BYTES) };
}

#[cfg(test)]
mod tests {
    use super::Encoding;

    macro_rules! const_encoding {
        ($e:expr) => {{
            const E: $crate::Encoding<'static> = $e;
            const X: [u8; E.end_str_len()] = E.get_str_array();
            unsafe { core::mem::transmute::<&'static [u8], &'static str>(&X) }
        }};
    }

    #[test]
    fn test_const_encoding() {
        const CHAR: &'static str = const_encoding!(Encoding::Char);
        assert_eq!(CHAR, "c");
        const BLOCK: &'static str = const_encoding!(Encoding::Block);
        assert_eq!(BLOCK, "@?");
        const STRUCT: &'static str =
            const_encoding!(Encoding::Struct("abc", &[Encoding::Int, Encoding::Double]));
        assert_eq!(STRUCT, "{abc=id}");
        const VARIOUS: &'static str = const_encoding!(Encoding::Struct(
            "abc",
            &[
                Encoding::Pointer(&Encoding::Array(8, &Encoding::Bool)),
                Encoding::Union("def", &[Encoding::Block]),
                Encoding::Pointer(&Encoding::Pointer(&Encoding::BitField(255))),
                Encoding::Unknown,
            ]
        ));
        assert_eq!(VARIOUS, "{abc=^[8B](def=@?)^^b255?}");
    }
}
