mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn can_reach(s: &str, min_jump: i32, max_jump: i32) -> bool {
    let (s, n) = (s.as_bytes(), s.len());
    let [minj, maxj] = [min_jump, max_jump].map(|v| v as usize);
    let mut dp = vec![false; n];
    dp[0] = true;
    let mut count = 0;
    // sliding window [i-maxj..i-minj]
    for idx in minj..n {
        if dp[idx - minj] {
            count += 1;
        }
        if idx > maxj && dp[idx - maxj - 1] {
            count -= 1;
        }
        dp[idx] = count > 0 && s[idx] == b'0';
    }
    dp[n - 1]
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
        assert!(can_reach("011010", 2, 3));
        assert!(!can_reach("01101110", 2, 3));
    }

    #[test]
    fn test() {}
}
