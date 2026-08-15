mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn elevator_requests(n: i32, start: i32, mut requests: Vec<i32>) -> i64 {
    let n = requests.len();
    requests.sort_unstable();
    let mut memo = vec![vec![[-1; 3]; 1 + n]; 1 + n];
    let i = requests.partition_point(|&v| v < start);
    dfs(&requests, start, i, i, 0, &mut memo)
}

fn dfs(
    reqs: &[i32],
    start: i32,
    left: usize,
    right: usize,
    dir: usize,
    memo: &mut [Vec<[i64; 3]>],
) -> i64 {
    let n = reqs.len();
    if left == 0 && right >= n {
        return 0;
    }
    if memo[left][right][dir] > -1 {
        return memo[left][right][dir];
    }
    let rem = (left + n - right) as i64;
    let [mut a, mut b] = [i64::MAX >> 2; 2];
    if left > 0 {
        if dir == 0 {
            a = i64::from(start - reqs[left - 1]).abs() * rem
                + dfs(reqs, start, left - 1, right, 1, memo);
        } else if dir == 1 {
            a = i64::from(reqs[left] - reqs[left - 1]).abs() * rem
                + dfs(reqs, start, left - 1, right, 1, memo);
        } else {
            a = i64::from(reqs[right - 1] - reqs[left - 1]).abs() * rem
                + dfs(reqs, start, left - 1, right, 1, memo)
        }
    }
    if right < n {
        if dir == 0 {
            b = i64::from(start - reqs[right]).abs() * rem
                + dfs(reqs, start, left, 1 + right, 2, memo);
        } else if dir == 2 {
            b = i64::from(reqs[right] - reqs[right - 1]) * rem
                + dfs(reqs, start, left, 1 + right, 2, memo);
        } else {
            b = i64::from(reqs[left] - reqs[right]).abs() * rem
                + dfs(reqs, start, left, 1 + right, 2, memo);
        }
    }
    memo[left][right][dir] = a.min(b);
    memo[left][right][dir]
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
