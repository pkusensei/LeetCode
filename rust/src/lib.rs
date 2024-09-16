mod helper;

use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(unused_imports)]
use helper::*;

pub fn find_min_difference(time_points: &[&str]) -> i32 {
    let mut nums: Vec<_> = time_points
        .iter()
        .map(|s| {
            if let Some((h, m)) = s.split_once(':') {
                let h = h.parse().unwrap_or(0);
                let m = m.parse().unwrap_or(0);
                h * 60 + m
            } else {
                0
            }
        })
        .collect();
    nums.sort_unstable();
    nums.push(nums[0]);
    let mut deltas: BinaryHeap<_> = nums
        .windows(2)
        .map(|w: &[i32]| {
            let temp = w[0].abs_diff(w[1]);
            if temp > 12 * 60 {
                Reverse(24 * 60 - temp)
            } else {
                Reverse(temp)
            }
        })
        .collect();
    deltas.pop().unwrap().0 as _
}

#[cfg(test)]
mod tests {
    use std::{fmt::Debug, ops::DerefMut};

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(find_min_difference(&["23:59", "00:00"]), 1);
        debug_assert_eq!(find_min_difference(&["00:00", "23:59", "00:00"]), 0);
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: DerefMut<Target = [T1]>,
        I2: DerefMut<Target = [T2]>,
    {
        i1.sort_unstable();
        i2.sort_unstable();
        debug_assert_eq!(*i1, *i2);
    }
}
