mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_width_of_vertical_area(mut points: Vec<Vec<i32>>) -> i32 {
    points.sort_unstable_by_key(|p| p[0]);
    points
        .windows(2)
        .map(|w| w[1][0] - w[0][0])
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    #[test]
    fn basics() {
        assert_eq!(matrix_rank_transform(&[&[1, 2], &[3, 4]]), [[1, 2], [2, 3]]);
        assert_eq!(matrix_rank_transform(&[&[7, 7], &[7, 7]]), [[1, 1], [1, 1]]);
        assert_eq!(
            matrix_rank_transform(&[&[20, -21, 14], &[-19, 4, 19], &[22, -47, 24], &[-19, 4, 19]]),
            [[4, 2, 3], [1, 3, 4], [5, 1, 6], [1, 3, 4]]
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            matrix_rank_transform(&[
                &[-37, -50, -3, 44],
                &[-37, 46, 13, -32],
                &[47, -42, -3, -40],
                &[-17, -22, -39, 24]
            ]),
            [[2, 1, 4, 6], [2, 6, 5, 4], [5, 2, 4, 3], [4, 3, 1, 5]]
        );
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
