mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_matching_subarrays(nums: &[i32], pattern: &[i32]) -> i32 {
    let lps = kmp(pattern);
    let diff: Vec<_> = nums.windows(2).map(|w| (w[1] - w[0]).signum()).collect();
    let mut res = 0;
    let mut len = 0;
    for &d in &diff {
        while len > 0 && d != pattern[len] {
            len = lps[len - 1];
        }
        if d == pattern[len] {
            len += 1;
        }
        if len == pattern.len() {
            res += 1;
            len = lps[len - 1];
        }
    }
    res
}

fn kmp(s: &[i32]) -> Vec<usize> {
    let n = s.len();
    let mut lps = vec![0; n];
    let mut len = 0;
    for idx in 1..n {
        while len > 0 && s[len] != s[idx] {
            len = lps[len - 1];
        }
        if s[len] == s[idx] {
            len += 1;
        }
        lps[idx] = len;
    }
    lps
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
        assert_eq!(count_matching_subarrays(&[1, 2, 3, 4, 5, 6], &[1, 1]), 4);
        assert_eq!(
            count_matching_subarrays(&[1, 4, 4, 1, 3, 5, 5, 3], &[1, 0, -1]),
            2
        );
    }

    #[test]
    fn test() {}
}
