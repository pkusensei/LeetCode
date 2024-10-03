mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_count(m: i32, n: i32, ops: &[[i32; 2]]) -> i32 {
    let x = ops.iter().map(|v| v[0]).min().unwrap_or(n);
    let y = ops.iter().map(|v| v[1]).min().unwrap_or(m);
    x * y
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(max_count(3, 3, &[[2, 2], [3, 3]]), 4);
        debug_assert_eq!(
            max_count(
                3,
                3,
                &[[2, 2], [3, 3], [3, 3], [2, 2], [3, 3], [2, 2], [3, 3],]
            ),
            4
        );
        debug_assert_eq!(max_count(3, 3, &[]), 9);
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
