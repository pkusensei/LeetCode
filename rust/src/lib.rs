mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn decode_at_index(s: &str, k: i32) -> String {
    let mut curr = 0;
    let k = k as usize;
    for b in s.bytes() {
        if b.is_ascii_alphabetic() {
            curr += 1;
            if k == curr {
                return char::from(b).into();
            }
        } else if b.is_ascii_digit() {
            let digit = usize::from(b - b'0');
            if curr * digit >= k {
                if k % curr == 0 {
                    return decode_at_index(s, curr as i32);
                } else {
                    return decode_at_index(s, (k % curr) as i32);
                }
            }
            curr *= digit
        }
    }
    "".to_string()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(decode_at_index("leet2code3", 10), "o");
        debug_assert_eq!(decode_at_index("ha22", 5), "h");
        debug_assert_eq!(decode_at_index("a2345678999999999999999", 1), "a");
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64>,
        T2: Into<f64>,
    {
        const EP: f64 = 1e-5;
        debug_assert!((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP);
    }
}
