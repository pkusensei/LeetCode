mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn check(nums: &[i32]) -> bool {
    let n = nums.len();
    let mut right = n - 1;
    while right > 0 && nums[right] == nums[0] {
        right -= 1;
    }
    if right == 0 {
        return true;
    }
    if nums[0] <= nums[right] {
        return nums[..=right].is_sorted();
    }
    let mut seen_dec = false;
    for w in nums[..=right].windows(2) {
        if w[0] > w[1] {
            if seen_dec {
                return false;
            }
            seen_dec = true
        }
    }
    true
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
        assert!(!check(&[5, 1, 5, 1]));
    }

    #[test]
    fn test() {}
}
