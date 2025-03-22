mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_palindromes(s: &str) -> i32 {
    let (s, n) = (s.as_bytes(), s.len());
    let mut res = 0;
    for a in b'0'..=b'9' {
        for b in b'0'..=b'9' {
            let mut memo = vec![[-1; 5]; n];
            res += dfs(s, [a, b, b'#', b, a], 0, 0, &mut memo);
            res %= MOD;
        }
    }
    res
}

const MOD: i32 = 1_000_000_007;

fn dfs(s: &[u8], pattern: [u8; 5], si: usize, pi: usize, memo: &mut [[i32; 5]]) -> i32 {
    if pi == 5 || si >= s.len() {
        return i32::from(pi == 5);
    }
    if memo[si][pi] > -1 {
        return memo[si][pi];
    }
    let mut res = dfs(s, pattern, 1 + si, pi, memo);
    if s[si] == pattern[pi] || pattern[pi] == b'#' {
        res += dfs(s, pattern, 1 + si, 1 + pi, memo);
        res %= MOD;
    }
    memo[si][pi] = res;
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
        assert_eq!(count_palindromes("103301"), 2);
        assert_eq!(count_palindromes("0000000"), 21);
        assert_eq!(count_palindromes("9999900000"), 2);
    }

    #[test]
    fn test() {}
}
