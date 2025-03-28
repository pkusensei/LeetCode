mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_num_of_marked_indices(mut nums: Vec<i32>) -> i32 {
    let n = nums.len();
    nums.sort_unstable();
    let mut left = 0;
    for &v in nums[(1 + n) / 2..].iter() {
        if nums[left] * 2 <= v {
            left += 1;
        }
    }
    2 * left as i32
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
        assert_eq!(max_num_of_marked_indices(vec![3, 5, 2, 4]), 2);
        assert_eq!(max_num_of_marked_indices(vec![9, 2, 5, 4]), 4);
        assert_eq!(max_num_of_marked_indices(vec![7, 6, 8]), 0);
    }

    #[test]
    fn test() {}
}
