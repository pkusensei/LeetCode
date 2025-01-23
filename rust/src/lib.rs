mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_product_path(grid: &[&[i32]]) -> i32 {
    let [rows, cols] = get_dimensions(grid);
    let mut dp = vec![vec![[i64::MAX, i64::MIN]; cols]; rows];
    dp[0][0] = [0, 1].map(|_| i64::from(grid[0][0]));
    for (r, row) in grid.iter().enumerate() {
        for (c, &v) in row.iter().enumerate() {
            if let Some(pr) = r.checked_sub(1) {
                let [pmin, pmax] = dp[pr][c];
                let [min, max] = if v < 0 {
                    [pmax, pmin].map(|x| x * i64::from(v))
                } else {
                    [pmin, pmax].map(|x| x * i64::from(v))
                };
                dp[r][c][0] = dp[r][c][0].min(min);
                dp[r][c][1] = dp[r][c][1].max(max);
            }
            if let Some(pc) = c.checked_sub(1) {
                let [pmin, pmax] = dp[r][pc];
                let [min, max] = if v < 0 {
                    [pmax, pmin].map(|x| x * i64::from(v))
                } else {
                    [pmin, pmax].map(|x| x * i64::from(v))
                };
                dp[r][c][0] = dp[r][c][0].min(min);
                dp[r][c][1] = dp[r][c][1].max(max);
            }
        }
    }
    let max = dp[rows - 1][cols - 1][1];
    if max < 0 {
        -1
    } else {
        (max % 1_000_000_007) as i32
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            max_product_path(&[&[-1, -2, -3], &[-2, -3, -3], &[-3, -3, -2]]),
            -1
        );
        assert_eq!(
            max_product_path(&[&[1, -2, 1], &[1, -2, 1], &[3, -4, 1]]),
            8
        );
        assert_eq!(max_product_path(&[&[1, 3], &[0, -4]]), 0);
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
