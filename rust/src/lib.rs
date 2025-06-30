mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_operations(word1: &str, word2: &str) -> i32 {
    let (s1, s2) = (word1.as_bytes(), word2.as_bytes());
    let n = word1.len();
    let mut dp = vec![i32::MAX; 1 + n];
    dp[0] = 0;
    for right in 1..=n {
        for left in 0..right {
            let curr = solve(
                s1[left..right].iter().copied(),
                s2[left..right].iter().copied(),
            )
            .min(
                1 + solve(
                    s1[left..right].iter().rev().copied(),
                    s2[left..right].iter().copied(),
                ),
            );
            dp[right] = dp[right].min(dp[left] + curr);
        }
    }
    dp[n]
}

fn solve(s1: impl Iterator<Item = u8>, s2: impl Iterator<Item = u8>) -> i32 {
    let mut res = 0;
    let mut freq = std::collections::HashMap::new();
    for (a, b) in s1.zip(s2) {
        if a == b {
            continue;
        }
        if freq.get(&[b, a]).is_some_and(|&v| v > 0) {
            *freq.entry([b, a]).or_insert(0) -= 1; // offset swap
        } else {
            *freq.entry([a, b]).or_insert(0) += 1;
            res += 1; // swap or replace
        }
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

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
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
        assert_eq!(min_operations("abcdf", "dacbe"), 4);
        assert_eq!(min_operations("abceded", "baecfef"), 4);
        assert_eq!(min_operations("abcdef", "fedabc"), 2);
    }

    #[test]
    fn test() {
        assert_eq!(min_operations("abcebc", "ecbade"), 3);
    }
}
