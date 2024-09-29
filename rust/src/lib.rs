mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn reverse_str(s: String, k: i32) -> String {
    if k < 2 {
        return s;
    }
    let mut s = s.into_bytes();
    let k = k as usize;
    if k > s.len() {
        s.reverse();
        return String::from_utf8(s).unwrap();
    }
    let (mut left, mut right) = (0, k);
    while right < s.len() {
        s[left..right].reverse();
        left += 2 * k;
        right = (right + 2 * k).min(s.len());
    }
    if left < s.len() {
        s[left..].reverse();
    }
    String::from_utf8(s).unwrap()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(reverse_str("abcdefg".into(), 2), "bacdfeg");
        debug_assert_eq!(reverse_str("abcd".into(), 2), "bacd");
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
