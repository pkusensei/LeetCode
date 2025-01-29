mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_incompatibility(nums: &mut [i32], k: i32) -> i32 {
    let (n, k) = (nums.len(), k as usize);
    let mut groups = vec![Vec::with_capacity(n / k); k];
    nums.sort_unstable();
    let mut res = i32::MAX;
    dfs(nums, 0, 0, &mut groups, n / k, 0, &mut res);
    if res == i32::MAX {
        -1
    } else {
        res
    }
}

fn dfs(
    nums: &[i32],
    k: usize,
    idx: usize,
    groups: &mut [Vec<i32>],
    group_len: usize,
    curr: i32,
    res: &mut i32,
) {
    let n = nums.len();
    if idx >= n {
        *res = (*res).min(curr);
        return;
    }
    let val = nums[idx];
    for i in 0..groups.len().min(1 + k) {
        if groups[i].len() < group_len && groups[i].last().is_none_or(|&v| v < val) {
            let temp = val - groups[i].last().copied().unwrap_or(val) + curr;
            if curr < *res {
                groups[i].push(val);
                dfs(nums, k.max(1 + i), 1 + idx, groups, group_len, temp, res);
                groups[i].pop();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    #[test]
    fn basics() {
        assert_eq!(minimum_incompatibility(&mut [1, 2, 1, 4], 2), 4);
        assert_eq!(minimum_incompatibility(&mut [6, 3, 8, 1, 3, 1, 2, 2], 4), 6);
        assert_eq!(minimum_incompatibility(&mut [5, 3, 3, 6, 3, 3], 3), -1);
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
