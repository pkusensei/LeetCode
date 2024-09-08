mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn count_range_sum(nums: &[i32], lower: i32, upper: i32) -> i32 {
    let n = nums.len();
    let mut prefix = Vec::with_capacity(n);
    prefix.push(nums[0].into());
    for (i, &v) in nums.iter().enumerate().skip(1) {
        prefix.push(prefix[i - 1] + i64::from(v));
    }
    count_merge_sort(&mut prefix, lower, upper)
        + nums.iter().filter(|&n| (lower..=upper).contains(n)).count() as i32
}

fn count_merge_sort(values: &mut [i64], lower: i32, upper: i32) -> i32 {
    let mid = values.len() / 2;
    if mid == 0 {
        return 0; // exclude when len <= 1
    }
    let mut count = count_merge_sort(&mut values[..mid], lower, upper)
        + count_merge_sort(&mut values[mid..], lower, upper);
    let (mut low, mut high) = (mid, mid);
    for &num in values[..mid].iter() {
        while values.get(low).is_some_and(|&v| v - num < i64::from(lower)) {
            low += 1 // the first index that lower <= delta
        }
        while values
            .get(high)
            .is_some_and(|&v| v - num <= i64::from(upper))
        {
            high += 1 // the last index that delta <= upper
        }
        count += (high - low) as i32;
    }
    values.sort(); // slice is partially sorted
    count
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(count_range_sum(&[-2, 5, -1], -2, 2), 3);
        debug_assert_eq!(count_range_sum(&[0], 0, 0), 1);
    }

    #[test]
    fn test() {
        debug_assert_eq!(count_range_sum(&[0, -3, -3, 1, 1, 2], 3, 5), 2);
    }

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(i1: I1, i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: IntoIterator<Item = T1>,
        I2: IntoIterator<Item = T2>,
    {
        let mut v1: Vec<_> = i1.into_iter().collect();
        let mut v2: Vec<_> = i2.into_iter().collect();
        v1.sort_unstable();
        v2.sort_unstable();
        debug_assert_eq!(v1, v2);
    }
}
