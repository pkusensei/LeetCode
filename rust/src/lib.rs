mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn break_palindrome(palindrome: String) -> String {
    if palindrome.len() <= 1 {
        return String::new();
    }
    let mut s = palindrome.into_bytes();
    if let Some(v) = s.iter_mut().find(|b| **b != b'a') {
        *v = b'a';
    } else {
        s.last_mut().map(|b| *b = b'b');
    }
    String::from_utf8(s).unwrap()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(break_palindrome("abccba".into()), "aaccba");
        assert_eq!(break_palindrome("a".into()), "");
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
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
