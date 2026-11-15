mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_difference(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    dfs(&nums, &mut vec![None; 1 + n])
}

fn dfs(nums: &[i32], memo: &mut [Option<i64>]) -> i64 {
    let n = nums.len();
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return i64::from(nums[0]);
    }
    if let Some(v) = memo[n] {
        return v;
    }
    let mut sum = i64::from(nums[0]);
    let a = if sum % 3 == 0 {
        sum - dfs(&nums[1..], memo)
    } else {
        sum + dfs(&nums[1..], memo)
    };
    sum += i64::from(nums[1]);
    let b = if sum % 3 == 0 {
        sum - dfs(&nums[2..], memo)
    } else {
        sum + dfs(&nums[2..], memo)
    };
    let res = a.max(b);
    memo[n] = Some(res);
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
