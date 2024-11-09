mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn di_string_match(s: &str) -> Vec<i32> {
    let n = s.len();
    let mut res = Vec::with_capacity(1 + n);
    let (mut low, mut high) = (0, n as i32);
    for b in s.bytes() {
        if b == b'D' {
            res.push(high);
            high -= 1;
        } else {
            res.push(low);
            low += 1;
        }
    }
    res.push(low);
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(di_string_match("IDID"), [0, 4, 1, 3, 2]);
        debug_assert_eq!(di_string_match("III"), [0, 1, 2, 3]);
        debug_assert_eq!(di_string_match("DDI"), [3, 2, 0, 1]);
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
