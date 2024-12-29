mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn matrix_block_sum(mat: &[&[i32]], k: i32) -> Vec<Vec<i32>> {
    let (rows, cols) = get_dimensions(mat);
    let mut prefix: Vec<Vec<i32>> = Vec::with_capacity(1 + rows);
    prefix.push(vec![0; 1 + cols]);
    for (row, r) in mat.iter().enumerate() {
        let mut curr = Vec::with_capacity(1 + cols);
        curr.push(0);
        for &num in r.iter() {
            curr.push(num + curr.last().unwrap_or(&0));
        }
        for (c, v) in curr.iter_mut().enumerate() {
            *v += prefix[row][c];
        }
        prefix.push(curr);
    }
    let mut res = vec![vec![0; cols]; rows];
    let k = k as usize;
    for r in 0..rows {
        for c in 0..cols {
            let up = r.saturating_sub(k);
            let down = (r + k).min(rows - 1);
            let left = c.saturating_sub(k);
            let right = (c + k).min(cols - 1);
            res[r][c] =
                prefix[1 + down][1 + right] - prefix[1 + down][left] - prefix[up][1 + right]
                    + prefix[up][left];
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
            matrix_block_sum(&[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]], 1),
            [[12, 21, 16], [27, 45, 33], [24, 39, 28]]
        );
        assert_eq!(
            matrix_block_sum(&[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]], 2),
            [[45, 45, 45], [45, 45, 45], [45, 45, 45]]
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
