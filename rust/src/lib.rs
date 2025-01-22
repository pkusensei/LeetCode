mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn modify_string(s: String) -> String {
    let mut s = s.into_bytes();
    let n = s.len();
    for i in 0..n {
        if s[i] == b'?' {
            for b in b'a'..=b'z' {
                if !s.get(1 + i).is_some_and(|&v| v == b) {
                    if i == 0 {
                        s[i] = b;
                    } else if s[i - 1] != b {
                        s[i] = b;
                    }
                    break;
                }
            }
        }
    }
    String::from_utf8(s).unwrap()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {}

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
