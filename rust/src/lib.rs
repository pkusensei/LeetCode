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
    let mut left = 0;
    let mut right = n - 1;
    while left < right {
        let mid = left + (1 + right - left) / 2;
        if nums[mid] == target {
            return mid as i32;
        } else if nums[left] <= nums[mid] {
            // `mid` falls in first half
            if (nums[left]..nums[mid]).contains(&target) {
                // `target` falls here
                right = mid - 1
            } else {
                // `target` is to the right
                left = mid
            }
        } else {
            if (nums[mid]..=nums[right]).contains(&target) {
                left = mid
            } else {
                right = mid - 1
            }
        }
    }
    if nums[left] == target {
        left as i32
    } else {
        -1
    }
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
