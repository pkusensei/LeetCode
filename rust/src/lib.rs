mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_sum_subarray(nums: Vec<i32>, l: i32, r: i32) -> i32 {
    let n = nums.len();
    let [low, high] = [l, r].map(|v| v as usize);
    let mut res = i32::MAX;
    for len in low..=high {
        let mut sum = nums[..len].iter().sum();
        if sum > 0 {
            res = res.min(sum)
        }
        let [mut left, mut right] = [0, len];
        while right < n {
            sum += nums[right];
            sum -= nums[left];
            left += 1;
            right += 1;
            if sum > 0 {
                res = res.min(sum)
            }
        }
    }
    if res < i32::MAX { res } else { -1 }
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
