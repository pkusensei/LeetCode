mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_time(n: i32, edges: &[[i32; 4]]) -> i32 {
    use std::{cmp::Reverse, collections::BinaryHeap};

    let n = n as usize;
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        acc[a].push((b, e[2], e[3]));
        acc
    });
    let mut seen = vec![i32::MAX; n];
    seen[0] = 0;
    let mut heap = BinaryHeap::from([(Reverse(0), 0)]);
    while let Some((Reverse(cost), node)) = heap.pop() {
        if node == n - 1 {
            return cost;
        }
        if seen[node] < cost {
            continue;
        }
        for &(next, start, end) in &adj[node] {
            let ncost = if cost < start {
                1 + start
            } else if cost <= end {
                1 + cost
            } else {
                continue;
            };
            if seen[next] > ncost {
                seen[next] = ncost;
                heap.push((Reverse(ncost), next));
            }
        }
    }
    -1
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
