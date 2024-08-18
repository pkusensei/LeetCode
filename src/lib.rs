mod helper;

use std::collections::{HashMap, HashSet};

#[allow(unused_imports)]
use helper::*;

pub fn can_finish(num_courses: i32, prerequisites: &[[i32; 2]]) -> bool {
    let graph: HashMap<i32, Vec<_>> = prerequisites.iter().fold(HashMap::new(), |mut acc, p| {
        acc.entry(p[0]).or_default().push(p[1]);
        acc
    });

    let (mut seen, mut path) = (HashSet::new(), HashSet::new());
    for node in 0..num_courses {
        if has_cycle(node, &graph, &mut seen, &mut path) {
            return false;
        }
    }
    true
}

fn has_cycle(
    node: i32,
    graph: &HashMap<i32, Vec<i32>>,
    seen: &mut HashSet<i32>,
    path: &mut HashSet<i32>,
) -> bool {
    if path.contains(&node) {
        return true;
    }
    if seen.contains(&node) {
        return false;
    }

    seen.insert(node);
    path.insert(node);

    if let Some(neighbors) = graph.get(&node) {
        for &neighbor in neighbors {
            if has_cycle(neighbor, graph, seen, path) {
                return true;
            }
        }
    }
    path.remove(&node);
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert!(can_finish(2, &[[1, 0]]));
        debug_assert!(!can_finish(2, &[[1, 0], [0, 1]]));
    }

    #[test]
    fn test() {
        debug_assert!(can_finish(5, &[[1, 4], [2, 4], [3, 1], [3, 2]]));
    }
}
