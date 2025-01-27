mod dsu;
mod helper;
mod trie;

use std::{cmp::Reverse, collections::HashSet};

#[allow(unused_imports)]
use helper::*;

pub fn min_deletions(s: &str) -> i32 {
    let mut nums: Vec<_> = s
        .bytes()
        .fold([0i16; 26], |mut acc, b| {
            acc[usize::from(b - b'a')] += 1;
            acc
        })
        .into_iter()
        .filter(|&v| v > 0)
        .collect();
    nums.sort_unstable_by_key(|&v| Reverse(v));
    let mut seen = HashSet::new();
    let mut res = 0;
    for mut num in nums {
        while num > 0 && !seen.insert(num) {
            num -= 1;
            res += 1;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    #[test]
    fn basics() {
        assert_eq!(min_deletions("bbcebab"), 2)
    }

    #[test]
    fn test() {}

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
