mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

const M: i32 = 1_000_000_007;

pub fn count_partitions(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::BTreeMap;
    let n = nums.len();
    let mut map = BTreeMap::new();
    let mut left = 0;
    let mut dp = vec![0; 1 + n];
    dp[0] = 1;
    let mut curr = 0;
    for (right, &num) in nums.iter().enumerate() {
        *map.entry(num).or_insert(0) += 1;
        curr = (curr + dp[right]) % M;
        while let (Some(a), Some(b)) = (map.keys().next(), map.keys().next_back())
            && b - a > k
        {
            let v = map.entry(nums[left]).or_insert(0);
            *v -= 1;
            if *v == 0 {
                map.remove(&nums[left]);
            }
            curr = (curr - dp[left]).rem_euclid(M);
            left += 1;
        }
        dp[1 + right] = curr;
    }
    dp[n]
}

pub fn with_deque(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::VecDeque;

    let n = nums.len();
    let mut dp = vec![0; 1 + n];
    dp[0] = 1;
    let mut curr = 0;
    let mut minq = VecDeque::new();
    let mut maxq = VecDeque::new();
    let mut left = 0;
    for (right, &num) in nums.iter().enumerate() {
        while minq.back().is_some_and(|&v| nums[v] >= num) {
            minq.pop_back();
        }
        minq.push_back(right);
        while maxq.back().is_some_and(|&v| nums[v] <= num) {
            maxq.pop_back();
        }
        maxq.push_back(right);
        curr = (curr + dp[right]) % M;
        while let (Some(&a), Some(&b)) = (minq.front(), maxq.front())
            && nums[b] - nums[a] > k
        {
            if a == left {
                minq.pop_front();
            }
            if b == left {
                maxq.pop_front();
            }
            curr = (curr - dp[left]).rem_euclid(M);
            left += 1;
        }
        dp[1 + right] = curr;
    }
    dp[n]
}

fn find_window_max(nums: &[i32], k: usize) -> Vec<i32> {
    use std::collections::VecDeque;

    let n = nums.len();
    let mut deque = VecDeque::with_capacity(k);
    let mut res = Vec::with_capacity(1 + n - k);
    for (right, &num) in nums.iter().enumerate() {
        while deque.back().is_some_and(|&v| nums[v] <= num) {
            deque.pop_back();
        }
        deque.push_back(right);
        while deque.front().is_some_and(|v| right - v > k) {
            deque.pop_front();
        }
        if right >= k - 1 {
            res.push(nums[*deque.front().unwrap()]);
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
    fn basics() {
        assert_eq!(
            find_window_max(&[1, 3, -1, -3, 5, 3, 6, 7], 3),
            [3, 3, 5, 5, 6, 7]
        );
    }

    #[test]
    fn test() {}
}
