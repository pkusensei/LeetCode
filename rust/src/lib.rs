mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_error_nums(nums: &mut [i32]) -> Vec<i32> {
    let n = nums.len();
    let sum: i32 = nums.iter().sum();
    let expected_sum = n * (n + 1) / 2;
    let mut dup = -1;
    for i in 0..n {
        let idx = (nums[i].abs() - 1) as usize;
        if nums[idx] < 0 {
            dup = nums[i].abs();
            break;
        }
        nums[idx] *= -1;
    }
    vec![dup, expected_sum as i32 - sum + dup]
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(find_error_nums(&mut [1, 2, 2, 4]), [2, 3]);
        debug_assert_eq!(find_error_nums(&mut [1, 1]), [1, 2]);
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
