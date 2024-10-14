mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn smallest_distance_pair(nums: &mut [i32], k: i32) -> i32 {
    nums.sort_unstable();
    let (mut low, mut high) = (0, nums.last().unwrap() - nums[0]);
    while low < high {
        let mid = low + (high - low) / 2;
        let count = count_pairs_with_dist(nums, mid);
        if count < k {
            low = 1 + mid;
        } else {
            high = mid
        }
    }
    low
}

fn count_pairs_with_dist(nums: &[i32], dist: i32) -> i32 {
    let mut res = 0;
    let mut left = 0;
    for (right, &num) in nums.iter().enumerate() {
        while num - nums[left] > dist {
            left += 1;
        }
        res += right - left;
    }
    res as _
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(smallest_distance_pair(&mut [1, 3, 1], 1), 0);
        debug_assert_eq!(smallest_distance_pair(&mut [1, 1, 1], 2), 0);
        debug_assert_eq!(smallest_distance_pair(&mut [1, 6, 1], 3), 5);
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
