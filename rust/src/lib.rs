mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
    (left..=right).filter(|&num| check(num)).collect()
}

fn check(num: i32) -> bool {
    let mut n = num;
    while n > 0 {
        let d = n % 10;
        if d == 0 || num % d > 0 {
            return false;
        }
        n /= 10
    }
    true
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            self_dividing_numbers(1, 22),
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22]
        );
        debug_assert_eq!(self_dividing_numbers(47, 85), [48, 55, 66, 77]);
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
