mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn find_redundant_directed_connection(edges: &[[i32; 2]]) -> Vec<i32> {
    let mut parents: HashMap<i32, i32> = HashMap::new();
    let mut cycle = None;
    let mut two_parents = vec![];
    for edge in edges.iter() {
        if let Some(&p1) = parents.get(&edge[1]) {
            // two parents
            two_parents.push([p1, edge[1]]);
            two_parents.push([edge[0], edge[1]]);
            if cycle.is_some() {
                return two_parents[0].to_vec();
            }
        } else {
            parents.insert(edge[1], edge[0]);
        }
        if cycle.is_none() && find_root(&parents, edge[0]).is_none() {
            cycle = Some(edge);
            if !two_parents.is_empty() {
                return two_parents[0].to_vec();
            }
        }
    }
    cycle
        .map(|v| v.to_vec())
        .or(two_parents.get(1).map(|v| v.to_vec()))
        .unwrap_or_default()
}

fn find_root(parents: &HashMap<i32, i32>, node: i32) -> Option<i32> {
    if !parents.contains_key(&node) {
        return Some(node); // root has no parent
    };
    let mut curr = node;
    while let Some(&p) = parents.get(&curr) {
        curr = p;
        if curr == node {
            return None; // a cycle
        }
    }
    Some(curr)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            find_redundant_directed_connection(&[[1, 2], [1, 3], [2, 3]]),
            [2, 3]
        );
        debug_assert_eq!(
            find_redundant_directed_connection(&[[1, 2], [2, 3], [3, 4], [4, 1], [1, 5]]),
            [4, 1]
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
