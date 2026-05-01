mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let sum: i32 = nums.iter().sum();
    let mut curr: i32 = nums
        .iter()
        .enumerate()
        .map(|(i, &num)| i as i32 * num)
        .sum();
    let mut res = curr;
    for i in 1..n {
        curr -= nums[n - i - 1] * n as i32;
        curr += sum;
        res = res.max(curr);
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
