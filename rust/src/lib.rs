mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn delete_string(s: &str) -> i32 {
    let (s, n) = (s.as_bytes(), s.len());
    let mut dp = vec![1; n];
    let mut lcs = vec![vec![0; 1 + n]; 1 + n];
    for left in (0..n).rev() {
        dp[left] = 1;
        for right in 1 + left..n {
            if s[left] == s[right] {
                lcs[left][right] = 1 + lcs[1 + left][1 + right];
            }
            if lcs[left][right] >= (right - left) as i32 {
                dp[left] = dp[left].max(1 + dp[right]);
            }
        }
    }
    dp[0]
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
        assert_eq!(delete_string("abcabcdabc"), 2);
        assert_eq!(delete_string("aaabaab"), 4);
        assert_eq!(delete_string("aaaaa"), 5);
    }

    #[test]
    fn test() {}
}
