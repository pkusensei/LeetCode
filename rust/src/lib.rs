mod dsu;
mod helper;
mod trie;

use std::collections::{HashMap, HashSet};

#[allow(unused_imports)]
use helper::*;

pub fn people_indexes(favorite_companies: &[&[&str]]) -> Vec<i32> {
    let map = favorite_companies
        .iter()
        .fold(HashMap::new(), |mut acc, row| {
            for c in row.iter() {
                let len = acc.len();
                acc.entry(c).or_insert(len);
            }
            acc
        });
    let list: Vec<_> = favorite_companies
        .iter()
        .map(|row| row.iter().map(|c| map[c]).collect::<HashSet<_>>())
        .collect();
    let mut res = vec![];
    'outer: for (i1, a) in list.iter().enumerate() {
        for (i2, b) in list.iter().enumerate() {
            if i1 == i2 {
                continue;
            }
            if a.is_subset(b) {
                continue 'outer;
            }
        }
        res.push(i1 as i32);
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            people_indexes(&[
                &["leetcode", "google", "facebook"],
                &["google", "microsoft"],
                &["google", "facebook"],
                &["google"],
                &["amazon"]
            ]),
            [0, 1, 4]
        );
        assert_eq!(
            people_indexes(&[
                &["leetcode", "google", "facebook"],
                &["leetcode", "amazon"],
                &["facebook", "google"]
            ]),
            [0, 1]
        );
        assert_eq!(
            people_indexes(&[&["leetcode"], &["google"], &["facebook"], &["amazon"]]),
            [0, 1, 2, 3]
        );
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
