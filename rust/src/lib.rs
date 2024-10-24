mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn xor_game(nums: &[i32]) -> bool {
    let x = nums.iter().fold(0, |acc, n| acc ^ n);
    if x == 0 {
        return true;
    }
    // When nums.len() is even and x!=0
    // (n1^n2^...) != 0
    // (x^n1) ^ (x^n2) ^.. != 0
    // x^..^x != 0 => an even number of x xoring itself is zero is impossible
    // There must be a number ni where x^ni != 0
    // Thus whoever faces this even length turn wins
    nums.len() & 1 == 0
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(!xor_game(&[1, 1, 2]));
        debug_assert!(xor_game(&[0, 1]));
        debug_assert!(xor_game(&[1, 2, 3]))
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
