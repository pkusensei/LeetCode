mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn special_perm(nums: &[i32]) -> i32 {
        let n = nums.len();
        dfs(nums, 0, n, &mut HashMap::new())
}

fn dfs(nums: &[i32], mask: i16, prev: usize, memo: &mut HashMap<(i16, usize), i32>) -> i32 {
    let n = nums.len();
    if n == mask.count_ones() as usize {
        return 1;
    }
    if let Some(&v) = memo.get(&(mask, prev)) {
        return v;
    }
    let mut res = 0;
    for bit in 0..n {
        if (mask >> bit) & 1 == 1 {
            continue;
        }
        let curr = nums[bit];
        if nums
            .get(prev)
            .is_none_or(|&v| v % curr == 0 || curr % v == 0)
        {
            res += dfs(nums, mask | (1 << bit), bit, memo);
            res %= 1_000_000_007;
        }
    }
    memo.insert((mask, prev), res);
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
        assert_eq!(special_perm(&[2, 3, 6]), 2);
        assert_eq!(special_perm(&[1, 4, 3]), 2);
    }

    #[test]
    fn test() {}
}
