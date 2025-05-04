mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_permutation(nums: &[i32]) -> Vec<i32> {
    let n = nums.len();
    let mut vals = vec![vec![0; 1 << n]; n];
    dfs(nums, 0, 1, &mut vec![vec![-1; 1 << n]; n], &mut vals);
    let mut res = vec![0];
    let mut mask = 1;
    let mut prev = 0;
    while res.len() < n {
        let curr = vals[prev][mask];
        res.push(curr);
        mask |= 1 << curr;
        prev = curr as usize;
    }
    res
}

fn dfs(nums: &[i32], prev: i32, mask: usize, memo: &mut [Vec<i32>], vals: &mut [Vec<i32>]) -> i32 {
    let n = nums.len();
    if n == mask.count_ones() as usize {
        return (prev - nums[0]).abs();
    }
    if memo[prev as usize][mask] > -1 {
        return memo[prev as usize][mask];
    }
    memo[prev as usize][mask] = i32::MAX;
    for bit in 1..n {
        if mask & (1 << bit) == 0 {
            let curr =
                (prev - nums[bit]).abs() + dfs(nums, bit as i32, mask | (1 << bit), memo, vals);
            if curr < memo[prev as usize][mask] {
                memo[prev as usize][mask] = curr;
                vals[prev as usize][mask] = bit as i32;
            }
        }
    }
    memo[prev as usize][mask]
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
        assert_eq!(find_permutation(&[1, 0, 2]), [0, 1, 2]);
        assert_eq!(find_permutation(&[0, 2, 1]), [0, 2, 1]);
    }

    #[test]
    fn test() {}
}
