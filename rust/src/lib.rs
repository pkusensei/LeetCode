mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn max_number(nums1: &[i32], nums2: &[i32], k: i32) -> Vec<i32> {
    let k = k as usize;
    let mut res = vec![];
    for k1 in 0..=nums1.len() {
        let k2 = k.saturating_sub(k1).min(nums2.len());
        if k1 + k2 != k {
            continue;
        }
        let v1 = most_competitive(nums1, k1, |a, b| a > b);
        let v2 = most_competitive(nums2, k2, |a, b| a > b);
        res.push(largest_merge(&v1, &v2));
    }
    res.into_iter().max().unwrap()
}

// 1673
pub fn most_competitive(nums: &[i32], k: usize, pred: fn(i32, i32) -> bool) -> Vec<i32> {
    if k == 0 {
        return vec![];
    }
    // monotonic stack
    let mut stack = Vec::with_capacity(k);
    for (i, &num) in nums.iter().enumerate() {
        while stack
            .last()
            .is_some_and(|&n| pred(num, n) && nums.len() - i > k - stack.len())
        {
            // nums.len() - i => amount of num left to choose from
            // k - stack.len() => amount of num missing in res
            stack.pop();
        }
        // Only keeps the largest k numbers
        if stack.len() < k {
            stack.push(num);
        }
    }
    stack
}

// 1754
pub fn largest_merge<T>(mut s1: &[T], mut s2: &[T]) -> Vec<T>
where
    T: Copy + PartialOrd,
{
    let mut res = Vec::with_capacity(s1.len() + s2.len());
    while !s1.is_empty() && !s2.is_empty() {
        if s1 > s2 {
            res.push(s1[0]);
            s1 = &s1[1..];
        } else {
            res.push(s2[0]);
            s2 = &s2[1..];
        }
    }
    res.extend_from_slice(s1);
    res.extend_from_slice(s2);
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        // 1673
        debug_assert_eq!(most_competitive(&[3, 5, 2, 6], 2, |a, b| a < b), [2, 6]);
        debug_assert_eq!(
            most_competitive(&[2, 4, 3, 3, 5, 4, 9, 6], 4, |a, b| a < b),
            [2, 3, 3, 4]
        );

        // 1754
        debug_assert_eq!(largest_merge(b"cabaa", b"bcaaa"), b"cbcabaaaaa");
        debug_assert_eq!(largest_merge(b"abcabc", b"abdcaba"), b"abdcabcabcaba");

        // 321
        debug_assert_eq!(
            max_number(&[3, 4, 6, 5], &[9, 1, 2, 5, 8, 3], 5),
            [9, 8, 6, 5, 3]
        );
        debug_assert_eq!(max_number(&[6, 7], &[6, 0, 4], 5), [6, 7, 6, 0, 4]);
        debug_assert_eq!(max_number(&[3, 9], &[8, 9], 3), [9, 8, 9]);
    }

    #[test]
    fn test() {}

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
