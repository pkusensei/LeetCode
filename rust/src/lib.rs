mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn can_convert_string(s: &str, t: &str, k: i32) -> bool {
    s.len() == t.len()
        && s.bytes()
            .zip(t.bytes())
            .filter_map(|(a, b)| match a.cmp(&b) {
                std::cmp::Ordering::Less => Some(i32::from(b - a)),
                std::cmp::Ordering::Equal => None,
                std::cmp::Ordering::Greater => Some(i32::from(b + 26 - a)),
            })
            .fold(std::collections::HashMap::new(), |mut acc, diff| {
                acc.entry(diff).and_modify(|v| *v += 26).or_insert(diff);
                acc
            })
            .into_values()
            .all(|v| v <= k)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert!(can_convert_string("input", "ouput", 9));
        assert!(!can_convert_string("abc", "bcd", 10));
        assert!(can_convert_string("aab", "bbb", 27));
    }

    #[test]
    fn test() {
        assert!(!can_convert_string("jicfxaa", "ocxltbp", 15));
        assert!(!can_convert_string("atmtxzjkz", "tvbtjhvjd", 35));
        assert!(can_convert_string("iqssxdlb", "dyuqrwyr", 40));
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
