mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn special_array(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable_by_key(|&v| std::cmp::Reverse(v));
    let mut left = 0;
    let mut right = nums.len() as i32;
    while left <= right {
        let mid = left + (right - left) / 2;
        let i = nums.partition_point(|&v| v >= mid) as i32;
        match i.cmp(&mid) {
            std::cmp::Ordering::Less => right = mid - 1,
            std::cmp::Ordering::Equal => return mid,
            std::cmp::Ordering::Greater => left = 1 + mid,
        }
    }
    -1
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
        assert_eq!(special_array(vec![0, 4, 3, 0, 4]), 3);
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
