mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;
use itertools::{Itertools, izip};
use std::collections::BinaryHeap;

pub fn min_sum_square_diff(nums1: Vec<i32>, nums2: Vec<i32>, k1: i32, k2: i32) -> i64 {
    let mut k = i64::from(k1 + k2);
    let mut heap = izip!(nums1.iter(), nums2.iter())
        .map(|(a, b)| i64::from((a - b).abs()))
        .filter(|v| *v > 0)
        .counts()
        .into_iter()
        .collect::<BinaryHeap<_>>();
    while let Some((val, mut f)) = heap.pop() {
        let d = (f as i64).min(k);
        f -= d as usize;
        if f > 0 {
            heap.push((val, f));
        }
        if let Some(&(v, ff)) = heap.peek()
            && v == val - 1
        {
            heap.pop();
            heap.push((v, ff + d as usize));
        } else if val > 1 {
            heap.push((val - 1, d as usize));
        }
        k -= d;
        if k == 0 {
            break;
        }
    }
    heap.into_iter().map(|(v, f)| v.pow(2) * f as i64).sum()
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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
        assert_eq!(min_insertions("(()))"), 1);
    }

    #[test]
    fn test() {}
}
