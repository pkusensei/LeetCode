mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_freq(s: &str, max_letters: i32, min_size: i32, _max_size: i32) -> i32 {
    let n = min_size as usize;
    if s.len() < n {
        return 0;
    }
    let mut count = s[..n].bytes().fold([0; 26], |mut acc, b| {
        acc[usize::from(b - b'a')] += 1;
        acc
    });
    let mut map = std::collections::HashMap::new();
    if count.iter().filter(|&&c| c > 0).count() <= max_letters as usize {
        map.insert(&s[..n], 1);
    }
    for i in 1..=s.len() - n {
        count[usize::from(s.as_bytes()[i - 1] - b'a')] -= 1;
        count[usize::from(s.as_bytes()[i + n - 1] - b'a')] += 1;
        if count.iter().filter(|&&c| c > 0).count() <= max_letters as usize {
            *map.entry(&s[i..i + n]).or_insert(0) += 1;
        }
    }
    map.into_values().max().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(max_freq("aababcaab", 2, 3, 4), 2);
        assert_eq!(max_freq("aaaa", 1, 3, 3), 2);
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
