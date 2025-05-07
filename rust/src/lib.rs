mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_total_damage(mut power: Vec<i32>) -> i64 {
    power.sort_unstable();
    let mut dp = [0; 3];
    dp[0] = i64::from(power[0]);
    for (idx, &num) in power.iter().enumerate().skip(1) {
        let mut curr = [0; 3];
        match num - power[idx - 1] {
            0 => {
                curr = dp;
            }
            1 => {
                // 1 2 3 +4
                curr[0] = dp[2];
                curr[1] = dp[0];
                curr[2] = dp[1].max(dp[2]);
            }
            2 => {
                // 1 2 3 +5
                curr[1..].fill(dp.into_iter().max().unwrap());
                curr[0] = dp[1].max(dp[2]);
            }
            _ => {
                // 1 2 3 +6
                curr.fill(dp.into_iter().max().unwrap());
            }
        }
        curr[0] += i64::from(num);
        dp = curr;
    }
    dp.into_iter().max().unwrap()
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
        assert_eq!(maximum_total_damage(vec![1, 1, 3, 4]), 6);
        assert_eq!(maximum_total_damage(vec![7, 1, 6, 6]), 13);
    }

    #[test]
    fn test() {
        assert_eq!(maximum_total_damage(vec![7, 1, 6, 3]), 10);
    }
}
