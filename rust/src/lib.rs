mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_steps(s: &str) -> i64 {
    let mut res = 0;
    let mut ones = 0;
    for b in s.bytes() {
        if b == b'0' {
            // A '0' has to pass all '1' on its left
            res += ones;
        } else {
            ones += 1
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
        debug_assert_eq!(minimum_steps("101"), 1);
        debug_assert_eq!(minimum_steps("100"), 2);
        debug_assert_eq!(minimum_steps("0111"), 0);
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
