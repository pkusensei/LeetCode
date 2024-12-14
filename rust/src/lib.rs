mod dsu;
mod helper;
mod trie;

use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap, VecDeque},
};

#[allow(unused_imports)]
use helper::*;

pub fn continuous_subarrays(nums: &[i32]) -> i64 {
    let mut res = 0;
    let mut left = 0;
    let mut map = BTreeMap::new();
    for (right, &num) in nums.iter().enumerate() {
        *map.entry(num).or_insert(0) += 1;
        while map.keys().next_back().unwrap() - map.keys().next().unwrap() > 2 {
            map.entry(nums[left]).and_modify(|v| (*v) -= 1);
            if map[&nums[left]] == 0 {
                map.remove(&nums[left]);
            }
            left += 1;
        }
        res += right - left + 1;
    }
    res as i64
}

fn two_ptrs(nums: &[i32]) -> i64 {
    let mut res = 0;
    let mut left = 0;
    let [mut min, mut max] = [nums[0]; 2];
    for (right, &num) in nums.iter().enumerate() {
        min = min.min(num);
        max = max.max(num);
        if max - min > 2 {
            // window is broken
            let window = right - left;
            res += window * (1 + window) / 2;
            left = right;
            [min, max] = [num; 2];
            while left > 0 && num.abs_diff(nums[left - 1]) <= 2 {
                left -= 1;
                min = min.min(nums[left]);
                max = max.max(nums[left]);
            }
            if left < right {
                let window = right - left;
                res -= window * (1 + window) / 2;
            }
        }
    }
    let window = nums.len() - left;
    res += window * (1 + window) / 2;
    res as _
}

fn with_heap(nums: &[i32]) -> i64 {
    let mut res = 0;
    let mut left = 0;
    let mut min_heap = BinaryHeap::new();
    let mut max_heap = BinaryHeap::new();
    for (right, &num) in nums.iter().enumerate() {
        min_heap.push((Reverse(num), right));
        max_heap.push((num, right));
        while left < right && max_heap.peek().unwrap().0 - min_heap.peek().unwrap().0 .0 > 2 {
            left += 1;
            while min_heap.peek().is_some_and(|(_, i)| *i < left) {
                min_heap.pop();
            }
            while max_heap.peek().is_some_and(|(_, i)| *i < left) {
                max_heap.pop();
            }
        }
        res += right + 1 - left;
    }
    res as _
}

fn with_deque(nums: &[i32]) -> i64 {
    let mut res = 0;
    let mut left = 0;
    let mut min_q = VecDeque::new();
    let mut max_q = VecDeque::new();
    for (right, &num) in nums.iter().enumerate() {
        // increasing
        while min_q.back().is_some_and(|(_, v)| *v > num) {
            min_q.pop_back();
        }
        min_q.push_back((right, num));
        // decreasing
        while max_q.back().is_some_and(|(_, v)| *v < num) {
            max_q.pop_back();
        }
        max_q.push_back((right, num));
        while min_q
            .front()
            .zip(max_q.front())
            .is_some_and(|(a, b)| a.1 + 2 < b.1)
        {
            if min_q.front().unwrap().0 < max_q.front().unwrap().0 {
                left = 1 + min_q.pop_front().unwrap().0;
            } else {
                left = 1 + max_q.pop_front().unwrap().0;
            }
        }
        res += right - left + 1;
    }
    res as _
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(two_ptrs(&[5, 4, 2, 4]), 8);
        assert_eq!(two_ptrs(&[1, 2, 3]), 6);
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
        if (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() > EP {
            dbg!(a, b);
        }
    }
}
