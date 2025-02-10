mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn rearrange_sticks(n: i32, k: i32) -> i32 {
    let [n, k] = [n, k].map(|v| v as usize);
    let mut dp = vec![vec![0; 1 + k]; 1 + n];
    dp[0][0] = 1;
    for i1 in 1..=n {
        for i2 in 1..=i1.min(k) {
            dp[i1][i2] = (dp[i1 - 1][i2 - 1] + (i1 as i64 - 1) * dp[i1 - 1][i2]) % MOD;
        }
    }
    dp[n][k] as _
    // dfs(n, k, &mut vec![vec![-1; 1 + k]; 1 + n]) as _
}

const MOD: i64 = 1_000_000_007;

fn dfs(n: usize, k: usize, memo: &mut [Vec<i64>]) -> i64 {
    if k == 0 {
        return 0;
    }
    if n <= k {
        return 1;
    }
    if memo[n][k] > -1 {
        return memo[n][k];
    }
    // (n-1, k-1) => (n, k) => Put nth stick at last
    let mut res = dfs(n - 1, k - 1, memo);
    // Put any other stick at last
    res += (n as i64 - 1) * dfs(n - 1, k, memo) % MOD;
    res %= MOD;
    memo[n][k] = res;
    res
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {{
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        }};
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        }};
    }

    #[test]
    fn basics() {
        assert_eq!(rearrange_sticks(3, 2), 3);
        assert_eq!(rearrange_sticks(5, 5), 1);
        assert_eq!(rearrange_sticks(20, 11), 647427950);
    }

    #[test]
    fn test() {}
}
