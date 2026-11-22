mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_cost(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let [mut even, mut odd] = [0, i64::MAX >> 2];
    for &num in &nums[1..n - 1] {
        let num = i64::from(num);
        let new_even = even.min(odd + num);
        let new_odd = odd.min(even + num);
        [even, odd] = [new_even, new_odd];
    }
    even + i64::from(nums[0]) + i64::from(nums[n - 1])
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
