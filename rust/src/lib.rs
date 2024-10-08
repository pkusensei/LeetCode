mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn construct_array(n: i32, k: i32) -> Vec<i32> {
    let mut res: Vec<i32> = (1..=n).collect();
    // k=1 [1,2,3,4]
    // k=2 [1,4,3,2]
    // k=3 [1,4,2,3]
    for i in 1..k as usize {
        res[i..].reverse();
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(construct_array(3, 1), [1, 2, 3]);
        debug_assert_eq!(construct_array(3, 2), [1, 3, 2]);
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
