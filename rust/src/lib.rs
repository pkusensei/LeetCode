mod helper;

#[allow(unused_imports)]
use helper::*;

fn is_power_of_four(n: i32) -> bool {
        const POW4: i32 = 0b_0101_0101_0101_0101_0101_0101_0101_0101;
        const MASK: i32 = POW4 << 1;
        n > 0 && n & (n - 1) == 0 && n & MASK == 0
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(is_power_of_four(16));
        debug_assert!(!is_power_of_four(5));
        debug_assert!(is_power_of_four(1));
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
