mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn can_split_array(nums: Vec<i32>, m: i32) -> bool {
    let n = nums.len();
    if n <= 2 {
        return true;
    }
    // let sum: i32 = nums.iter().sum();
    // dfs(&nums, m, sum, 0, n - 1, &mut vec![vec![None; n]; n])
    nums.windows(2).any(|w| w[0] + w[1] >= m)
}

fn dfs(
    nums: &[i32],
    m: i32,
    sum: i32,
    left: usize,
    right: usize,
    memo: &mut [Vec<Option<bool>>],
) -> bool {
    if left == right {
        return true;
    }
    if sum < m {
        return false;
    }
    if let Some(v) = memo[left][right] {
        return v;
    }
    let res = dfs(nums, m, sum - nums[left], left + 1, right, memo)
        || dfs(nums, m, sum - nums[right], left, right - 1, memo);
    memo[left][right] = Some(res);
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
    fn basics() {}

    #[test]
    fn test() {}
}
