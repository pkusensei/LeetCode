mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
    // [1, 10, 91, 739, 5275, 32491, 168571, 712891, 2345851]
    if n < 1 {
        1
    } else {
        let mut res = 9;
        let mut curr = 9;
        for _ in 1..n {
            res *= curr;
            curr -= 1;
        }
        res + count_numbers_with_unique_digits(n - 1)
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(count_numbers_with_unique_digits(2), 91);
        debug_assert_eq!(count_numbers_with_unique_digits(0), 1);
        debug_assert_eq!(count_numbers_with_unique_digits(4), 5275);
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
