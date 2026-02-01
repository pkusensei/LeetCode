mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn final_element(nums: &[i32]) -> i32 {
    let n = nums.len();
    nums[0].max(nums[n - 1])
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
        assert_eq!(final_element(&[4, 16, 3, 9]), 9);
        assert_eq!(final_element(&[1, 5, 2]), 2);
        assert_eq!(final_element(&[3, 7]), 7);
        assert_eq!(final_element(&[1, 17, 12, 9]), 9);
    }

    #[test]
    fn test() {
        assert_eq!(final_element(&[22, 26, 16, 3, 20]), 22);
        assert_eq!(final_element(&[22, 6, 24, 15, 4]), 22);
    }
}
