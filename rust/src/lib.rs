mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_alternating_sum(nums: &[i32]) -> i64 {
    let n = nums.len();
    let mut memo = vec![[[[i64::MIN >> 1; 2]; 2]; 2]; n];
    dfs(&nums, 0, 0, 0, 0, &mut memo)
}

fn dfs(
    nums: &[i32],
    idx: usize,
    parity: usize,
    deleted: usize,
    started: usize,
    memo: &mut [[[[i64; 2]; 2]; 2]],
) -> i64 {
    if idx >= nums.len() {
        return if started == 1 { 0 } else { i64::MIN >> 1 };
    }
    if memo[idx][parity][deleted][started] > i64::MIN >> 1 {
        return memo[idx][parity][deleted][started];
    }
    let sign = if parity == 0 { 1 } else { -1 };
    let val = sign * i64::from(nums[idx]);
    let res = match [deleted, started] {
        [0, 0] => {
            let v1 = dfs(nums, 1 + idx, 0, deleted, started, memo);
            let v2 = dfs(nums, 1 + idx, parity, 1, started, memo);
            let v3 = val + dfs(nums, 1 + idx, 1 - parity, deleted, 1, memo);
            v1.max(v2).max(v3)
        }
        [1, 0] => {
            let v1 = dfs(nums, 1 + idx, 0, deleted, started, memo);
            let v3 = val + dfs(nums, 1 + idx, 1 - parity, deleted, 1, memo);
            v1.max(v3)
        }
        [0, 1] => {
            let v1 = val + dfs(nums, 1 + idx, 1 - parity, deleted, started, memo);
            let v2 = dfs(nums, 1 + idx, parity, 1, started, memo);
            v1.max(v2).max(0)
        }
        [1, 1] => {
            let v1 = val + dfs(nums, 1 + idx, 1 - parity, deleted, started, memo);
            v1.max(0)
        }
        _ => unreachable!(),
    };
    memo[idx][parity][deleted][started] = res;
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
    fn test() {
        assert_eq!(max_alternating_sum(&[-41, -75]), 34)
    }
}
