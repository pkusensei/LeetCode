mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn constrained_subset_sum(nums: &[i32], k: i32) -> i32 {
    let k = k as usize;
    let mut heap = std::collections::BinaryHeap::from([(nums[0], 0)]);
    let mut res = nums[0];
    for (idx, &num) in nums.iter().enumerate().skip(1) {
        while heap.peek().is_some_and(|v| idx - v.1 > k) {
            heap.pop();
        }
        let curr = num + heap.peek().map(|v| v.0.max(0)).unwrap_or(0);
        res = res.max(curr);
        heap.push((curr, idx));
    }
    res
}

fn with_deque(nums: &[i32], k: i32) -> i32 {
    let (n, k) = (nums.len(), k as usize);
    let mut dp = vec![0; n];
    let mut queue = std::collections::VecDeque::new();
    for (idx, &num) in nums.iter().enumerate() {
        while queue.front().is_some_and(|v| idx - v > k) {
            queue.pop_front();
        }
        dp[idx] = num + queue.front().map(|&i| dp[i]).unwrap_or(0);
        while queue.back().is_some_and(|&v| dp[v] < dp[idx]) {
            queue.pop_back();
        }
        if dp[idx] > 0 {
            queue.push_back(idx);
        }
    }
    dp.into_iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(constrained_subset_sum(&[10, 2, -10, 5, 20], 2), 37);
        assert_eq!(constrained_subset_sum(&[-1, -2, -3], 1), -1);
        assert_eq!(constrained_subset_sum(&[10, -2, -10, -5, 20], 2), 23);

        assert_eq!(with_deque(&[10, 2, -10, 5, 20], 2), 37);
        assert_eq!(with_deque(&[-1, -2, -3], 1), -1);
        assert_eq!(with_deque(&[10, -2, -10, -5, 20], 2), 23);
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }

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
