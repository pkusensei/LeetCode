mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn subsequence_pair_count(nums: &[i32]) -> i32 {
    dfs(nums, 0, 0, 0, &mut HashMap::new())
}

fn dfs(
    nums: &[i32],
    idx: usize,
    gcd1: i32,
    gcd2: i32,
    memo: &mut HashMap<(usize, i32, i32), i32>,
) -> i32 {
    const M: i32 = 1_000_000_007;
    if idx >= nums.len() {
        return i32::from(gcd1 > 0 && gcd1 == gcd2);
    }
    let k = (idx, gcd1, gcd2);
    if let Some(&v) = memo.get(&k) {
        return v;
    }
    let val = nums[idx];
    let mut res = (dfs(nums, 1 + idx, gcd1, gcd2, memo)
        + dfs(
            nums,
            1 + idx,
            if gcd1 > 0 { gcd(gcd1, val) } else { val },
            gcd2,
            memo,
        ))
        % M
        + dfs(
            nums,
            1 + idx,
            gcd1,
            if gcd2 > 0 { gcd(gcd2, val) } else { val },
            memo,
        );
    res %= M;
    memo.insert(k, res);
    res
}

const fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 { b } else { gcd(b % a, a) }
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
        assert_eq!(subsequence_pair_count(&[1, 2, 3, 4]), 10);
        assert_eq!(subsequence_pair_count(&[10, 20, 30]), 2);
    }

    #[test]
    fn test() {}
}
