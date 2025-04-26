mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_strength(nums: &[i32], k: i32) -> i64 {
    let n = nums.len();
    let mut memo = vec![vec![[i64::MIN; 2]; 1 + k as usize]; n];
    dfs(&nums, k.into(), 0, 1, &mut memo)
}

fn dfs(nums: &[i32], k: i64, idx: usize, fresh: usize, memo: &mut [Vec<[i64; 2]>]) -> i64 {
    let n = nums.len();
    if k == 0 {
        return 0;
    }
    if idx >= n || n - idx < k as usize {
        return i64::MIN / 2;
    }
    if memo[idx][k as usize][fresh] > i64::MIN {
        return memo[idx][k as usize][fresh];
    }
    let skip = if fresh == 1 {
        dfs(nums, k, 1 + idx, 1, memo)
    } else {
        i64::MIN / 2
    };
    let curr = i64::from(nums[idx]) * if k & 1 == 1 { k } else { -k };
    let extend_old = curr + dfs(nums, k, 1 + idx, 0, memo);
    let start_new = curr + dfs(nums, k - 1, 1 + idx, 1, memo);
    let res = skip.max(extend_old).max(start_new);
    memo[idx][k as usize][fresh] = res;
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
        assert_eq!(maximum_strength(&[1, 2, 3, -1, 2], 3), 22);
        assert_eq!(maximum_strength(&[12, -2, -2, -2, -2], 5), 64);
        assert_eq!(maximum_strength(&[-1, -2, -3], 1), -1);
    }

    #[test]
    fn test() {
        assert_eq!(maximum_strength(&[-99, 85], 1), 85);
    }
}
