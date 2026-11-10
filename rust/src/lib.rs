mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn climb_stairs(n: i32, costs: Vec<i32>) -> i32 {
    let n = n as usize;
    let mut dp = vec![i64::MAX >> 2; 1 + n];
    dp[0] = 0;
    for left in 0..n {
        for d in 1..=3 {
            let right = left + d;
            if right <= n {
                dp[right] = dp[right].min(dp[left] + i64::from(costs[right - 1]) + d.pow(2) as i64);
            }
        }
    }
    dp[n] as i32
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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
