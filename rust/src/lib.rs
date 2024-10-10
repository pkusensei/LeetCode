mod helper;
mod trie;

use std::collections::{HashMap, HashSet};

#[allow(unused_imports)]
use helper::*;

pub fn find_redundant_connection(edges: &[[i32; 2]]) -> Vec<i32> {
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut seen = HashSet::new();
        for edge in edges.iter() {
            seen.clear();
            if graph.get(&edge[0]).is_some_and(|v| !v.is_empty())
                && graph.get(&edge[1]).is_some_and(|v| !v.is_empty())
                && dfs(&graph, &mut seen, edge[0], edge[1])
            {
                return edge.to_vec();
            }
            graph.entry(edge[0]).or_default().push(edge[1]);
            graph.entry(edge[1]).or_default().push(edge[0]);
        }
        edges[0].to_vec() // unreachable!()
}

fn dfs(graph: &HashMap<i32, Vec<i32>>, seen: &mut HashSet<i32>, from: i32, to: i32) -> bool {
    if !seen.insert(from) {
        return false;
    }
    if from == to {
        return true;
    }
    if let Some(v) = graph.get(&from) {
        for &n in v.iter() {
            if dfs(graph, seen, n, to) {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(find_redundant_connection(&[[1, 2], [1, 3], [2, 3]]), [2, 3]);
        debug_assert_eq!(
            find_redundant_connection(&[[1, 2], [2, 3], [3, 4], [1, 4], [1, 5]]),
            [1, 4]
        );
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }
}
