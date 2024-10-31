mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

// all palindrome numbers with even length is divisible by 11
pub fn prime_palindrome(n: i32) -> i32 {
    if (8..=11).contains(&n) {
        return 11;
    }
    for i in 1..100_000 {
        let mut s = i.to_string();
        let rev: String = s.chars().rev().skip(1).collect();
        s.push_str(&rev);
        let Ok(x) = s.parse::<i32>() else {
            continue;
        };
        if x >= n && is_prime(x) {
            return x;
        }
    }
    -1
}

const fn is_prime(n: i32) -> bool {
    if n <= 2 || n & 1 == 0 {
        return n == 2;
    }
    let mut x = 3;
    while x * x <= n {
        if n % x == 0 {
            return false;
        }
        x += 2
    }
    true
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(prime_palindrome(6), 7);
        debug_assert_eq!(prime_palindrome(8), 11);
        debug_assert_eq!(prime_palindrome(13), 101);
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

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64>,
        T2: Into<f64>,
    {
        const EP: f64 = 1e-5;
        debug_assert!((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP);
    }
}
