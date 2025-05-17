mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn results_array(queries: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    use std::collections::BinaryHeap;
    let mut res = vec![];
    let mut heap = BinaryHeap::new();
    let k = k as usize;
    for q in queries {
        heap.push(q[0].abs() + q[1].abs());
        if heap.len() > k {
            heap.pop();
        }
        if heap.len() < k {
            res.push(-1);
        } else {
            res.push(*heap.peek().unwrap());
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
    fn basics() {}

    #[test]
    fn test() {}
}
