mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn reachable_nodes(edges: Vec<Vec<i32>>, max_moves: i32, n: i32) -> i32 {
    use std::{cmp::Reverse, collections::BinaryHeap};
    let n = n as usize;
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        acc[a].push((b, e[2]));
        acc[b].push((a, e[2]));
        acc
    });
    let mut heap = BinaryHeap::from([(Reverse(0), 0)]);
    let mut seen = vec![i32::MAX; n];
    seen[0] = 0;
    let mut moves_left = vec![vec![0; n]; n];
    let mut res = 0;
    while let Some((Reverse(dist), node)) = heap.pop() {
        if dist > seen[node] {
            continue;
        }
        res += 1;
        for &(next, cnt) in &adj[node] {
            let nd = dist + 1 + cnt;
            moves_left[node][next] = cnt.min(max_moves - dist);
            if nd < seen[next].min(1 + max_moves) {
                // reachable
                seen[next] = nd;
                heap.push((Reverse(nd), next));
            }
        }
    }
    res += edges
        .iter()
        .map(|e| {
            let [a, b] = [0, 1].map(|i| e[i] as usize);
            (moves_left[a][b] + moves_left[b][a]).min(e[2])
        })
        .sum::<i32>();
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
