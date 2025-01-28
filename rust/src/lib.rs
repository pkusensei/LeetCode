mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn ways_to_make_fair(nums: &[i32]) -> i32 {
    let n = nums.len();
    let [mut odd_prefix, mut even_prefix] = [0, 1].map(|_| Vec::with_capacity(n));
    for (idx, &num) in nums.iter().enumerate() {
        odd_prefix.push(if idx & 1 == 1 { num } else { 0 } + odd_prefix.last().unwrap_or(&0));
        even_prefix.push(if idx & 1 == 0 { num } else { 0 } + even_prefix.last().unwrap_or(&0));
    }
    let mut res = 0;
    for idx in 0..n {
        let odd = if idx & 1 == 1 {
            odd_prefix[idx - 1]
        } else {
            odd_prefix[idx]
        };
        let even = if idx == 0 {
            0
        } else if idx & 1 == 0 {
            even_prefix[idx - 1]
        } else {
            even_prefix[idx]
        };
        let odd_suffix = odd_prefix[n - 1] - odd_prefix[idx];
        let even_suffix = even_prefix[n - 1] - even_prefix[idx];
        res += i32::from(odd + even_suffix == even + odd_suffix);
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
        assert_eq!(ways_to_make_fair(&[2, 1, 6, 4]), 1);
        assert_eq!(ways_to_make_fair(&[1, 1, 1]), 3);
        assert_eq!(ways_to_make_fair(&[1, 2, 3]), 0);
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
