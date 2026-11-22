mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_dominant_elements(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n <= 1 {
        return 1;
    }
    let mut res = 0;
    for (i, &num) in nums.iter().enumerate() {
        if i == 0 {
            if num >= nums[1] {
                res += 1
            }
        } else if i == n - 1 {
            if nums[n - 2] <= num {
                res += 1
            }
        } else if 2 * num >= nums[i - 1] + nums[1 + i] {
            res += 1
        }
    }
    res
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
