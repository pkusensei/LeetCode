mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_max_consecutive_ones(nums: &[i32]) -> i32 {
    let (mut curr, mut res) = (0, 0);
    for &num in nums.iter() {
        if num == 1 {
            curr += 1;
            res = res.max(curr)
        } else {
            curr = 0
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(find_max_consecutive_ones(&[1, 1, 0, 1, 1, 1]), 3);
        debug_assert_eq!(find_max_consecutive_ones(&[1, 0, 1, 1, 0, 1]), 2);
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
