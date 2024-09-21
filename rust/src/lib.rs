mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
    let attempt = minutes_to_test / minutes_to_die;
    // (1 + attempt).pow(x) >= buckets
    let mut res = 0;
    while (1 + attempt).pow(res) < buckets {
        res += 1;
    }
    res as _
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(poor_pigs(4, 15, 15), 2);
        debug_assert_eq!(poor_pigs(4, 15, 30), 2);
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
