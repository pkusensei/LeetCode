mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn incremovable_subarray_count(nums: &[i32]) -> i64 {
    let n = nums.len();
    let x = nums.windows(2).take_while(|w| w[0] < w[1]).count();
    let y = (0..n - 1)
        .rev()
        .take_while(|&i| nums[i] < nums[1 + i])
        .last()
        .unwrap_or(n - 1);
    if y == 0 {
        return ((1 + n) * n / 2) as i64; // sorted
    }
    let mut res = n - y + 1; // Everything to its left could be removed
    let mut right = y;
    for left in 0..=x {
        while nums.get(right).is_some_and(|&v| v <= nums[left]) {
            right += 1;
        }
        res += n - right + 1;
    }
    res as i64
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
        assert_eq!(incremovable_subarray_count(&[6, 5, 7, 8]), 7);
        assert_eq!(incremovable_subarray_count(&[1, 2, 3, 4]), 10);
        assert_eq!(incremovable_subarray_count(&[8, 7, 6, 6]), 3);
    }

    #[test]
    fn test() {}
}
