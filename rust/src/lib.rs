mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn find_duplicate(nums: &[i32]) -> i32 {
    // binary_search(nums)
    detect_cycle(nums)
}

fn binary_search(nums: &[i32]) -> i32 {
    let (mut left, mut right) = (1, nums.len() as i32 - 1);
    while left < right {
        let mid = (right - left) / 2 + left;
        // [1, 2, 2, 3, 4] mid = 2
        // count is 3 and 3>2 => dup must be in left half
        let count = nums.iter().filter(|&&n| n <= mid).count() as i32;
        if count > mid {
            right = mid
        } else {
            left = mid + 1
        }
    }
    left
}

fn detect_cycle(nums: &[i32]) -> i32 {
    let (mut slow, mut fast) = (nums[0], nums[0]);
    loop {
        slow = nums[slow as usize];
        fast = nums[nums[fast as usize] as usize];
        if slow == fast {
            break;
        }
    }
    slow = nums[0];
    while slow != fast {
        slow = nums[slow as usize];
        fast = nums[fast as usize];
    }
    slow
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(find_duplicate(&[1, 3, 4, 2, 2]), 2);
        debug_assert_eq!(find_duplicate(&[3, 1, 3, 4, 2]), 3);
        debug_assert_eq!(find_duplicate(&[3, 3, 3, 3, 3]), 3);
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
