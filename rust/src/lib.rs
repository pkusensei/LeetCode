mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_moves(nums: &[i32]) -> i32 {
    let min = nums.iter().copied().min().unwrap_or(0);
    nums.iter().fold(0, |acc, &n| acc + n - min)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(min_moves(&[1, 2, 3]), 3);
        debug_assert_eq!(min_moves(&[1, 1, 1]), 0);
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
