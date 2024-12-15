mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let (rows, cols) = get_dimensions(&grid);
    let mut nums: Vec<_> = grid.into_iter().flatten().collect();
    let n = nums.len();
    nums.rotate_right(k as usize % n);
    let mut res = vec![vec![0; cols]; rows];
    for (row, r) in res.iter_mut().enumerate() {
        for (col, v) in r.iter_mut().enumerate() {
            *v = nums[row * cols + col]
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
            shift_grid(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 1),
            [[9, 1, 2], [3, 4, 5], [6, 7, 8]]
        );
        assert_eq!(
            shift_grid(
                vec![
                    vec![3, 8, 1, 9],
                    vec![19, 7, 2, 5],
                    vec![4, 6, 11, 10],
                    vec![12, 0, 21, 13]
                ],
                4
            ),
            [[12, 0, 21, 13], [3, 8, 1, 9], [19, 7, 2, 5], [4, 6, 11, 10]]
        );
        assert_eq!(
            shift_grid(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 9),
            [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
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
            "left = {a:?}, right = {b:?}",
        );
    }
}
