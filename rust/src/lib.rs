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

pub fn count_arrays(digit_sum: &[i32]) -> i32 {
    let mut prev_count = vec![1];
    let mut prev_nums = [0].as_slice();
    for &sum in digit_sum.iter() {
        let Some(nums) = DSUM_NUMS.get(sum as usize) else {
            return 0;
        };
        if nums.is_empty() {
            return 0;
        }
        let mut curr = Vec::with_capacity(nums.len());
        for &num in nums {
            let i = prev_nums.partition_point(|&v| v <= num);
            curr.push(if i > 0 { prev_count[i - 1] } else { 0 });
        }
        for i in 0..nums.len() - 1 {
            curr[1 + i] = (curr[1 + i] + curr[i]) % M;
        }
        prev_nums = nums;
        prev_count = curr;
    }
    *prev_count.last().unwrap_or(&0)
}

const M: i32 = 1_000_000_007;
const MAX: usize = 5_000;
static DSUM_NUMS: LazyLock<[Vec<usize>; 32]> = LazyLock::new(|| {
    let mut res = [const { vec![] }; 32];
    for num in 0..=MAX {
        let mut x = num;
        let mut d = 0;
        while x > 0 {
            d += x % 10;
            x /= 10;
        }
        res[d].push(num);
    }
    res
});

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
    fn basics() {
        assert_eq!(count_arrays(&[25, 1]), 6);
        assert_eq!(count_arrays(&[1]), 4);
        assert_eq!(count_arrays(&[2, 49, 23]), 0);
    }

    #[test]
    fn test() {}
}
