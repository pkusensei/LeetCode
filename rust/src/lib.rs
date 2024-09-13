mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn find_the_difference(s: &str, t: &str) -> char {
    // let bs = s.bytes().fold(0, |acc, b| acc + u32::from(b - b'a'));
    // let bt = t.bytes().fold(0, |acc, b| acc + u32::from(b - b'a'));
    // char::from((bt - bs) as u8 + b'a')
    s.bytes().chain(t.bytes()).fold(0, |acc, b| acc ^ b).into()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(find_the_difference("abcd", "abcde"), 'e');
        debug_assert_eq!(find_the_difference("", "y"), 'y');
    }

    #[test]
    fn test() {}

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
