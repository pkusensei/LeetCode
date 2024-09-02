mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn h_index(nums: &[i32]) -> i32 {
    let (mut left, mut right) = (0, nums.len() - 1);
    let mut res = 0;
    while left <= right {
        let mid = (right - left) / 2 + left;
        if nums.len() - mid <= nums[mid] as usize {
            res = nums.len() - mid;
            if mid == 0 {
                break;
            }
            right = mid - 1
        } else {
            left = mid + 1;
        }
    }
    res as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(h_index(&[0, 1, 3, 5, 6]), 3);
        debug_assert_eq!(h_index(&[1, 2, 100]), 2);
    }

    #[test]
    fn test() {
        debug_assert_eq!(h_index(&[1]), 1);
    }
}
