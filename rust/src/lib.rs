mod dsu;
mod helper;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn longest_valid_substring(word: &str, forbidden: &[&str]) -> i32 {
    let (s, n) = (word.as_bytes(), word.len());
    let set: HashSet<_> = forbidden.iter().map(|s| s.as_bytes()).collect();
    let mut res = 0;
    let mut left = 0;
    for right in 0..n {
        let start = left.max(right.saturating_sub(10));
        for i in start..=right {
            if is_valid(&set, &s[i..=right]) {
                break;
            } else {
                left = 1 + i;
            }
        }
        res = res.max(right + 1 - left);
    }
    res as i32
}

fn is_valid(set: &HashSet<&[u8]>, s: &[u8]) -> bool {
    let n = s.len();
    for i in (0..n).rev() {
        if set.contains(&s[i..]) {
            return false;
        }
    }
    true
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
        assert_eq!(longest_valid_substring("cbaaaabc", &["aaa", "cb"]), 4);
        assert_eq!(longest_valid_substring("leetcode", &["de", "le", "e"]), 4);
    }

    #[test]
    fn test() {}
}
