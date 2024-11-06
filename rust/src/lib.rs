mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_subarray_sum_circular(nums: &[i32]) -> i32 {
    let [mut min, mut curr_min] = [30_001; 2];
    let [mut max, mut curr_max] = [-30_001; 2];
    let mut sum = 0;
    for &num in nums.iter() {
        curr_min = num.min(curr_min + num);
        curr_max = num.max(curr_max + num);
        min = min.min(curr_min);
        max = max.max(curr_max);
        sum += num;
    }
    if max < 0 {
        // Everything is negative, returns max
        max
    } else {
        // max => the max sum in original array
        // (sum-min) => max sum in wrapped around array
        // as in, dodged the min part
        max.max(sum - min)
    }
}

fn prefix_suffix(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut suffix = Vec::with_capacity(n);
    let mut suff_sum = nums[n - 1];
    suffix.push(nums[n - 1]);
    for &num in nums.iter().rev().skip(1) {
        suff_sum += num;
        suffix.push(suff_sum.max(*suffix.last().unwrap()));
    }
    suffix.reverse();

    let [mut max, mut special] = [nums[0]; 2];
    let [mut prefix, mut curr] = [0; 2];
    for (idx, &num) in nums.iter().enumerate() {
        curr = num.max(curr + num);
        max = max.max(curr);
        prefix += num;
        if 1 + idx < n {
            special = special.max(prefix + suffix[1 + idx]);
        }
    }
    max.max(special)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(prefix_suffix(&[1, -2, 3, -2]), 3);
        debug_assert_eq!(prefix_suffix(&[5, -3, 5]), 10);
        debug_assert_eq!(prefix_suffix(&[-3, -2, -3]), -2);
    }

    #[test]
    fn test() {
        debug_assert_eq!(prefix_suffix(&[2, -2, 2, 7, 8, 0]), 19);
    }

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
        T1: Into<f64>,
        T2: Into<f64>,
    {
        const EP: f64 = 1e-5;
        debug_assert!((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP);
    }
}
