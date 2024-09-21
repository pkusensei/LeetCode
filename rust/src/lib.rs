mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_moves2(nums: &mut [i32]) -> i32 {
    let n = nums.len();
    if n < 2 {
        return 0;
    }
    let mid = *nums.select_nth_unstable(n / 2).1;
    nums.iter().map(|&n| n.abs_diff(mid)).sum::<u32>() as _
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(min_moves2(&mut [1, 2, 3]), 2);
        debug_assert_eq!(min_moves2(&mut [1, 10, 2, 9]), 16);
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
