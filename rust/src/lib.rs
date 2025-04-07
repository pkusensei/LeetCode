mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_ways(n: i32, x: i32) -> i32 {
    let n = n as usize;
    let mut dp = vec![vec![-1; 1 + n]; 1 + n];
    dfs(n, x as _, 1, &mut dp)
}

fn dfs(n: usize, x: u32, start: usize, memo: &mut [Vec<i32>]) -> i32 {
    if n == 0 {
        return 1;
    }
    if memo[n][start] > -1 {
        return memo[n][start];
    }
    let mut res = 0;
    let mut base = start;
    while base.pow(x) <= n {
        res += dfs(n - base.pow(x), x, 1 + base, memo);
        res %= 1_000_000_007;
        base += 1;
    }
    memo[n][start] = res;
    res
}

pub fn tabulation(n: i32, x: i32) -> i32 {
    let n = n as usize;
    let x = x as u32;
    let mut base = 0;
    let mut v = 1_usize;
    while v.pow(x) <= n {
        base = v;
        v += 1;
    }
    let mut dp = vec![0; 1 + n];
    dp[0] = 1;
    for i1 in 1..=base {
        let p = i1.pow(x);
        for i2 in (p..=n).rev() {
            dp[i2] += dp[i2 - p];
            dp[i2] %= 1_000_000_007;
        }
    }
    dp[n]
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
        assert_eq!(number_of_ways(10, 2), 1);
        assert_eq!(number_of_ways(4, 1), 2);

        assert_eq!(tabulation(10, 2), 1);
        assert_eq!(tabulation(4, 1), 2);
    }

    #[test]
    fn test() {}
}
