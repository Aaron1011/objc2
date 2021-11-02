pub(crate) const fn end_str_len(mut n: u128) -> usize {
    let mut i = 0;
    if n == 0 {
        return 1;
    }
    while n > 0 {
        n = n / 10;
        i += 1;
    }
    i
}

pub(crate) const fn get_str_array<const RES: usize>(mut n: u128) -> [u8; RES] {
    let mut res: [u8; RES] = [0; RES];
    let mut i = 0;
    if n == 0 {
        res[0] = '0' as u8;
        return res;
    }
    while n > 0 {
        res[i] = '0' as u8 + (n % 10) as u8;
        n = n / 10;
        i += 1;
    }

    let mut rev: [u8; RES] = [0; RES];
    let mut rev_i = 0;
    while 0 < i {
        i -= 1;
        rev[rev_i] = res[i];
        n = n / 10;
        rev_i += 1;
    }
    rev
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! const_int_str {
        ($n:expr) => {{
            const X: [u8; end_str_len($n as u128)] = get_str_array($n as u128);
            unsafe { core::mem::transmute::<&[u8], &str>(&X) }
        }};
    }

    const STR_0: &'static str = const_int_str!(0);
    const STR_4: &'static str = const_int_str!(4);
    const STR_42: &'static str = const_int_str!(42);
    const STR_100: &'static str = const_int_str!(100);
    const STR_999: &'static str = const_int_str!(999);
    const STR_1236018655: &'static str = const_int_str!(1236018655);

    #[test]
    fn test() {
        assert_eq!(STR_0, "0");
        assert_eq!(STR_4, "4");
        assert_eq!(STR_42, "42");
        assert_eq!(STR_100, "100");
        assert_eq!(STR_999, "999");
        assert_eq!(STR_1236018655, "1236018655");
    }
}
