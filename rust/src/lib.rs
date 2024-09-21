mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn circular_array_loop(nums: &mut [i32]) -> bool {
    let n = nums.len();
    if n < 2 {
        return false;
    }
    for i in 0..nums.len() {
        if nums[i] == 0 {
            continue;
        }
        let (mut slow, mut fast) = (i, next(nums, i));
        while nums[i] * nums[fast] > 0 && nums[i] * nums[next(nums, fast)] > 0 {
            if slow == fast {
                if slow == next(nums, slow) {
                    break;
                }
                return true;
            }
            slow = next(nums, slow);
            fast = next(nums, next(nums, fast));
        }

        let dir = i32::from(nums[i] > 0); // direction
        let mut curr = i;
        while dir * nums[curr] > 0 {
            let temp = next(nums, curr);
            nums[curr] = 0;
            curr = temp
        }
    }
    false
}

const fn next(nums: &[i32], start: usize) -> usize {
    let n = nums.len() as i32;
    let res = ((start as i32 + nums[start] % n) + n) % n;
    res as _
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(circular_array_loop(&mut [2, -1, 1, 2, 2]));
        debug_assert!(!circular_array_loop(&mut [-1, -2, -3, -4, -5, 6]));
        debug_assert!(circular_array_loop(&mut [1, -1, 5, 1, 4]));
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
