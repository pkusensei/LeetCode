mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_max_subarray_sum(nums: &[i32], k: i32) -> i64 {
    let k = k as i64;
    let [prev_smaller, next_smaller] = build(nums, |top, num| top > num);
    let [prev_greater, next_greater] = build(nums, |top, num| top < num);
    // nums[i] as local max + as local min
    solve(nums, k, prev_smaller, next_smaller) + solve(nums, k, prev_greater, next_greater)
}

fn solve(nums: &[i32], k: i64, prev: Vec<Option<usize>>, next: Vec<Option<usize>>) -> i64 {
    let n = nums.len();
    let mut res = 0;
    for (idx, &num) in nums.iter().enumerate() {
        let num = i64::from(num);
        let left = k.min(idx as i64 - prev[idx].map(|v| v as i64).unwrap_or(-1));
        let right = k.min((next[idx].unwrap_or(n) - idx) as i64);
        let extra = (left + right - 1 - k).max(0);
        res += num * (left * right - extra * (1 + extra) / 2);
    }
    res
}

fn build(nums: &[i32], f: fn(i32, i32) -> bool) -> [Vec<Option<usize>>; 2] {
    let n = nums.len();
    let mut next = vec![None; n];
    let mut stack = vec![];
    for (idx, &num) in nums.iter().enumerate() {
        while stack.last().is_some_and(|&i| f(nums[i], num)) {
            let top = stack.pop().unwrap();
            next[top] = Some(idx);
        }
        stack.push(idx);
    }
    stack.clear();
    let mut prev = vec![None; n];
    for (idx, &num) in nums.iter().enumerate().rev() {
        while stack
            .last()
            .is_some_and(|&i| f(nums[i], num) || nums[i] == num)
        {
            let top = stack.pop().unwrap();
            prev[top] = Some(idx);
        }
        stack.push(idx);
    }
    [prev, next]
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
        assert_eq!(min_max_subarray_sum(&[1, 2, 3], 2), 20);
        assert_eq!(min_max_subarray_sum(&[1, -3, 1], 2), -6);
    }

    #[test]
    fn test() {
        assert_eq!(min_max_subarray_sum(&[-7, -7], 2), -42);
        assert_eq!(min_max_subarray_sum(&[-7, 13], 1), 12);
    }
}
