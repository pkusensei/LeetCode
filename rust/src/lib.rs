mod helper;

#[allow(unused_imports)]
use helper::*;

// pub fn max_number(nums1: &[i32], nums2: &[i32], k: i32) -> Vec<i32> {
//     todo!()
// }

// 1673
pub fn most_competitive(nums: &[i32], k: i32) -> Vec<i32> {
    fn with_binary_heap(nums: &[i32], k: i32) -> Vec<i32> {
        use std::{cmp::Reverse, collections::BinaryHeap};

        let mut queue = BinaryHeap::new();
        let mut res = vec![];
        for (i, &num) in nums.iter().enumerate() {
            queue.push(Reverse((num, i)));
            if i >= nums.len() - k as usize {
                let Some(Reverse((num, idx))) = queue.pop() else {
                    break;
                };
                res.push(num);
                while queue.peek().is_some_and(|&Reverse((_n, j))| j < idx) {
                    // pop any number eligible but to the left of (num, idx)
                    queue.pop();
                }
            }
        }
        res
    }

    let k = k as usize;
    // monotonically increasing stack
    let mut stack = Vec::with_capacity(k);
    for (i, &num) in nums.iter().enumerate() {
        while stack
            .last()
            .is_some_and(|&n| num < n && nums.len() - i > k - stack.len())
        {
            // nums.len() - i => amount of num left to choose from
            // k - stack.len() => amount of num missing in res
            stack.pop();
        }
        // Only keeps the smallest k numbers
        if stack.len() < k {
            stack.push(num);
        }
    }
    stack
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        // 1673
        debug_assert_eq!(most_competitive(&[3, 5, 2, 6], 2), [2, 6]);
        debug_assert_eq!(most_competitive(&[2, 4, 3, 3, 5, 4, 9, 6], 4), [2, 3, 3, 4]);

        // debug_assert_eq!(
        //     max_number(&[3, 4, 6, 5], &[9, 1, 2, 5, 8, 3], 5),
        //     [9, 8, 6, 5, 3]
        // );
        // debug_assert_eq!(max_number(&[6, 7], &[6, 0, 4], 5), [6, 7, 6, 0, 4]);
        // debug_assert_eq!(max_number(&[3, 9], &[8, 9], 3), [9, 8, 9]);
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(i1: I1, i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: IntoIterator<Item = T1>,
        I2: IntoIterator<Item = T2>,
    {
        let mut v1: Vec<_> = i1.into_iter().collect();
        let mut v2: Vec<_> = i2.into_iter().collect();
        v1.sort_unstable();
        v2.sort_unstable();
        debug_assert_eq!(v1, v2);
    }
}
