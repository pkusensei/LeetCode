mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_cost(nums: &[i32]) -> i32 {
    let n = nums.len();
    dfs(&nums, 1, 0, &mut vec![vec![-1; n]; n])
}

fn dfs(nums: &[i32], idx: usize, prev: usize, memo: &mut [Vec<i32>]) -> i32 {
    let n = nums.len();
    if idx >= n {
        return nums[prev];
    }
    if idx == n - 1 {
        return nums[prev].max(nums[idx]);
    }
    if memo[idx][prev] > -1 {
        return memo[idx][prev];
    }
    let a = nums[prev].max(nums[idx]) + dfs(nums, 2 + idx, 1 + idx, memo);
    let b = nums[prev].max(nums[1 + idx]) + dfs(nums, 2 + idx, idx, memo);
    let c = nums[idx].max(nums[1 + idx]) + dfs(nums, 2 + idx, prev, memo);
    memo[idx][prev] = a.min(b).min(c);
    memo[idx][prev]
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
        assert_eq!(min_cost(&[6, 2, 8, 4]), 12);
        assert_eq!(min_cost(&[2, 1, 3, 3]), 5);
    }

    #[test]
    fn test() {}
}
