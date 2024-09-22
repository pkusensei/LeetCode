mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn rand10() -> i32 {
    fn rand7() -> i32 {
        // stub
        0
    }

    loop {
        let (a, b) = (rand7(), rand7());
        let num = (a - 1) * 7 + b;
        if num <= 40 {
            return (num - 1) % 10 + 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {}

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
