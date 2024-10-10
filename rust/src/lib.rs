mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn valid_palindrome(s: &str) -> bool {
    let (s, n) = (s.as_bytes(), s.len());
    if check(s) {
        return true;
    }
    let (mut left, mut right) = (0, n - 1);
    while left < right {
        if s[left] == s[right] {
            left += 1;
            right -= 1;
        } else {
            return check(&s[left..=right - 1]) || check(&s[left + 1..=right]);
        }
    }
    true
}

fn check(s: &[u8]) -> bool {
    let n = s.len();
    if n < 2 {
        return true;
    }
    if n == 2 {
        return s[0] == s[1];
    }
    s.iter()
        .take(n / 2)
        .zip(s.iter().rev())
        .all(|(a, b)| a == b)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        // debug_assert!(valid_palindrome("aba"));
        // debug_assert!(valid_palindrome("abca"));
        debug_assert!(!valid_palindrome("abc"));
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
