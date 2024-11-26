mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn gcd_of_strings(str1: &str, str2: &str) -> String {
    let (n1, n2) = (str1.len(), str2.len());
    let n = n1.min(n2);
    let mut res = "";
    for len in 1..=n {
        if n1 % len > 0 || n2 % len > 0 {
            continue;
        }
        let prefix = &str1[..len];
        if [str1, str2].into_iter().all(|s| {
            s.as_bytes()
                .chunks_exact(len)
                .all(|w| w == prefix.as_bytes())
        }) {
            res = prefix;
        }
    }
    res.into()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(gcd_of_strings("ABCABC", "ABC"), "ABC");
        debug_assert_eq!(gcd_of_strings("ABABAB", "ABAB"), "AB");
        debug_assert_eq!(gcd_of_strings("LEET", "CODE"), "");
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
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
