mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn check_possibility(nums: &[i32]) -> bool {
    let mut idx = None;
    for (i, w) in nums.windows(2).enumerate() {
        if w[0] > w[1] {
            if idx.is_some() {
                return false;
            }
            idx = Some(i);
        }
    }
    let Some(idx) = idx else {
        return true;
    };
    let n = nums.len();
    if idx == 0 || idx == n - 2 {
        return true;
    }
    // [1, 3, 2, 4], [3]=>[2]         [1,3,3,2,4], [2]=>[3 or 4]
    nums[idx - 1] <= nums[idx + 1] || nums[idx] <= nums[idx + 2]
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(check_possibility(&[4, 2, 3]));
        debug_assert!(!check_possibility(&[4, 2, 1]));
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
