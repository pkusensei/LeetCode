mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn is_toeplitz_matrix(matrix: &[&[i32]]) -> bool {
    for (y, row) in matrix.iter().enumerate() {
        for (x, &n1) in row.iter().enumerate() {
            if matrix
                .get(1 + y)
                .and_then(|r| r.get(1 + x))
                .is_some_and(|&n2| n2 != n1)
            {
                return false;
            }
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
        debug_assert!(is_toeplitz_matrix(&[
            &[1, 2, 3, 4],
            &[5, 1, 2, 3],
            &[9, 5, 1, 2]
        ]));
        debug_assert!(!is_toeplitz_matrix(&[&[1, 2], &[2, 2]]));
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
