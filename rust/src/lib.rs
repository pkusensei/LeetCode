mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn valid_subarray_size(nums: &[i32], threshold: i32) -> i32 {
    let n = nums.len();
    let mut next_smaller = vec![-1; n];
    let mut stack = vec![];
    for (idx, &num) in nums.iter().enumerate() {
        while stack.last().is_some_and(|&i| num < nums[i]) {
            let top = stack.pop().unwrap();
            next_smaller[top] = idx as i32;
        }
        stack.push(idx);
    }
    let mut prev_smaller = vec![-1; n];
    stack.clear();
    for (idx, &num) in nums.iter().enumerate().rev() {
        while stack.last().is_some_and(|&i| num < nums[i]) {
            prev_smaller[stack.pop().unwrap()] = idx as i32;
        }
        stack.push(idx);
    }
    for ((left, right), &num) in prev_smaller.into_iter().zip(next_smaller).zip(nums.iter()) {
        let right = if right == -1 { n as i32 } else { right };
        let len = right - left - 1;
        if f64::from(num) > f64::from(threshold) / f64::from(len) {
            return len;
        }
    }
    -1
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
        assert_eq!(valid_subarray_size(&[1, 3, 4, 3, 1], 6), 3);
        assert_eq!(valid_subarray_size(&[6, 5, 6, 5, 8], 7), 5);
    }

    #[test]
    fn test() {}
}
