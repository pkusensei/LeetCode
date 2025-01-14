mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_diagonal_order(nums: &[&[i32]]) -> Vec<i32> {
    #[derive(Clone, Copy)]
    struct Coord {
        row: usize,
        col: usize,
        val: i32,
    }
    let mut coords = vec![];
    for (row, r) in nums.iter().enumerate() {
        for (col, &val) in r.iter().enumerate() {
            coords.push(Coord { row, col, val });
        }
    }
    coords.sort_unstable_by(|a, b| {
        (a.row + a.col)
            .cmp(&(b.row + b.col))
            .then(b.row.cmp(&a.row))
    });
    coords.into_iter().map(|c| c.val).collect()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            find_diagonal_order(&[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]]),
            [1, 4, 2, 7, 5, 3, 8, 6, 9]
        );
        assert_eq!(
            find_diagonal_order(&[
                &[1, 2, 3, 4, 5],
                &[6, 7],
                &[8],
                &[9, 10, 11],
                &[12, 13, 14, 15, 16]
            ]),
            [1, 6, 2, 8, 7, 3, 9, 4, 12, 10, 5, 13, 11, 14, 15, 16]
        )
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

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
