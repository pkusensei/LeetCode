mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
    const PRIMES: [u32; 8] = [2, 3, 5, 7, 11, 13, 17, 19];
    (left..=right)
        .filter(|&n| PRIMES.contains(&n.count_ones()))
        .count() as _
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(count_prime_set_bits(6, 10), 4);
        debug_assert_eq!(count_prime_set_bits(10, 15), 5);
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
