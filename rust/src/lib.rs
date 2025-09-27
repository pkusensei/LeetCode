mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn climb_stairs(n: i32, costs: &[i32]) -> i32 {
    let n = n as usize;
    let mut dp = vec![i32::MAX; 1 + n];
    dp[0] = 0;
    for left in 0..n {
        for step in 1..=3 {
            let right = left + step;
            if right > n {
                break;
            }
            dp[right] = dp[right].min(dp[left] + costs[right - 1] + step.pow(2) as i32);
        }
    }
    dp[n]
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
        assert_eq!(climb_stairs(4, &[1, 2, 3, 4]), 13);
        assert_eq!(climb_stairs(4, &[5, 1, 6, 2]), 11);
    }

    #[test]
    fn test() {}
}
