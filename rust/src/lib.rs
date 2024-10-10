mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn repeated_string_match(a: &str, b: &str) -> i32 {
    let n = b.len();
    let mut s = String::with_capacity(2 * n);
    while s.len() < n {
        s.push_str(a);
    }
    let count = s.len() / a.len();
    if s.contains(b) {
        return count as _;
    }
    for i in 0..count {
        s.push_str(a);
        if s.contains(b) {
            return (i + count + 1) as _;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(repeated_string_match("abcd", "cdabcdab"), 3);
        debug_assert_eq!(repeated_string_match("a", "aa"), 2);
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
