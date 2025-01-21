mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn min_cost(n: i32, cuts: &mut [i32]) -> i32 {
    cuts.sort_unstable();
    let mut cuts = cuts.to_vec();
    cuts.insert(0, 0);
    cuts.push(n);
    let n1 = cuts.len();
    let mut dp = vec![vec![0; n1]; n1];
    for len in 2..n1 {
        for left in 0..n1 - len {
            let right = left + len;
            let mut curr = i32::MAX;
            dp[left][right] = cuts[right] - cuts[left];
            for c in 1 + left..right {
                curr = curr.min(dp[left][c] + dp[c][right]);
            }
            dp[left][right] += curr;
        }
    }
    dp[0][n1 - 1]
    // dfs(cuts, 0, n, &mut HashMap::new())
}

fn dfs(cuts: &[i32], left: i32, right: i32, dp: &mut HashMap<[i32; 2], i32>) -> i32 {
    if cuts.is_empty() {
        return 0;
    }
    if let Some(&v) = dp.get(&[left, right]) {
        return v;
    }
    let mut res = i32::MAX;
    for (i, &c) in cuts.iter().enumerate() {
        let curr = dfs(&cuts[..i], left, c, dp) + dfs(&cuts[1 + i..], c, right, dp);
        res = res.min(right - left + curr);
    }
    dp.insert([left, right], res);
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(min_cost(7, &mut [1, 3, 4, 5]), 16);
        assert_eq!(min_cost(9, &mut [5, 6, 1, 4, 2]), 22);
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
