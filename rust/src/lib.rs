mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_star_sum(vals: Vec<i32>, edges: Vec<Vec<i32>>, k: i32) -> i32 {
    use std::{cmp::Reverse, collections::BinaryHeap};
    let n = vals.len();
    let k = k as usize;
    let mut adj = vec![BinaryHeap::with_capacity(1 + k); n];
    for e in edges.iter() {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        if vals[b] > 0 {
            adj[a].push(Reverse(vals[b]));
        }
        if vals[a] > 0 {
            adj[b].push(Reverse(vals[a]));
        }
        if adj[a].len() > k {
            adj[a].pop();
        }
        if adj[b].len() > k {
            adj[b].pop();
        }
    }
    adj.into_iter()
        .enumerate()
        .map(|(i, heap)| vals[i] + heap.into_iter().map(|v| v.0).sum::<i32>())
        .max()
        .unwrap()
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
