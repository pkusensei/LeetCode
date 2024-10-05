mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn check_inclusion(s1: &str, s2: &str) -> bool {
    let n = s1.len();
    if n > s2.len() {
        return false;
    }
    let mut counts = s1.bytes().fold([0; 26], |mut acc, b| {
        acc[usize::from(b - b'a')] += 1;
        acc
    });
    for b in s2[..n].bytes() {
        counts[usize::from(b - b'a')] -= 1;
    }
    if counts.iter().all(|&n| n == 0) {
        return true;
    }
    let s2 = s2.as_bytes();
    for i in 1..=s2.len() - n {
        counts[usize::from(s2[i - 1] - b'a')] += 1;
        counts[usize::from(s2[i + n - 1] - b'a')] -= 1;
        if counts.iter().all(|&n| n == 0) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(check_inclusion("ab", "eidbaooo"));
        debug_assert!(!check_inclusion("ab", "eidboaoo"));
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
