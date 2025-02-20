mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn first_day_been_in_all_rooms(next_visit: &[i32]) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let n = next_visit.len();
    let mut dp = vec![0i64; n];
    for idx in 1..n {
        let prev = idx - 1;
        let pvisit = next_visit[prev] as usize;
        // First time to prev
        // +1 to pvisit
        // From pvisit back to prev
        // Finally onto idx
        dp[idx] = dp[prev] + 1 + (dp[prev] - dp[pvisit]) + 1;
        dp[idx] = dp[idx].rem_euclid(MOD);
    }
    dp[n - 1] as _
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
        assert_eq!(first_day_been_in_all_rooms(&[0, 0]), 2);
        assert_eq!(first_day_been_in_all_rooms(&[0, 0, 2]), 6);
        assert_eq!(first_day_been_in_all_rooms(&[0, 1, 2, 0]), 6);
    }

    #[test]
    fn test() {}
}
