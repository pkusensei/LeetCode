mod helper;

use std::collections::VecDeque;

#[allow(unused_imports)]
use helper::*;

pub fn max_sliding_window(nums: &[i32], k: i32) -> Vec<i32> {
    fn tle(nums: &[i32], k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut res = vec![];
        let mut queue = VecDeque::new();
        for (idx, &num) in nums.iter().enumerate() {
            // for every window, keep only the big elements in queue
            // queue.retain(|&n| n >= num); is somehow much slower and casues TLE
            while queue.back().is_some_and(|&n| n < num) {
                queue.pop_back();
            }
            queue.push_back(num);
            if idx >= k && nums[idx - k] == queue[0] {
                // one big num out of window
                queue.pop_front();
            }
            if idx >= k - 1 {
                res.push(queue[0])
            }
        }
        res
    }

    let k = k as usize;
    if nums.len() <= k {
        return vec![nums.iter().copied().max().unwrap_or(0)];
    }
    let mut queue: VecDeque<_> = (0..k).map(|i| nums[i]).collect();
    let mut curr_max = queue.iter().copied().max().unwrap();
    let mut res = vec![curr_max];

    for &num in nums.iter().skip(k) {
        let popped = queue.pop_front();
        queue.push_back(num);
        curr_max = curr_max.max(num);
        if popped.is_some_and(|n| n == curr_max) {
            curr_max = queue.iter().copied().max().unwrap();
        }
        res.push(curr_max);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            max_sliding_window(&[1, 3, -1, -3, 5, 3, 6, 7], 3),
            [3, 3, 5, 5, 6, 7]
        );
        debug_assert_eq!(max_sliding_window(&[1], 1), [1]);
    }

    #[test]
    fn test() {}
}
