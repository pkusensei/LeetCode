mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_subarray_xor(nums: &[i32], queries: &[[i32; 2]]) -> Vec<i32> {
    let n = nums.len();
    let mut memo = vec![vec![None; n]; n];
    queries
        .iter()
        .map(|q| dfs(nums, q[0] as usize, q[1] as usize, &mut memo)[1])
        .collect()
}

fn dfs(nums: &[i32], left: usize, right: usize, memo: &mut [Vec<Option<[i32; 2]>>]) -> [i32; 2] {
    if left == right {
        return [nums[left]; 2];
    }
    if let Some(v) = memo[left][right] {
        return v;
    }
    let [xor1, max1] = dfs(nums, left, right - 1, memo);
    let [xor2, max2] = dfs(nums, 1 + left, right, memo);
    let res = [xor1 ^ xor2, (xor1 ^ xor2).max(max1).max(max2)];
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
    fn basics() {
        assert_eq!(
            maximum_subarray_xor(&[2, 8, 4, 32, 16, 1], &[[0, 2], [1, 4], [0, 5]]),
            [12, 60, 60]
        );
        assert_eq!(
            maximum_subarray_xor(
                &[0, 7, 3, 2, 8, 5, 1],
                &[[0, 3], [1, 5], [2, 4], [2, 6], [5, 6]]
            ),
            [7, 14, 11, 14, 5]
        );
    }

    #[test]
    fn test() {}
}
