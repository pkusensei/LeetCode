mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_unique_subarray(nums: &[i32]) -> i32 {
    let mut left = 0;
    let mut curr_sum = 0;
    let mut window = std::collections::HashMap::new();
    let mut res = 0;
    for (right, &num) in nums.iter().enumerate() {
        *window.entry(num).or_insert(0) += 1;
        curr_sum += num;
        while window.len() < right + 1 - left {
            let v = window.entry(nums[left]).or_insert(0);
            *v -= 1;
            curr_sum -= nums[left];
            if *v == 0 {
                window.remove(&nums[left]);
            }
            left += 1;
        }
        res = res.max(curr_sum);
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
        assert_eq!(maximum_unique_subarray(&[4, 2, 4, 5, 6]), 17);
        assert_eq!(maximum_unique_subarray(&[5, 2, 1, 2, 5, 2, 1, 2, 5]), 8);
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
