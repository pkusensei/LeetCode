mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn get_sum(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let carry = a & b;
        a ^= b; // sum without carry
        b = carry << 1;
    }
    a
}

// 2&3
// carry = a & b  10&11=10     1&100=0
// a = a ^ b      10^11=1      1^100=101
// b = carry << 1 100

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(get_sum(2, 3), 5);
        debug_assert_eq!(get_sum(1, 2), 3);
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(i1: I1, i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: IntoIterator<Item = T1>,
        I2: IntoIterator<Item = T2>,
    {
        let mut v1: Vec<_> = i1.into_iter().collect();
        let mut v2: Vec<_> = i2.into_iter().collect();
        v1.sort_unstable();
        v2.sort_unstable();
        debug_assert_eq!(v1, v2);
    }
}
