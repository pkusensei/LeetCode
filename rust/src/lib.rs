mod dsu;
mod helper;
mod trie;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

#[allow(unused_imports)]
use helper::*;
use itertools::{chain, izip};

pub fn minimum_cost(
    source: &str,
    target: &str,
    original: &[&str],
    changed: &[&str],
    cost: &[i32],
) -> i64 {
    let mut id_map = HashMap::new();
    let mut id = 0;
    for s in chain!(original.iter(), changed.iter()) {
        let v = id_map.entry(s.as_bytes()).or_insert(id);
        if *v == id {
            id += 1;
        }
    }
    let cn = id_map.len();
    let mut graph = vec![vec![]; cn];
    let mut lengths = HashSet::new();
    for (a, b, &c) in izip!(original.iter(), changed.iter(), cost.iter()) {
        let i1 = id_map[&a.as_bytes()];
        let i2 = id_map[&b.as_bytes()];
        graph[i1].push((i2, i64::from(c)));
        lengths.insert(a.len());
    }
    let sn = source.len();
    let mut dp = vec![i64::MAX; 1 + sn];
    dp[0] = 0;
    let mut memo = HashMap::new();
    for len in 1..=sn {
        if source.as_bytes()[len - 1] == target.as_bytes()[len - 1] {
            dp[len] = dp[len].min(dp[len - 1])
        }
        for &val in &lengths {
            if len < val {
                continue;
            }
            let left = len - val;
            let src = source[left..len].as_bytes();
            let tgt = target[left..len].as_bytes();
            if let Some((&i1, &i2)) = id_map.get(&src).zip(id_map.get(&tgt)) {
                let key = [i1, i2];
                let cost = *memo.entry(key).or_insert_with(|| dijkstra(&graph, i1, i2));
                if cost < i64::MAX {
                    dp[len] = dp[len].min(dp[left].saturating_add(cost));
                }
            }
        }
    }
    if dp[sn] == i64::MAX { -1 } else { dp[sn] }
}

fn dijkstra(graph: &[Vec<(usize, i64)>], start: usize, goal: usize) -> i64 {
    let mut dists = vec![i64::MAX; graph.len()];
    let mut heap = BinaryHeap::from([(Reverse(0), start)]);
    dists[start] = 0;
    while let Some((Reverse(dist), node)) = heap.pop() {
        if node == goal {
            return dist;
        }
        if dist > dists[node] {
            continue;
        }
        for &(next, d) in &graph[node] {
            let nc = d + dist;
            if nc < dists[next] {
                dists[next] = nc;
                heap.push((Reverse(nc), next));
            }
        }
    }
    i64::MAX
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
    fn basics() {
        assert_eq!(
            minimum_cost(
                "abcd",
                "acbe",
                &["a", "b", "c", "c", "e", "d"],
                &["b", "c", "b", "e", "b", "e"],
                &[2, 5, 5, 1, 2, 20]
            ),
            28
        );
        assert_eq!(
            minimum_cost(
                "abcdefgh",
                "acdeeghh",
                &["bcd", "fgh", "thh"],
                &["cde", "thh", "ghh"],
                &[1, 3, 5]
            ),
            9
        );
        assert_eq!(
            minimum_cost(
                "abcdefgh",
                "addddddd",
                &["bcd", "defgh"],
                &["ddd", "ddddd"],
                &[100, 1578]
            ),
            -1
        );
    }

    #[test]
    fn test() {}
}
