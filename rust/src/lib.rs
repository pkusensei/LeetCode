mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_earnings(mut meetings: Vec<Vec<i32>>) -> i64 {
    let n = meetings.len();
    meetings.sort_unstable_by_key(|v| v[0]);
    let mut dp = vec![[0; 2]; 1 + n];
    for (idx, curr) in meetings.iter().enumerate().rev() {
        let [start, end, val] = curr[..] else {
            unreachable!()
        };
        let i = meetings.partition_point(|v| v[0] < end);
        for started in [0, 1] {
            // dp[next] + curr_val + gap
            let take = dp[i][1]
                + i64::from(val)
                + meetings.get(i).map(|v| i64::from(v[0] - end)).unwrap_or(0);
            let mut skip = dp[1 + idx][started];
            if 1 + idx < n && started == 1 {
                skip += i64::from(meetings[1 + idx][0] - start);
            }
            dp[idx][started] = dp[idx][started].max(take).max(skip);
        }
    }
    dp[0][0]
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
