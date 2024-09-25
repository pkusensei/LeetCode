mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn reverse_pairs(nums: &mut [i32]) -> i32 {
    merge_sort(nums)
}

fn merge_sort(nums: &mut [i32]) -> i32 {
    let n = nums.len();
    let mut res = 0;
    if n < 2 {
        return res;
    }

    let mid = n / 2;
    res += merge_sort(&mut nums[..mid]);
    res += merge_sort(&mut nums[mid..]);

    let mut right = mid;
    for left in 0..mid {
        while right < n && i64::from(nums[left]) > 2 * i64::from(nums[right]) {
            right += 1;
        }
        res += (right - mid) as i32;
    }
    let (mut left, mut right) = (0, mid);
    let mut sorted = Vec::with_capacity(n);
    while left < mid && right < n {
        if nums[left] <= nums[right] {
            sorted.push(nums[left]);
            left += 1
        } else {
            sorted.push(nums[right]);
            right += 1
        }
    }
    sorted.extend_from_slice(&nums[left..mid]);
    sorted.extend_from_slice(&nums[right..]);
    nums.copy_from_slice(&sorted);
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(reverse_pairs(&mut [1, 3, 2, 3, 1]), 2);
        debug_assert_eq!(reverse_pairs(&mut [2, 4, 3, 5, 1]), 3);
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
