mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_operations(nums: &[i32], x: i32) -> i32 {
    let sum: i32 = nums.iter().sum();
    if sum == x {
        return nums.len() as _;
    }
    let target = sum - x;
    let mut seen = std::collections::HashMap::from([(0, -1)]);
    let mut res = 0;
    let mut curr = 0;
    for (idx, &num) in nums.iter().enumerate() {
        curr += num;
        if let Some(prev) = seen.get(&(curr - target)) {
            res = res.max(idx as i32 - prev);
        }
        seen.entry(curr).or_insert(idx as i32);
    }
    if res == 0 {
        -1
    } else {
        nums.len() as i32 - res
    }
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
        assert_eq!(min_operations(&[1, 1, 4, 2, 3], 5), 2);
        assert_eq!(min_operations(&[5, 6, 7, 8, 9], 4), -1);
        assert_eq!(min_operations(&[3, 2, 20, 1, 1, 3], 10), 5);
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
