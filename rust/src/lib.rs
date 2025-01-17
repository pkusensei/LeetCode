mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
    let n = bloom_day.len();
    if (m as usize) * (k as usize) > n {
        return -1;
    }
    let mut left = bloom_day.iter().copied().min().unwrap_or(1);
    let mut right = bloom_day.iter().copied().max().unwrap_or(10i32.pow(9));
    while left < right {
        let mid = (right - left) / 2 + left;
        if count(&bloom_day, mid, k) >= m {
            right = mid;
        } else {
            left = 1 + mid;
        }
    }
    left
}

fn count(nums: &[i32], mid: i32, k: i32) -> i32 {
    nums.split(|&num| num > mid)
        .map(|v| v.len() as i32 / k)
        .sum()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(min_days(vec![1, 10, 3, 10, 2], 3, 1), 3);
        assert_eq!(min_days(vec![1, 10, 3, 10, 2], 3, 2), -1);
        assert_eq!(min_days(vec![7, 7, 7, 7, 12, 7, 7], 2, 3), 12);
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
