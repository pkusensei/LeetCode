mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_swaps(mut nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let ones: i32 = nums.iter().sum();
    let window_len = ones as usize;
    nums.extend_from_within(..);
    let mut zeros = ones - nums[..window_len].iter().sum::<i32>();
    let mut res = zeros;
    for idx in 1..(2 * n - window_len) {
        zeros -= i32::from(nums[idx - 1] == 0);
        zeros += i32::from(nums[idx + window_len - 1] == 0);
        res = res.min(zeros);
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
