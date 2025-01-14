mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn check_if_can_break(s1: String, s2: String) -> bool {
    let [mut s1, mut s2] = [s1, s2].map(|v| v.into_bytes());
    s1.sort_unstable();
    s2.sort_unstable();
    let mut it = s1.iter().zip(s2.iter());
    it.clone().all(|(a, b)| a >= b) || it.all(|(a, b)| a <= b)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert!(check_if_can_break("abc".into(), "xya".into()));
        assert!(!check_if_can_break("abe".into(), "acd".into()));
        assert!(check_if_can_break("leetcodee".into(), "interview".into()));
    }

    #[test]
    fn test() {
        assert!(!check_if_can_break("qvgjjsp".into(), "qmsbphx".into()));
    }

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
