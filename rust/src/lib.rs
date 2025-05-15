mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_energy_boost(energy_drink_a: Vec<i32>, energy_drink_b: Vec<i32>) -> i64 {
    let n = energy_drink_a.len();
    let mut dp = vec![[0, 0]; n];
    dp[0] = [energy_drink_a[0], energy_drink_b[0]].map(i64::from);
    dp[1][0] = dp[0][0] + i64::from(energy_drink_a[1]);
    dp[1][1] = dp[0][1] + i64::from(energy_drink_b[1]);
    for (i, (&a, &b)) in energy_drink_a
        .iter()
        .zip(energy_drink_b.iter())
        .enumerate()
        .skip(2)
    {
        dp[i][0] = dp[i - 1][0].max(dp[i - 2][1]) + i64::from(a);
        dp[i][1] = dp[i - 1][1].max(dp[i - 2][0]) + i64::from(b);
    }
    dp[n - 1][0].max(dp[n - 1][1])
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
    fn basics() {}

    #[test]
    fn test() {}
}
