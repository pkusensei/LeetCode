mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let n = nums.len();
    let mut res = 0;
    for big in (2..n).rev() {
        let mut left = 0;
        let mut right = big - 1;
        while left < right {
            if nums[left] + nums[right] > nums[big] {
                res += (right - left) as i32;
                right -= 1;
            } else {
                left += 1;
            }
        }
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
