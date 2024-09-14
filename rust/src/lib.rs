mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn is_subsequence(s: &str, t: &str) -> bool {
    let (s, t) = (s.as_bytes(), t.as_bytes());
    let (mut si, mut ti) = (0, 0);
    while si < s.len() && ti < t.len() {
        if s[si] == t[ti] {
            si += 1;
            ti += 1
        } else {
            ti += 1
        }
    }
    si == s.len()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(is_subsequence("abc", "ahbgdc"));
        debug_assert!(!is_subsequence("axc", "ahbgdc"));
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(i1: I1, i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: IntoIterator<Item = T1>,
        I2: IntoIterator<Item = T2>,
    {
        let mut v1: Vec<_> = i1.into_iter().collect();
        let mut v2: Vec<_> = i2.into_iter().collect();
        v1.sort_unstable();
        v2.sort_unstable();
        debug_assert_eq!(v1, v2);
    }
}
