mod dsu;
mod helper;
mod trie;

use std::collections::{BinaryHeap, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn max_result(nums: &[i32], k: i32) -> i32 {
    let (n, k) = (nums.len(), k as usize);
    let mut dp = vec![i32::MIN; n];
    dp[n - 1] = nums[n - 1];
    let mut heap = BinaryHeap::from([(dp[n - 1], n - 1)]);
    for (idx, &num) in nums.iter().enumerate().take(n - 1).rev() {
        while heap.peek().is_some_and(|&(_val, i)| idx + k < i) {
            heap.pop();
        }
        dp[idx] = num + heap.peek().map(|(val, _i)| val).unwrap_or(&0);
        heap.push((dp[idx], idx));
    }
    dp[0]
}

pub fn with_deque(nums: &[i32], k: i32) -> i32 {
    let (n, k) = (nums.len(), k as usize);
    let mut dp = vec![i32::MIN; n];
    dp[0] = nums[0];
    let mut queue = VecDeque::from([0]);
    for (idx, &num) in nums.iter().enumerate().skip(1) {
        // queue.front() is the max, but is potentially out of reach
        while queue.front().is_some_and(|i| i + k < idx) {
            queue.pop_front();
        }
        dp[idx] = num + queue.front().map(|&i| dp[i]).unwrap_or(0);
        // Maintaining a decreasing queue
        // i.e for queue [5,4,2], before pushing 3 in, pop 2 first
        while queue.back().is_some_and(|&i| dp[i] <= dp[idx]) {
            queue.pop_back();
        }
        queue.push_back(idx);
    }
    dp[n - 1]
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    #[test]
    fn basics() {
        assert_eq!(max_result(&[1, -1, -2, 4, -7, 3], 2), 7);
        assert_eq!(max_result(&[10, -5, -2, 4, 0, 3], 3), 17);
        assert_eq!(max_result(&[1, -5, -20, 4, -1, 3, -6, -3], 2), 0);

        assert_eq!(with_deque(&[1, -1, -2, 4, -7, 3], 2), 7);
        assert_eq!(with_deque(&[10, -5, -2, 4, 0, 3], 3), 17);
        assert_eq!(with_deque(&[1, -5, -20, 4, -1, 3, -6, -3], 2), 0);
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
