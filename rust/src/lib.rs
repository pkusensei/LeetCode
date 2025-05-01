mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_substrings_in_partition(s: &str) -> i32 {
    let (s, n) = (s.as_bytes(), s.len());
    let mut dp = vec![n as i32; n];
    for right in 0..n {
        let mut freq = [0; 26];
        for left in (0..=right).rev() {
            freq[usize::from(s[left] - b'a')] += 1;
            if check(&freq) {
                if left > 0 {
                    dp[right] = dp[right].min(1 + dp[left - 1]);
                } else {
                    dp[right] = 1;
                }
            }
        }
    }
    dp[n - 1]
}

fn check(freq: &[i32; 26]) -> bool {
    let mut min = i32::MAX;
    let mut max = 0;
    for &f in freq.iter().filter(|&&v| v > 0) {
        min = min.min(f);
        max = max.max(f);
    }
    min == max
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
        assert_eq!(minimum_substrings_in_partition("fabccddg"), 3);
        assert_eq!(minimum_substrings_in_partition("abababaccddb"), 2);
    }

    #[test]
    fn test() {}
}
