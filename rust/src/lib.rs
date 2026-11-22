mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn earliest_activation_times(
    n: i32,
    edges: Vec<Vec<i32>>,
    mut required_signals: Vec<i32>,
) -> Vec<i64> {
    use std::{cmp::Reverse, collections::BinaryHeap};
    let n = n as usize;
    // (target, delay)
    let mut adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        acc[a].push((b, i64::from(e[2])));
        acc
    });
    // (R(time), node)
    let mut heap = BinaryHeap::from([(Reverse(0), 0)]);
    let mut res = vec![-1; n];
    required_signals[0] = 1;
    while let Some((Reverse(time), node)) = heap.pop() {
        if res[node] != -1 && res[node] <= time {
            continue;
        }
        required_signals[node] -= 1;
        if required_signals[node] <= 0 {
            res[node] = time;
            for (next, delay) in adj[node].drain(..) {
                heap.push((Reverse(time + delay), next));
            }
        }
    }
    res
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
    fn basics() {}

    #[test]
    fn test() {}
}
