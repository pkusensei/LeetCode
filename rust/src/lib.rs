mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn stone_game_v(stone_value: &[i32]) -> i32 {
    let n = stone_value.len();
    // Cannot even reason about this...
    // dfs+memo is the way to go I guess
    let mut dp = vec![vec![0; n]; n];
    let mut max = vec![vec![0; n]; n];
    for (i, &v) in stone_value.iter().enumerate() {
        max[i][i] = v;
    }
    for right in 1..n {
        let mut mid = right;
        let mut sum = stone_value[right];
        let mut right_half = 0;
        for left in (0..right).rev() {
            sum += stone_value[left];
            while 2 * (right_half + stone_value[mid]) <= sum {
                right_half += stone_value[mid];
                mid -= 1;
            }
            dp[left][right] = if 2 * right_half == sum {
                max[left][mid]
            } else if mid == left {
                0
            } else {
                max[left][mid - 1]
            };
            dp[left][right] =
                dp[left][right].max(if mid == right { 0 } else { max[right][mid + 1] });
            max[left][right] = max[left][right - 1].max(dp[left][right] + sum);
            max[right][left] = max[right][left + 1].max(dp[left][right] + sum);
        }
    }
    dp[0][n - 1]
    // let mut prefix = Vec::with_capacity(n);
    // for &num in stone_value.iter() {
    //     prefix.push(num + prefix.last().unwrap_or(&0));
    // }
    // dfs(&prefix, 0, n - 1, &mut vec![vec![-1; n]; n])
}

fn dfs(prefix: &[i32], start: usize, end: usize, memo: &mut [Vec<i32>]) -> i32 {
    if start == end {
        return 0;
    }
    if memo[start][end] > -1 {
        return memo[start][end];
    }
    let mut res = 0;
    for i in 1 + start..=end {
        let left = prefix[i - 1] - if start > 0 { prefix[start - 1] } else { 0 };
        let right = prefix[end] - prefix[i - 1];
        match left.cmp(&right) {
            std::cmp::Ordering::Less => res = res.max(left + dfs(prefix, start, i - 1, memo)),
            std::cmp::Ordering::Equal => {
                res = res
                    .max(left + dfs(prefix, start, i - 1, memo))
                    .max(right + dfs(prefix, i, end, memo))
            }
            std::cmp::Ordering::Greater => res = res.max(right + dfs(prefix, i, end, memo)),
        }
    }
    memo[start][end] = res;
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(stone_game_v(&[6, 2, 3, 4, 5, 5]), 18);
        assert_eq!(stone_game_v(&[7, 7, 7, 7, 7, 7, 7]), 28);
        assert_eq!(stone_game_v(&[4]), 0);
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
