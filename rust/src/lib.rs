mod dsu;
mod helper;
mod trie;

use std::collections::{HashMap, HashSet};

#[allow(unused_imports)]
use helper::*;

pub fn check_ways(pairs: &[[i32; 2]]) -> i32 {
    let mut adj = pairs
        .iter()
        .fold(HashMap::<_, HashSet<_>>::new(), |mut acc, e| {
            acc.entry(e[0]).or_default().insert(e[1]);
            acc.entry(e[1]).or_default().insert(e[0]);
            acc
        });
    let nodes: Vec<_> = adj.keys().copied().collect();
    dfs(&mut adj, &nodes)
}

fn dfs(adj: &mut HashMap<i32, HashSet<i32>>, nodes: &[i32]) -> i32 {
    let mut it = nodes.iter().filter(|&node| {
        adj.get(node)
            .is_some_and(|set| set.len() == nodes.len() - 1)
    });
    // possible roots => possible configs at this level
    let (root, count) = match (it.next(), it.next()) {
        (Some(&root), None) => (root, 1),
        (Some(&root), Some(_)) => (root, 2),
        _ => return 0,
    };
    for set in adj.values_mut() {
        set.remove(&root);
    }
    adj.remove(&root);
    let mut seen = HashSet::new();
    let mut res = count;
    for &node in nodes {
        if node != root && seen.insert(node) {
            let next = build(adj, &mut seen, node);
            match dfs(adj, &next) {
                0 => return 0,
                2 => res *= 2, // propagate from subgraph
                _ => (),
            }
        }
    }
    if res >= 2 {
        2
    } else {
        1
    }
}

// build subgraph
fn build(adj: &HashMap<i32, HashSet<i32>>, seen: &mut HashSet<i32>, curr: i32) -> Vec<i32> {
    let mut res = vec![curr];
    for &neighbor in adj[&curr].iter() {
        if seen.insert(neighbor) {
            res.extend(build(adj, seen, neighbor));
        }
    }
    res
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        };
    }

    #[test]
    fn basics() {
        assert_eq!(check_ways(&[[1, 2], [2, 3]]), 1);
        assert_eq!(check_ways(&[[1, 2], [2, 3], [1, 3]]), 2);
        assert_eq!(check_ways(&[[1, 2], [2, 3], [2, 4], [1, 5]]), 0);
    }

    #[test]
    fn test() {
        assert_eq!(
            check_ways(&[[1, 5], [1, 3], [2, 3], [2, 4], [3, 5], [3, 4]]),
            2
        );
    }
}
