mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn shortest_path(n: i32, edges: Vec<Vec<i32>>, labels: String, k: i32) -> i32 {
    use std::{
        cmp::Reverse,
        collections::{BinaryHeap, HashMap},
    };
    let n = n as usize;
    let adj = edges.iter().fold(vec![HashMap::new(); n], |mut acc, e| {
        let v = acc[e[0] as usize].entry(e[1] as usize).or_insert(e[2]);
        *v = (*v).min(e[2]);
        acc
    });
    let s = labels.as_bytes();
    let mut heap = BinaryHeap::from([(Reverse(0), 0, 1)]);
    let mut weights = vec![vec![i32::MAX >> 1; 1 + k as usize]; n];
    let mut res = i32::MAX >> 1;
    while let Some((Reverse(weight), node, streak)) = heap.pop() {
        if node == n - 1 {
            res = res.min(weight);
            continue;
        }
        if weight > weights[node][streak] {
            continue;
        }
        for (&next, &w) in adj[node].iter() {
            let (nw, ns) = if s[next] != s[node] {
                (weight + w, 1)
            } else if 1 + streak <= k as usize {
                (weight + w, 1 + streak)
            } else {
                continue;
            };
            if nw < weights[next][ns] {
                weights[next][ns] = nw;
                heap.push((Reverse(nw), next, ns));
            }
        }
    }
    if res >= i32::MAX >> 1 { -1 } else { res }
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
