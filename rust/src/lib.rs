mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn bowl_subarrays(nums: &[i32]) -> i64 {
    let mut stack = vec![]; // mono dec stack
    let mut res = 0;
    for (right, &num) in nums.iter().enumerate() {
        // [left_end] <= [right_end]
        while let Some(&top) = stack.last()
            && nums[top] <= num
        {
            stack.pop();
            res += i64::from(right + 1 - top >= 3);
        }
        // [left_end] > [right_end]
        if stack.last().is_some_and(|&left| right + 1 - left >= 3) {
            res += 1;
        }
        stack.push(right);
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
    fn basics() {
        assert_eq!(bowl_subarrays(&[2, 5, 3, 1, 4]), 2);
        assert_eq!(bowl_subarrays(&[5, 1, 2, 3, 4]), 3);
    }

    #[test]
    fn test() {}
}
