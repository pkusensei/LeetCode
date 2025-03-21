mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn second_greater_element(nums: &[i32]) -> Vec<i32> {
    use std::{cmp::Reverse, collections::BinaryHeap};
    let n = nums.len();
    let mut res = vec![-1; n];
    let mut stack = Vec::<(i32, usize)>::new();
    let mut heap = BinaryHeap::<(Reverse<i32>, usize)>::new();
    for (idx, &num) in nums.iter().enumerate() {
        while heap.peek().is_some_and(|v| v.0.0 < num) {
            let i = heap.pop().unwrap().1;
            res[i] = num;
        }
        while stack.last().is_some_and(|v| v.0 < num) {
            let (top_num, top_idx) = stack.pop().unwrap();
            heap.push((Reverse(top_num), top_idx));
        }
        stack.push((num, idx));
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
        assert_eq!(second_greater_element(&[2, 4, 0, 9, 6]), [9, 6, 6, -1, -1]);
        assert_eq!(second_greater_element(&[3, 3]), [-1, -1]);
    }

    #[test]
    fn test() {}
}
