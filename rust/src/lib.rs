mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_stable_subsequences(nums: &[i32]) -> i32 {
    let n = nums.len();
    dfs(nums, 0, 2, 2, &mut vec![[[-1; 3]; 3]; n]) - 1
}

const M: i32 = 1_000_000_007;

fn dfs(nums: &[i32], idx: usize, prev1: usize, prev2: usize, memo: &mut [[[i32; 3]; 3]]) -> i32 {
    if idx >= nums.len() {
        return 1;
    }
    if memo[idx][prev1][prev2] > -1 {
        return memo[idx][prev1][prev2];
    }
    // skip
    let mut res = dfs(nums, 1 + idx, prev1, prev2, memo);
    let parity = (nums[idx] & 1) as usize;
    if prev1 == prev2 {
        if prev1 == 2 || prev2 != parity {
            res += dfs(nums, 1 + idx, prev2, parity, memo);
        }
    } else {
        res += dfs(nums, 1 + idx, prev2, parity, memo);
    }
    res %= M;
    memo[idx][prev1][prev2] = res;
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
