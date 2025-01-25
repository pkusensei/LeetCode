mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
    l.into_iter()
        .zip(r)
        .map(|(a, b)| check(&nums[a as usize..=b as usize]))
        .collect()
}

fn check(nums: &[i32]) -> bool {
    let n = nums.len() as i32;
    if n < 2 {
        return true;
    }
    let (min, max, set) = nums.iter().fold(
        (i32::MAX, i32::MIN, std::collections::HashSet::new()),
        |(min, max, mut set), &num| {
            set.insert(num);
            (min.min(num), max.max(num), set)
        },
    );
    if (max - min) % (n - 1) > 0 {
        return false;
    }
    let d = (max - min) / (n - 1);
    let mut curr = min + d;
    while curr < max {
        if !set.contains(&curr) {
            return false;
        }
        curr += d;
    }
    true
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
    fn basics() {}

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
