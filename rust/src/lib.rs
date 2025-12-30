mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn largest_sum_of_averages(nums: Vec<i32>, k: i32) -> f64 {
    let n = nums.len();
    let k = k as usize;
    let prefix = nums.iter().fold(vec![0.0], |mut acc, &v| {
        acc.push(f64::from(v) + acc.last().unwrap_or(&0.0));
        acc
    });
    dfs(&prefix, 0, k, &mut vec![vec![-1.0; 1 + k]; n])
}

fn dfs(prefix: &[f64], idx: usize, k: usize, memo: &mut [Vec<f64>]) -> f64 {
    let n = prefix.len() - 1;
    if idx >= n {
        return 0.0;
    }
    if k == 0 {
        return f64::MIN;
    }
    if memo[idx][k] > -1.0 {
        return memo[idx][k];
    }
    let mut res = f64::MIN;
    for i in idx..n {
        let val =
            (prefix[1 + i] - prefix[idx]) / (1 + i - idx) as f64 + dfs(prefix, 1 + i, k - 1, memo);
        res = res.max(val);
    }
    memo[idx][k] = res;
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
    fn basics() {}

    #[test]
    fn test() {}
}
