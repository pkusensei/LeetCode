mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_length(s: &str) -> i32 {
    let mut stack = vec![];
    for b in s.bytes() {
        match b {
            b'B' if stack.last().is_some_and(|&v| v == b'A') => {
                stack.pop();
            }
            b'D' if stack.last().is_some_and(|&v| v == b'C') => {
                stack.pop();
            }
            _ => stack.push(b),
        }
    }
    stack.len() as _
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(min_length("ABFCACDB"), 2);
        debug_assert_eq!(min_length("ACBBD"), 5);
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
