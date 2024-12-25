mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_side_length(mat: &[&[i32]], threshold: i32) -> i32 {
    let (rows, cols) = get_dimensions(mat);
    let mut prefix = Vec::with_capacity(1 + rows);
    prefix.push(vec![0; 1 + cols]);
    for (r, row) in mat.iter().enumerate() {
        let mut curr = Vec::with_capacity(1 + cols);
        curr.push(0);
        for num in row.iter() {
            curr.push(num + curr.last().unwrap_or(&0));
        }
        for (i, v) in curr.iter_mut().enumerate() {
            *v += prefix[r][i];
        }
        prefix.push(curr);
    }
    let mut res = 0;
    for r in 0..rows {
        for c in 0..cols {
            let mut left = res;
            let mut right = (rows - r).min(cols - c);
            while left <= right {
                let mid = left + (right - left) / 2;
                let curr = prefix[r + mid][c + mid] - prefix[r][c + mid] - prefix[r + mid][c]
                    + prefix[r][c];
                if curr <= threshold {
                    res = res.max(mid);
                    left = 1 + mid;
                } else {
                    right = mid - 1;
                }
            }
        }
    }
    res as _
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            max_side_length(
                &[
                    &[1, 1, 3, 2, 4, 3, 2],
                    &[1, 1, 3, 2, 4, 3, 2],
                    &[1, 1, 3, 2, 4, 3, 2]
                ],
                4
            ),
            2
        );
        assert_eq!(
            max_side_length(
                &[
                    &[2, 2, 2, 2, 2],
                    &[2, 2, 2, 2, 2],
                    &[2, 2, 2, 2, 2],
                    &[2, 2, 2, 2, 2],
                    &[2, 2, 2, 2, 2]
                ],
                1
            ),
            0
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            max_side_length(
                &[
                    &[18, 70],
                    &[61, 1],
                    &[25, 85],
                    &[14, 40],
                    &[11, 96],
                    &[97, 96],
                    &[63, 45]
                ],
                10000
            ),
            2
        );
    }

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
