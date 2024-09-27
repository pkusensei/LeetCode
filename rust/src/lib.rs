mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn check_subarray_sum(nums: &[i32], k: i32) -> bool {
    let mut seen = HashMap::from([(0, -1)]);
    let mut prev = 0;
    // prefix sum, but with sum%k
    // so that ...a,b..a...
    //             [^..^] is the subarray wanted
    for (i, &num) in nums.iter().enumerate() {
        prev = (prev + num) % k;
        if let Some(&v) = seen.get(&prev) {
            if i as i32 - v > 1 {
                return true;
            }
        } else {
            seen.insert(prev, i as i32);
        }
    }
    false
}
#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(check_subarray_sum(&[23, 2, 4, 6, 7], 6));
        debug_assert!(check_subarray_sum(&[23, 2, 6, 4, 7], 6));
        debug_assert!(!check_subarray_sum(&[23, 2, 6, 4, 7], 13));
    }

    #[test]
    fn test() {
        debug_assert!(check_subarray_sum(&[5, 0, 0, 0], 3));
    }

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
