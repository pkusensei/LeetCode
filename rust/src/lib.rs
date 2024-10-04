mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_product(nums: &mut [i32]) -> i32 {
    nums.sort_unstable();
    nums.iter()
        .rev()
        .take(3)
        .product::<i32>()
        .max(nums[0] * nums[1] * nums.last().unwrap())
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(maximum_product(&mut [1, 2, 3]), 6);
        debug_assert_eq!(maximum_product(&mut [1, 2, 3, 4]), 24);
        debug_assert_eq!(maximum_product(&mut [-1, -2, -3]), -6);
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
