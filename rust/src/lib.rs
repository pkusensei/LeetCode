mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};
type MinHeap = BinaryHeap<(Reverse<i64>, usize)>;
type MaxHeap = BinaryHeap<(i64, usize)>;

pub fn slide_with_heap(nums: &[i64], k: usize, size: usize) -> i64 {
    let mut res = i64::MAX >> 1;
    // Currently used in computing cost
    let mut max_heap = MaxHeap::new();
    // Candidates available
    let mut min_heap = MinHeap::new();
    let mut used = HashSet::new();
    let mut curr = 0;
    for (right, &num) in nums.iter().enumerate() {
        if let Some(left) = right.checked_sub(size)
            && used.remove(&left)
        {
            // [left] is out of window
            // but it is used in calculation
            curr = drop_left(nums, &mut max_heap, &mut min_heap, &mut used, left, curr)
        }
        if used.len() < k {
            // Not enough numbers
            // Add in right boundary to maintain valid window
            curr = add_right(&mut max_heap, &mut used, num, right, curr);
        } else {
            while let Some(top) = max_heap.peek()
                && !used.contains(&top.1)
            {
                max_heap.pop(); // pop unused
            }
            if let Some(top) = max_heap.peek()
                && top.0 > num
            {
                // [right]<top
                curr = balance(&mut max_heap, &mut min_heap, &mut used, num, right, curr)
            } else {
                min_heap.push((Reverse(num), right));
            }
        }
        if right >= size - 1 {
            res = res.min(curr);
        }
    }
    res
}

fn balance(
    max_heap: &mut MaxHeap,
    min_heap: &mut MinHeap,
    used: &mut HashSet<usize>,
    num: i64,
    right: usize,
    curr: i64,
) -> i64 {
    // Move top into candidates
    // Add in [right]
    let top = max_heap.pop().unwrap();
    min_heap.push((Reverse(top.0), top.1));
    used.remove(&top.1);
    max_heap.push((num, right));
    used.insert(right);
    curr + num - top.0
}

fn add_right(
    max_heap: &mut MaxHeap,
    used: &mut HashSet<usize>,
    num: i64,
    right: usize,
    curr: i64,
) -> i64 {
    used.insert(right);
    max_heap.push((num, right));
    curr + num
}

fn drop_left(
    nums: &[i64],
    max_heap: &mut MaxHeap,
    min_heap: &mut MinHeap,
    used: &mut HashSet<usize>,
    left: usize,
    mut curr: i64,
) -> i64 {
    curr -= nums[left];
    // Remove elements outside window from candidates
    while let Some((_, i)) = min_heap.peek()
        && *i < left
    {
        min_heap.pop();
    }
    // Can still find a small element
    if let Some((Reverse(top), i)) = min_heap.pop() {
        used.insert(i);
        curr += top;
        max_heap.push((top, i));
    }
    curr
}

pub fn minimum_cost(nums: &[i32], k: i32, dist: i32) -> i64 {
    let [k, dist] = [k, dist].map(|v| v as usize);
    let res = i64::from(nums[0]);
    res + slide_with_heap(
        &nums[1..].iter().map(|&v| i64::from(v)).collect::<Vec<_>>(),
        k - 1,
        1 + dist,
    )
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
        assert_eq!(slide_with_heap(&[3, 2, 6, 4, 2], 2, 4), 4);
        assert_eq!(slide_with_heap(&[1, 2, 2, 2, 1], 3, 4), 5);
        assert_eq!(slide_with_heap(&[8, 18, 9], 2, 2), 26);

        assert_eq!(minimum_cost(&[1, 3, 2, 6, 4, 2], 3, 3), 5);
        assert_eq!(minimum_cost(&[10, 1, 2, 2, 2, 1], 4, 3), 15);
        assert_eq!(minimum_cost(&[10, 8, 18, 9], 3, 1), 36);
    }

    #[test]
    fn test() {}
}
