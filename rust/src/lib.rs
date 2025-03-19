mod dsu;
mod helper;
mod trie;

use std::collections::VecDeque;

#[allow(unused_imports)]
use helper::*;

pub fn min_operations(nums: &mut [i32]) -> i32 {
    let n = nums.len();
    let mut res = 0;
    for i in 0..=n - 3 {
        if nums[i] == 0 {
            for v in &mut nums[i..i + 3] {
                *v = 1 - *v;
            }
            res += 1;
        }
    }
    if nums[n - 2] == 1 && nums[n - 1] == 1 {
        res
    } else {
        -1
    }
}

pub fn with_deque(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut res = 0;
    let mut queue = VecDeque::new();
    for (idx, &num) in nums.iter().enumerate() {
        while queue.front().is_some_and(|&v| v + 2 < idx) {
            queue.pop_front();
        }
        if (num + queue.len() as i32) & 1 == 0 {
            if idx + 2 >= n {
                return -1;
            }
            res += 1;
            queue.push_back(idx);
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
        assert_eq!(min_operations(&mut [0, 1, 1, 1, 0, 0]), 3);
        assert_eq!(min_operations(&mut [0, 1, 1, 1]), -1);

        assert_eq!(with_deque(&[0, 1, 1, 1, 0, 0]), 3);
        assert_eq!(with_deque(&[0, 1, 1, 1]), -1);
    }

    #[test]
    fn test() {}
}
