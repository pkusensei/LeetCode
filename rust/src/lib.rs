mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_total_value(nums: &[i32], k: i32) -> i64 {
    use seg_tree::SegTree;
    use std::collections::BinaryHeap;
    let n = nums.len();
    let mintree = SegTree::new(nums, std::cmp::min, i32::MAX);
    let maxtree = SegTree::new(nums, std::cmp::max, i32::MIN);
    // (diff, left, right)
    let mut heap = BinaryHeap::with_capacity(n);
    for left in 0..n {
        let diff = maxtree.query(left, n - 1) - mintree.query(left, n - 1);
        heap.push((diff, left, n - 1));
    }
    let mut res = 0;
    for _ in 0..k {
        let Some((diff, left, right)) = heap.pop() else {
            break;
        };
        res += i64::from(diff);
        if left < right {
            let diff = maxtree.query(left, right - 1) - mintree.query(left, right - 1);
            heap.push((diff, left, right - 1));
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
        assert_eq!(max_total_value(&[1, 3, 2], 2), 4);
        assert_eq!(max_total_value(&[4, 2, 5, 1], 3), 12);
    }

    #[test]
    fn test() {
        assert_eq!(max_total_value(&[11, 8], 2), 3);
    }
}
