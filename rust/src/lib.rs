mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn has_alternating_bits(mut n: i32) -> bool {
    let mut prev = -1;
    while n > 0 {
        let curr = n & 1;
        if curr == prev {
            return false;
        }
        prev = curr;
        n >>= 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(has_alternating_bits(5));
        debug_assert!(!has_alternating_bits(7));
        debug_assert!(!has_alternating_bits(11));
    }

    #[test]
    fn test() {
        debug_assert!(!has_alternating_bits(4));
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
