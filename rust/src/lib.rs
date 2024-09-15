mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn longest_palindrome(s: &str) -> i32 {
    let counts = s.bytes().fold([0; 52], |mut acc, b| {
        if b >= b'a' {
            acc[usize::from(b - b'a') + 26] += 1
        } else {
            acc[usize::from(b - b'A')] += 1
        }
        acc
    });
    let odd: i32 = counts.iter().map(|&n| n & 1).sum();
    let sum: i32 = counts.into_iter().sum();
    if odd > 0 {
        sum + 1 - odd
    } else {
        sum
    }
}

#[cfg(test)]
mod tests {
    use std::{fmt::Debug, ops::DerefMut};

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(longest_palindrome("abccccdd"), 7);
        debug_assert_eq!(longest_palindrome("a"), 1);
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: DerefMut<Target = [T1]>,
        I2: DerefMut<Target = [T2]>,
    {
        i1.sort_unstable();
        i2.sort_unstable();
        debug_assert_eq!(*i1, *i2);
    }
}
