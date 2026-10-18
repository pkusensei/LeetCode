mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::sync::LazyLock;

#[allow(unused_imports)]
use helper::*;

pub fn num_of_ways(nums: Vec<i32>) -> i32 {
    dfs(&nums) as i32 - 1
}

const M: i64 = 1_000_000_007;
fn dfs(nums: &[i32]) -> i64 {
    let n = nums.len();
    if n <= 2 {
        return 1;
    }
    let root = nums[0];
    let [mut small, mut big] = [vec![], vec![]];
    for &num in &nums[1..] {
        if num < root {
            small.push(num);
        } else {
            big.push(num);
        }
    }
    dfs(&small) * dfs(&big) % M * C[n - 1][small.len()] % M
}

const N: usize = 1_001;
static C: LazyLock<Vec<Vec<i64>>> = LazyLock::new(|| {
    let mut res = Vec::with_capacity(N);
    res.push(vec![1]);
    for row in 1..N {
        let mut curr = Vec::with_capacity(1 + row);
        curr.push(1);
        for col in 1..=row {
            curr.push((res[row - 1][col - 1] + res[row - 1].get(col).unwrap_or(&0)) % M);
        }
        res.push(curr);
    }
    res
});

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
