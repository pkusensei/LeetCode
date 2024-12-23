mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn palindrome_partition(s: &str, k: i32) -> i32 {
    dfs(s, k as usize, &mut HashMap::new())
}

fn dfs<'a>(s: &'a str, k: usize, memo: &mut HashMap<(&'a str, usize), i32>) -> i32 {
    let n = s.len();
    if n == k {
        return 0;
    }
    if k <= 1 {
        return s
            .bytes()
            .zip(s.bytes().rev())
            .take(n / 2)
            .filter(|&(a, b)| a != b)
            .count() as _;
    }
    if let Some(&v) = memo.get(&(s, k)) {
        return v;
    }
    let mut res = i32::MAX;
    for idx in 0..n - k + 1 {
        let temp = dfs(&s[..1 + idx], 1, memo) + dfs(&s[1 + idx..], k - 1, memo);
        res = res.min(temp);
    }
    memo.insert((s, k), res);
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(palindrome_partition("abc", 2), 1);
        assert_eq!(palindrome_partition("aabbc", 3), 0);
        assert_eq!(palindrome_partition("leetcode", 8), 0);
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
