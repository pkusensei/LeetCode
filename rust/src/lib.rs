mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

// 1 2 3 4
// 1 2
// 4 3
// 1 2 3 <-> 1 3 4
// 1 2 4 <-> 2 3 4
pub fn min_score_triangulation(values: &[i32]) -> i32 {
    let n = values.len();
    // dfs(values, 0, n - 1, &mut vec![vec![-1; n]; n])
    let mut dp = vec![vec![0; n]; n];
    for len in 2..n {
        for i1 in 0..n - len {
            let i2 = i1 + len;
            dp[i1][i2] = i32::MAX;
            for k in 1 + i1..i2 {
                let score = dp[i1][k] + dp[k][i2] + values[i1] * values[k] * values[i2];
                dp[i1][i2] = dp[i1][i2].min(score);
            }
        }
    }
    dp[0][n - 1]
}

fn dfs(values: &[i32], start: usize, end: usize, dp: &mut [Vec<i32>]) -> i32 {
    let n = end - start + 1;
    if n < 3 {
        return 0;
    }
    if dp[start][end] > -1 {
        return dp[start][end];
    }
    let mut res = i32::MAX;
    for k in 1 + start..end {
        let area = values[start] * values[end] * values[k];
        res = res.min(area + dfs(values, start, k, dp) + dfs(values, k, end, dp))
    }
    dp[start][end] = res;
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(min_score_triangulation(&[1, 2, 3]), 6);
        debug_assert_eq!(min_score_triangulation(&[3, 7, 4, 5]), 144);
        debug_assert_eq!(min_score_triangulation(&[1, 3, 1, 4, 1, 5]), 13);
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
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
