mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_valid_strings(words: &[&str], target: &str) -> i32 {
    let (s, n) = (target.as_bytes(), target.len());
    let mut vals = vec![0; n];
    for pat in words.iter() {
        for (idx, len) in kmp(pat.as_bytes(), s).into_iter().enumerate() {
            vals[idx] = vals[idx].max(len);
        }
    }
    let mut dp = vec![i32::MAX / 2; 1 + n];
    dp[n] = 0;
    for right in (0..n).rev() {
        let len = vals[right];
        dp[right + 1 - len] = dp[right + 1 - len].min(1 + dp[1 + right]);
    }
    if dp[0] >= i32::MAX / 2 { -1 } else { dp[0] }
}

fn kmp(pat: &[u8], target: &[u8]) -> Vec<usize> {
    let n = pat.len();
    let mut lps = vec![0];
    let mut len = 0;
    for idx in 1..n {
        while len > 0 && pat[len] != pat[idx] {
            len = lps[len - 1];
        }
        if pat[len] == pat[idx] {
            len += 1;
        }
        lps.push(len);
    }
    len = 0;
    let mut res = vec![];
    for &b in target {
        while len > 0 && (len == pat.len() || pat[len] != b) {
            len = lps[len - 1];
        }
        if pat[len] == b {
            len += 1;
        }
        res.push(len);
    }
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
        assert_eq!(min_valid_strings(&["abc", "aaaaa", "bcdef"], "aabcdabc"), 3);
        assert_eq!(min_valid_strings(&["abababab", "ab"], "ababaababa"), 2);
        assert_eq!(min_valid_strings(&["abcdef"], "xyz"), -1);
    }

    #[test]
    fn test() {}
}
