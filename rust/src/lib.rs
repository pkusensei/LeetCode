mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_width_ramp(nums: &[i32]) -> i32 {
    // let mut stack = vec![];
    // for (i, &num) in nums.iter().enumerate() {
    //     if stack.is_empty() || stack.last().is_some_and(|&(_, v)| v > num) {
    //         stack.push((i, num));
    //     }
    // }
    // let mut res = 0;
    // for (right, &num) in nums.iter().enumerate().rev() {
    //     while stack.last().is_some_and(|&(_, v)| v <= num) {
    //         let Some((left, _)) = stack.pop() else {
    //             break;
    //         };
    //         res = res.max(right - left);
    //     }
    // }
    // res as _
    let n = nums.len();
    let (mut mins, mut maxs) = (vec![0; n], vec![0; n]);
    mins[0] = nums[0];
    for (i, &n) in nums.iter().enumerate().skip(1) {
        mins[i] = mins[i - 1].min(n);
    }
    maxs[n - 1] = nums[n - 1];
    for (i, &n) in nums.iter().enumerate().rev().skip(1) {
        maxs[i] = maxs[1 + i].max(n);
    }
    let (mut left, mut right) = (0, 0);
    let mut res = 0;
    while right < n {
        if mins[left] <= maxs[right] {
            res = res.max(right - left);
            right += 1
        } else {
            left += 1
        }
    }
    res as _
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(max_width_ramp(&[6, 0, 8, 2, 1, 5]), 4);
        debug_assert_eq!(max_width_ramp(&[9, 8, 1, 0, 1, 9, 4, 0, 4, 1]), 7);
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
