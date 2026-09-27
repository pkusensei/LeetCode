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
    meetings.sort_unstable();
    let mut memo = vec![[-1; 2]; n];
    dfs(&meetings, 0, 0, &mut memo)
}

fn dfs(meets: &[Vec<i32>], idx: usize, started: usize, memo: &mut [[i64; 2]]) -> i64 {
    let n = meets.len();
    if idx >= n {
        return 0;
    }
    if memo[idx][started] > -1 {
        return memo[idx][started];
    }
    let i = meets.partition_point(|v| v[0] < meets[idx][1]);
    let take = {
        let mut v = i64::from(meets[idx][2]) + dfs(meets, i, 1, memo);
        if i < n {
            v += i64::from(meets[i][0] - meets[idx][1])
        }
        v
    };
    let skip = {
        let mut v = dfs(meets, 1 + idx, started, memo);
        if 1 + idx < n && started == 1 {
            v += i64::from(meets[1 + idx][0] - meets[idx][0])
        }
        v
    };
    let res = skip.max(take);
    memo[idx][started] = res;
    res
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
    fn basics() {
        assert_eq!(
            max_earnings(vec![vec![3, 5, 4], vec![4, 7, 8], vec![8, 10, 3]]),
            12
        );
        assert_eq!(max_earnings(vec![vec![2, 5, 4], vec![6, 8, 3]]), 8);
    }

    #[test]
    fn test() {}
}
