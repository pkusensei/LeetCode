mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn get_sum_absolute_differences(nums: &[i32]) -> Vec<i32> {
    let n = nums.len();
    let mut prefix = Vec::with_capacity(n);
    for num in nums.iter() {
        prefix.push(num + prefix.last().unwrap_or(&0));
    }
    let mut res = Vec::with_capacity(n);
    for (idx, num) in nums.iter().enumerate() {
        let mut curr = num * (1 + idx as i32) - prefix[idx];
        curr += prefix[n - 1] - prefix[idx] - num * (n - idx - 1) as i32;
        res.push(curr);
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
        assert_eq!(get_sum_absolute_differences(&[2, 3, 5]), [4, 3, 5]);
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
