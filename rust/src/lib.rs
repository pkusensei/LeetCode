mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn is_one_bit_character(bits: &[i32]) -> bool {
    let mut nums = bits;
    while nums.len() > 2 {
        let Some(n) = nums
            .strip_prefix(&[1, 0])
            .or_else(|| nums.strip_prefix(&[1, 1]))
            .or_else(|| nums.strip_prefix(&[0]))
        else {
            return false;
        };
        nums = n;
    }
    matches!(nums, [0] | [0, 0])
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(is_one_bit_character(&[1, 0, 0]));
        debug_assert!(!is_one_bit_character(&[1, 1, 1, 0]));
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
