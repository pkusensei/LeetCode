mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn restore_matrix(mut row_sum: Vec<i32>, mut col_sum: Vec<i32>) -> Vec<Vec<i32>> {
    let rows = row_sum.len();
    let cols = col_sum.len();
    let mut res = vec![vec![0; cols]; rows];
    // for r in 0..rows {
    //     for c in 0..cols {
    //         let curr = row_sum[r].min(col_sum[c]);
    //         res[r][c] = curr;
    //         row_sum[r] -= curr;
    //         col_sum[c] -= curr;
    //     }
    // }
    let [mut r, mut c] = [0, 0];
    while r < rows && c < cols {
        let curr = row_sum[r].min(col_sum[c]);
        res[r][c] = curr;
        row_sum[r] -= curr;
        col_sum[c] -= curr;
        if row_sum[r] == 0 {
            r += 1
        } else {
            c += 1;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            restore_matrix(vec![5, 7, 10], vec![8, 6, 8]),
            [[5, 0, 0], [3, 4, 0], [0, 2, 8]]
        );
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
