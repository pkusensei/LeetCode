mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

#[allow(unused_imports)]
use helper::*;

pub fn minimum_time(n: i32, edges: Vec<Vec<i32>>, disappear: Vec<i32>) -> Vec<i32> {
    let n = n as usize;
    let adj = edges.iter().fold(vec![HashMap::new(); n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        let v = acc[a].entry(b).or_insert(e[2]);
        *v = (*v).min(e[2]);
        let v = acc[b].entry(a).or_insert(e[2]);
        *v = (*v).min(e[2]);
        acc
    });
    let mut res = vec![-1; n];
    res[0] = 0;
    let mut heap = BinaryHeap::from([(Reverse(0), 0)]);
    while let Some((Reverse(cost), node)) = heap.pop() {
        if cost > res[node] {
            continue;
        }
        for (&next, &c) in adj[node].iter() {
            let nc = cost + c;
            if nc < disappear[next] && (res[next] == -1 || res[next] > nc) {
                res[next] = nc;
                heap.push((Reverse(nc), next));
            }
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
