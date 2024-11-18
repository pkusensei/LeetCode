mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn merge_stones(stones: &[i32], k: i32) -> i32 {
    let (n, k) = (stones.len(), k as usize);
    if (n - 1) % (k - 1) > 0 {
        return -1;
    }
    let mut prefix = Vec::with_capacity(n);
    for &num in stones.iter() {
        prefix.push(num + prefix.last().unwrap_or(&0));
    }
    let mut dp = vec![vec![vec![-1; 1 + k]; n]; n];
    dfs(0, n - 1, 1, &prefix, k, &mut dp)
}

fn dfs(
    i1: usize,
    i2: usize,
    piles: usize,
    prefix: &[i32],
    k: usize,
    dp: &mut [Vec<Vec<i32>>],
) -> i32 {
    const INF: i32 = i32::MAX / 4;
    if i1 == i2 {
        // merge [i..i] into one is noop
        // otherwise it is impossible
        return if piles == 1 { 0 } else { INF };
    }
    if dp[i1][i2][piles] > -1 {
        return dp[i1][i2][piles];
    }
    let res = if piles == 1 {
        // any merged pile was k piles before merge
        dfs(i1, i2, k, prefix, k, dp)
            + if i1 == 0 {
                prefix[i2]
            } else {
                prefix[i2] - prefix[i1 - 1]
            }
    } else {
        // or try each combination
        (i1..i2)
            .map(|t| dfs(i1, t, 1, prefix, k, dp) + dfs(1 + t, i2, piles - 1, prefix, k, dp))
            .min()
            .unwrap_or(INF)
    };
    dp[i1][i2][piles] = res;
    res
}

fn bottom_up(stones: &[i32], k: i32) -> i32 {
    let (n, k) = (stones.len(), k as usize);
    if (n - 1) % (k - 1) > 0 {
        return -1;
    }
    let mut prefix = Vec::with_capacity(1 + n);
    prefix.push(0);
    for &num in stones.iter() {
        prefix.push(num + prefix.last().unwrap_or(&0));
    }
    let mut dp = vec![vec![0; n]; n];
    for len in k..=n {
        for start in 0..=n - len {
            let end = start + len - 1;
            dp[start][end] = i32::MAX;
            for i in (start..end).step_by(k - 1) {
                dp[start][end] = dp[start][end].min(dp[start][i] + dp[1 + i][end]);
            }
            if (end - start) % (k - 1) == 0 {
                dp[start][end] += prefix[1 + end] - prefix[start];
            }
        }
    }
    dp[0][n - 1]
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(bottom_up(&[3, 2, 4, 1], 2), 20);
        debug_assert_eq!(bottom_up(&[3, 2, 4, 1], 3), -1);
        debug_assert_eq!(bottom_up(&[3, 5, 1, 2, 6], 3), 25);
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
