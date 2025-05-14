mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_of_pairs(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    dfs(&nums, 0, 0, i32::MAX, &mut vec![[-1; 51]; n])
}

fn dfs(nums: &[i32], idx: usize, prev1: i32, prev2: i32, memo: &mut [[i32; 51]]) -> i32 {
    if idx >= nums.len() {
        return 1;
    }
    if memo[idx][prev1 as usize] > -1 {
        return memo[idx][prev1 as usize];
    }
    let mut res = 0;
    for val1 in prev1..=nums[idx] {
        let val2 = nums[idx] - val1;
        if val2 <= prev2 {
            res += dfs(nums, 1 + idx, val1, val2, memo);
            res %= 1_000_000_007;
        }
    }
    memo[idx][prev1 as usize] = res;
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

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
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
        assert_eq!(count_of_pairs(vec![2, 3, 2]), 4);
        assert_eq!(count_of_pairs(vec![5, 5, 5, 5]), 126);
    }

    #[test]
    fn test() {}
}
