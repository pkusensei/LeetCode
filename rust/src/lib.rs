mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn num_subarray_product_less_than_k(nums: &[i32], k: i32) -> i32 {
    if k <= 1 {
        return 0;
    }
    let (mut left, mut res) = (0, 0);
    let mut curr = 1;
    for (right, num) in nums.iter().enumerate() {
        curr *= num;
        while curr >= k {
            curr /= nums[left];
            left += 1;
        }
        res += right - left + 1;
    }
    res as _
}

fn with_log_binary_search(nums: &[i32], k: i32) -> i32 {
    if k == 0 {
        return 0;
    }
    let log_k = f64::from(k).ln();
    let n = 1 + nums.len();
    let mut prefix = Vec::with_capacity(n);
    prefix.push(0.0); // ln(1)
    for &num in nums.iter() {
        prefix.push(f64::from(num).ln() + *prefix.last().unwrap());
    }
    let mut res = 0;
    for idx in 0..n {
        let (mut left, mut right) = (idx + 1, n);
        while left < right {
            let mid = left + (right - left) / 2;
            // want to find mid such that product of [idx..mid] is k
            // i.e sum of ln([idx])..ln([mid]) == ln(k)
            if prefix[mid] - prefix[idx] < log_k - 1_e-9 {
                left = 1 + mid;
            } else {
                right = mid;
            }
        }
        res += left - idx - 1;
    }
    res as _
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(with_log_binary_search(&[10, 5, 2, 6], 100), 8);
        debug_assert_eq!(with_log_binary_search(&[1, 2, 3], 0), 0);
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
}
