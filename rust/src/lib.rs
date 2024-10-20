mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn can_transform(start: &str, end: &str) -> bool {
    let start = start.bytes().enumerate().filter(|(_, b)| *b != b'X');
    let end = end.bytes().enumerate().filter(|(_, b)| *b != b'X');
    if start.clone().count() != end.clone().count() {
        return false;
    }
    for (s, e) in start.zip(end) {
        match (s.1, e.1) {
            (b'L', b'L') => {
                if s.0 < e.0 {
                    return false;
                }
            }
            (b'R', b'R') => {
                if s.0 > e.0 {
                    return false;
                }
            }
            _ => return false,
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(can_transform("RXXLRXRXL", "XRLXXRRLX"));
        debug_assert!(!can_transform("X", "L"));
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
