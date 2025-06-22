mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_travel_time(_l: i32, n: i32, k: i32, position: &[i32], time: &[i32]) -> i32 {
    let [n, k] = [n, k].map(|v| v as usize);
    let prefix = time.iter().fold(Vec::with_capacity(n), |mut acc, &v| {
        acc.push(v + acc.last().unwrap_or(&0));
        acc
    });
    dfs(
        position,
        &prefix,
        0,
        k,
        0,
        &mut vec![vec![vec![-1; 1 + n]; 1 + k]; n],
    )
}

fn dfs(
    position: &[i32],
    prefix: &[i32],
    idx: usize,
    k: usize,
    last: usize,
    memo: &mut [Vec<Vec<i32>>],
) -> i32 {
    let n = position.len();
    if idx == position.len() - 1 {
        return if k == 0 { 0 } else { i32::MAX >> 1 };
    }
    if memo[idx][k][last] > -1 {
        return memo[idx][k][last];
    }
    let time = prefix[idx] - if last > 0 { prefix[last - 1] } else { 0 };
    let end = (n - 1).min(idx + k + 1);
    let mut res = i32::MAX;
    for next in 1 + idx..=end {
        let dist = position[next] - position[idx];
        let curr = dist * time + dfs(position, prefix, next, k - (next - idx - 1), 1 + idx, memo);
        res = res.min(curr);
    }
    memo[idx][k][last] = res;
    res
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
        assert_eq!(min_travel_time(10, 4, 1, &[0, 3, 8, 10], &[5, 8, 3, 6]), 62);
        assert_eq!(
            min_travel_time(5, 5, 1, &[0, 1, 2, 3, 5], &[8, 3, 9, 3, 3]),
            34
        );
    }

    #[test]
    fn test() {}
}
