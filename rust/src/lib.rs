mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn check_perfect_number(num: i32) -> bool {
    if num == 1 {
        return false;
    }
    let mut sum = 1;
    for n in 2..=f64::from(num).sqrt() as i32 {
        if num % n == 0 {
            sum += n + num / n;
        }
    }
    sum == num
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(check_perfect_number(28));
        debug_assert!(!check_perfect_number(7));
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
