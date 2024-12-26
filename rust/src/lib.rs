mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_best_value(arr: &mut [i32], target: i32) -> i32 {
    let n = arr.len();
    arr.sort_unstable();
    let mut prefix = Vec::with_capacity(n);
    for &num in arr.iter() {
        prefix.push(num + prefix.last().unwrap_or(&0));
    }
    let mut left = 0;
    let mut right = arr[n - 1];
    let mut curr_diff = u32::MAX;
    let mut res = i32::MAX;
    while left <= right {
        let mid = left + (right - left) / 2;
        let i = arr.partition_point(|&v| v <= mid);
        let sum = if i > 0 {
            prefix[i - 1] + mid * (n - i) as i32
        } else {
            mid * n as i32
        };
        if sum <= target {
            left = 1 + mid;
        } else {
            right = mid - 1;
        }
        if sum.abs_diff(target) < curr_diff {
            curr_diff = sum.abs_diff(target);
            res = mid;
        }
        if sum.abs_diff(target) == curr_diff {
            res = res.min(mid);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(find_best_value(&mut [4, 9, 3], 10), 3);
        assert_eq!(find_best_value(&mut [2, 3, 5], 10), 5);
        assert_eq!(
            find_best_value(&mut [60864, 25176, 27249, 21296, 20204], 56803),
            11361
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
