mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_lu_slength(a: &str, b: &str) -> i32 {
    if a == b {
        -1
    } else {
        a.len().max(b.len()) as _
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(find_lu_slength("aba", "cdc"), 3);
        debug_assert_eq!(find_lu_slength("aaa", "bbb"), 3);
        debug_assert_eq!(find_lu_slength("aaa", "aaa"), -1);
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
