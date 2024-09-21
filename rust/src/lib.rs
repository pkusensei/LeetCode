mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn repeated_substring_pattern(s: &str) -> bool {
    let (s, n) = (s.as_bytes(), s.len());
    let mut failure = vec![0; n];
    for i in 1..n {
        let mut j = failure[i - 1];
        while j > 0 && s[i] != s[j] {
            j = failure[j - 1]
        }
        failure[i] = j + usize::from(s[i] == s[j]);
    }
    let repeated = failure[n - 1];
    repeated > 0 && n % (n - repeated) == 0
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(repeated_substring_pattern("abab"));
        debug_assert!(!repeated_substring_pattern("aba"));
        debug_assert!(repeated_substring_pattern("abcabcabcabc"));
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
