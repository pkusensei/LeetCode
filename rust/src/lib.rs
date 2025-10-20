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
    let n1 = s1.len();
    let n2 = s2.len();
    let mut dp = vec![vec![0; 1 + n2]; 1 + n1];
    for (i1, b1) in s1.bytes().enumerate() {
        for (i2, b2) in s2.bytes().enumerate() {
            dp[1 + i1][1 + i2] = if b1 == b2 {
                b1 as i32 + dp[i1][i2]
            } else {
                dp[1 + i1][i2].max(dp[i1][1 + i2])
            };
        }
    }
    s1.bytes().chain(s2.bytes()).map(|b| b as i32).sum::<i32>() - 2 * dp[n1][n2]
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
