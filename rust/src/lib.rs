mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_removal(nums: &[i32], mut queries: Vec<[i32; 2]>) -> i32 {
    use std::{cmp::Reverse, collections::BinaryHeap};
    queries.sort_unstable_by_key(|&q| Reverse(q));
    let mut candids = BinaryHeap::new();
    let mut used = BinaryHeap::<Reverse<usize>>::new();
    for (idx, &num) in nums.iter().enumerate() {
        while queries.last().is_some_and(|&q| (q[0] as usize) <= idx) {
            let [_, right] = queries.pop().unwrap();
            candids.push(right as usize);
        }
        while used.peek().is_some_and(|&Reverse(v)| v < idx) {
            used.pop();
        }
        while (num as usize) > used.len() {
            let Some(v) = candids.pop() else { return -1 };
            if v < idx {
                return -1;
            }
            used.push(Reverse(v));
        }
    }
    candids.len() as i32
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
        assert_eq!(max_removal(&[2, 0, 2], vec![[0, 2], [0, 2], [1, 1]]), 1);
        assert_eq!(
            max_removal(&[1, 1, 1, 1], vec![[1, 3], [0, 2], [1, 3], [1, 2]]),
            2
        );
        assert_eq!(max_removal(&[1, 2, 3, 4], vec![[0, 3]]), -1);
    }

    #[test]
    fn test() {}
}
