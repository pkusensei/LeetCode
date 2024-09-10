mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn reverse_vowels(s: &str) -> String {
    const VOWELS: &[u8; 10] = b"AEIOUaeiou";
    let mut s: Vec<_> = s.bytes().collect();
    let (mut left, mut right) = (0, s.len() - 1);
    while left < right {
        while left < s.len() && !VOWELS.contains(&s[left]) {
            left += 1;
        }
        while 0 < right && !VOWELS.contains(&s[right]) {
            right -= 1;
        }
        if left < s.len() && left < right {
            // usize guarantees 0<=right
            s.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
    s.into_iter().map(char::from).collect()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(reverse_vowels("hello"), "holle");
        debug_assert_eq!(reverse_vowels("leetcode"), "leotcede");
    }

    #[test]
    fn test() {
        debug_assert_eq!(reverse_vowels(".,"), ".,");
    }

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
