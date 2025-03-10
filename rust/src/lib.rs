mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_texts(pressed_keys: &str) -> i32 {
    let (s, n) = (pressed_keys.as_bytes(), pressed_keys.len());
    // dfs(s, 0, &mut vec![-1; n])
    let mut dp = vec![0; 1 + n];
    dp[n] = 1;
    for idx in (0..n).rev() {
        let len = if matches!(s[idx], b'7' | b'9') { 4 } else { 3 };
        for i in idx..(idx + len).min(n) {
            if s[i] != s[idx] {
                break;
            }
            dp[idx] += dp[1 + i];
            dp[idx] %= MOD;
        }
    }
    dp[0]
}

const MOD: i32 = 1_000_000_007;

fn dfs(s: &[u8], idx: usize, memo: &mut [i32]) -> i32 {
    let n = s.len();
    if idx >= n {
        return 1;
    }
    if memo[idx] > -1 {
        return memo[idx];
    }
    let mut res = 0;
    let len = if matches!(s[idx], b'7' | b'9') { 4 } else { 3 };
    for i in idx..(idx + len).min(n) {
        if s[i] != s[idx] {
            break;
        }
        res += dfs(s, 1 + i, memo);
        res %= MOD;
    }
    memo[idx] = res;
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
        assert_eq!(count_texts("22233"), 8);
        assert_eq!(
            count_texts("222222222222222222222222222222222222"),
            82876089
        );
    }

    #[test]
    fn test() {}
}
