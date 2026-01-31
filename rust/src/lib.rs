mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_partition_score(nums: &[i32], k: i32) -> i64 {
    let n = nums.len();
    let k = k as usize;
    dfs(&nums, 0, k, &mut vec![vec![-1; 1 + k]; n])
}

fn dfs(nums: &[i32], idx: usize, k: usize, memo: &mut [Vec<i64>]) -> i64 {
    let n = nums.len();
    if k == 0 {
        return if idx >= n { 0 } else { i64::MAX >> 1 };
    }
    if idx >= n {
        return i64::MAX >> 1;
    }
    if memo[idx][k] > -1 {
        return memo[idx][k];
    }
    let mut res = i64::MAX >> 1;
    let mut sum = 0;
    for i in idx..n {
        if n - i < k {
            // not enough numbers
            break;
        }
        sum += i64::from(nums[i]);
        let curr = sum * (1 + sum) / 2;
        if curr >= res {
            break; // future recursion is useless
        }
        res = res.min(curr + dfs(nums, 1 + i, k - 1, memo));
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
    fn basics() {
        assert_eq!(min_partition_score(&[1], 1), 1);
        assert_eq!(min_partition_score(&[5, 1, 2, 1], 2), 25);
    }

    #[test]
    fn test() {}
}
