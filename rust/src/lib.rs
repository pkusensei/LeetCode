mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn num_squareful_perms(mut nums: Vec<i32>) -> i32 {
    let n = nums.len();
    nums.sort_unstable();
    backtrack(&nums, 0, None)
}

fn backtrack(nums: &[i32], mask: i32, prev: Option<i32>) -> i32 {
    let n = nums.len();
    if mask == (1 << n) - 1 {
        return 1;
    }
    let mut res = 0;
    for i in 0..n {
        if (mask >> i) & 1 == 1 {
            continue;
        }
        if i > 0 && nums[i] == nums[i - 1] && (mask >> (i - 1)) & 1 == 1 {
            continue;
        }
        if let Some(p) = prev {
            let v = i64::from(p) + i64::from(nums[i]);
            if v.isqrt().pow(2) == v {
                res += backtrack(nums, mask | (1 << i), Some(nums[i]));
            }
        } else {
            res += backtrack(nums, 1 << i, Some(nums[i]));
        }
    }
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
