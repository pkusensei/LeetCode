mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn elevator_requests(_n: i32, start: i32, mut requests: Vec<i32>) -> i64 {
    if !requests.contains(&start) {
        requests.push(start);
    }
    requests.sort_unstable();
    let n = requests.len();
    let i = requests.binary_search(&start).unwrap();
    let mut dp = vec![vec![[i64::MAX >> 2; 2]; n]; n];
    dp[i][i].fill(0);
    for len in 1..=n {
        for left in 0..n {
            let right = left + len - 1;
            if right >= n {
                break;
            }
            let rem = (n - len) as i64;
            if left > 0 {
                let a = i64::from(requests[left] - requests[left - 1]) * rem + dp[left][right][0];
                let b = i64::from(requests[right] - requests[left - 1]) * rem + dp[left][right][1];
                dp[left - 1][right][0] = dp[left - 1][right][0].min(a.min(b));
            }
            if 1 + right < n {
                let a = i64::from(requests[1 + right] - requests[right]) * rem + dp[left][right][1];
                let b = i64::from(requests[1 + right] - requests[left]) * rem + dp[left][right][0];
                dp[left][1 + right][1] = dp[left][1 + right][1].min(a.min(b));
            }
        }
    }
    dp[0][n - 1][0].min(dp[0][n - 1][1])
    // let mut memo = vec![vec![[-1; 2]; n]; n];
    // dfs(&requests, i, i, 0, &mut memo).min(dfs(&requests, i, i, 1, &mut memo))
}

fn dfs(reqs: &[i32], left: usize, right: usize, dir: usize, memo: &mut [Vec<[i64; 2]>]) -> i64 {
    let n = reqs.len();
    if left == 0 && right >= n - 1 {
        return 0;
    }
    if memo[left][right][dir] > -1 {
        return memo[left][right][dir];
    }
    // n - (1+right-left)
    let rem = (left + n - right - 1) as i64;
    let mut res = i64::MAX >> 2;
    if left > 0 {
        let curr = if dir == 0 {
            i64::from(reqs[left] - reqs[left - 1]) * rem + dfs(reqs, left - 1, right, 0, memo)
        } else {
            i64::from(reqs[right] - reqs[left - 1]) * rem + dfs(reqs, left - 1, right, 0, memo)
        };
        res = res.min(curr);
    }
    if 1 + right < n {
        let curr = if dir == 1 {
            i64::from(reqs[1 + right] - reqs[right]) * rem + dfs(reqs, left, 1 + right, 1, memo)
        } else {
            i64::from(reqs[1 + right] - reqs[left]) * rem + dfs(reqs, left, 1 + right, 1, memo)
        };
        res = res.min(curr);
    }
    memo[left][right][dir] = res;
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
    fn basics() {}

    #[test]
    fn test() {}
}
