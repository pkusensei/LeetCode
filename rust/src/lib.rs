mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
    let [s1, s2] = [&s1, &s2].map(|s| s.as_bytes());
    let [n1, n2] = [s1, s2].map(|s| s.len());
    let mut dp = vec![vec![0; 1 + n2]; 1 + n1];
    for i1 in 0..n1 {
        for i2 in 0..n2 {
            dp[1 + i1][1 + i2] = if s1[i1] == s2[i2] {
                i32::from(s1[i1]) + dp[i1][i2]
            } else {
                dp[1 + i1][i2].max(dp[i1][1 + i2])
            };
        }
    }
    s1.iter()
        .chain(s2.iter())
        .map(|&b| i32::from(b))
        .sum::<i32>()
        - 2 * dp[n1][n2]
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
    fn basics() {}

    #[test]
    fn test() {}
}
