mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn compressed_string(word: &str) -> String {
    let mut res = vec![];
    for ch in word.as_bytes().chunk_by(|a, b| a == b) {
        let mut n = ch.len();
        let b = ch[0];
        while n >= 9 {
            res.push(b'9');
            res.push(b);
            n -= 9;
        }
        if n > 0 {
            res.push(n as u8 + b'0');
            res.push(b);
        }
    }
    String::from_utf8(res).unwrap()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(compressed_string("abcde"), "1a1b1c1d1e");
        debug_assert_eq!(compressed_string("aaaaaaaaaaaaaabb"), "9a5a2b");
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
