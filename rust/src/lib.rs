mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn search(nums: &[i32], target: i32) -> i32 {
    let n = nums.len();
    if n == 1 {
        return if nums[0] == target { 0 } else { -1 };
    }
    if nums[0] < nums[n - 1] {
        return nums.binary_search(&target).map(|i| i as i32).unwrap_or(-1);
    }
    let mut left = 0;
    let mut right = n - 1;
    while left < right {
        let mid = left + (right - left) / 2;
        if nums[mid] < nums[right] {
            right = mid
        } else {
            left = 1 + mid;
        }
    }
    let min = left;
    if nums[0] <= target {
        nums[..min].binary_search(&target)
    } else {
        nums[min..].binary_search(&target).map(|i| i + min)
    }
    .map(|i| i as i32)
    .unwrap_or(-1)
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
    fn basics() {
        assert_eq!(search(&[4, 5, 6, 7, 0, 1, 2], 0), 4);
    }

    #[test]
    fn test() {
        assert_eq!(search(&[1], 1), 0);
        assert_eq!(search(&[3, 1], 3), 0);
    }
}
