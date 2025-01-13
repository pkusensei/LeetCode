mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn num_steps(s: String) -> i32 {
    let mut s = s.into_bytes();
    let mut res = 0;
    while s.len() > 1 {
        while s.last().is_some_and(|&v| v == b'0') {
            res += 1;
            s.pop();
        }
        if s.len() == 1 {
            break;
        }
        if let Some(i) = s.iter().rposition(|&v| v == b'0') {
            for v in s.iter_mut().skip(i) {
                (*v) = b'0'
            }
            s[i] = b'1';
            res += 1;
        } else {
            res += 1 + s.len() as i32;
            break;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(num_steps("1101".into()), 6);
        assert_eq!(num_steps("10".into()), 1);
        assert_eq!(num_steps("1".into()), 0);
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
