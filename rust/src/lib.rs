mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn freq_alphabets(s: &str) -> String {
    let (s, n) = (s.as_bytes(), s.len());
    let mut res = vec![];
    let mut i = n - 1;
    loop {
        let b = if s[i] == b'#' {
            i -= 2;
            10 * (s[i] - b'0') + s[i + 1] - b'0'
        } else {
            s[i] - b'0'
        };
        res.push(b + b'a' - 1);
        if i == 0 {
            break;
        }
        i -= 1
    }
    res.into_iter().rev().map(|b| b as char).collect()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(freq_alphabets("10#11#12"), "jkab");
        assert_eq!(freq_alphabets("1326#"), "acz");
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
