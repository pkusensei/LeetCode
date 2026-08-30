mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_operations(nums: Vec<i32>, sum: i32) -> i32 {
    let n = nums.len();
    let mut memo = vec![vec![-1; 1 + sum as usize]; n];
    let v = dfs(&nums, 0, sum, &mut memo);
    if v >= i32::MAX >> 1 { -1 } else { v }
}

fn dfs(nums: &[i32], idx: usize, sum: i32, memo: &mut [Vec<i32>]) -> i32 {
    if sum == 0 {
        return 0;
    }
    if sum < 0 || idx >= nums.len() {
        return i32::MAX >> 1;
    }
    if memo[idx][sum as usize] > -1 {
        return memo[idx][sum as usize];
    }
    let mut res = dfs(nums, 1 + idx, sum, memo);
    let mut curr = nums[idx];
    res = res.min(dfs(nums, 1 + idx, sum - nums[idx], memo));
    for v in 1.. {
        curr *= 2;
        if curr > sum {
            break;
        }
        res = res.min(v + dfs(nums, 1 + idx, sum - curr, memo));
    }
    curr = nums[idx];
    for v in 1.. {
        curr /= 2;
        if curr < 1 {
            break;
        }
        res = res.min(v + dfs(nums, 1 + idx, sum - curr, memo));
    }
    memo[idx][sum as usize] = res;
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
