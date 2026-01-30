mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
    use itertools::{Itertools, izip};
    use std::collections::BinaryHeap;
    let arr = izip!(quality.iter(), wage.iter())
        .map(|(&qu, &wa)| (qu, f64::from(wa) / f64::from(qu)))
        .sorted_unstable_by(|a, b| a.1.total_cmp(&b.1))
        .collect_vec();
    let mut sum = 0;
    let mut res = f64::MAX;
    let mut heap = BinaryHeap::new();
    // low rate of wage/quality
    // and low sum(quality)
    for curr in arr {
        sum += curr.0;
        heap.push(curr.0);
        if heap.len() > k as usize {
            let top = heap.pop().unwrap();
            sum -= top;
        }
        if heap.len() == k as usize {
            res = res.min(f64::from(sum) * curr.1);
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
    fn basics() {}

    #[test]
    fn test() {}
}
