mod helper;

use std::collections::{HashMap, HashSet, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn calc_equation(equations: &[[&str; 2]], values: &[f64], queries: &[[&str; 2]]) -> Vec<f64> {
    let mut graph: HashMap<&str, HashMap<&str, f64>> =
        equations
            .iter()
            .zip(values)
            .fold(HashMap::new(), |mut acc, (eq, &v)| {
                acc.entry(eq[0]).or_default().insert(eq[1], v);
                acc.entry(eq[1]).or_default().insert(eq[0], 1.0 / v);
                acc
            });
    queries
        .iter()
        .map(|[start, goal]| bfs(&mut graph, start, goal))
        .collect()
}

fn bfs<'a>(
    graph: &mut HashMap<&'a str, HashMap<&'a str, f64>>,
    start: &'a str,
    goal: &'a str,
) -> f64 {
    if let Some(v) = graph.get(start).and_then(|m| m.get(goal)) {
        return *v;
    }
    if !graph.contains_key(start) || !graph.contains_key(goal) {
        return -1.0;
    }
    let mut queue = VecDeque::from([(start, 1.0)]);
    let mut seen = HashSet::new();
    while let Some((curr, dist)) = queue.pop_front() {
        if !seen.insert(curr) {
            continue;
        }
        if curr == goal {
            graph.entry(start).or_default().insert(goal, dist);
            graph.entry(goal).or_default().insert(start, 1.0 / dist);
            return dist;
        }
        for (next_node, delta) in graph[curr].iter() {
            let next_dist = delta * dist;
            queue.push_back((next_node, next_dist));
        }
    }
    -1.0
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            calc_equation(
                &[["a", "b"], ["b", "c"]],
                &[2.0, 3.0],
                &[["a", "c"], ["b", "a"], ["a", "e"], ["a", "a"], ["x", "x"]]
            ),
            [6.00000, 0.50000, -1.00000, 1.00000, -1.00000]
        );
        debug_assert_eq!(
            calc_equation(
                &[["a", "b"], ["b", "c"], ["bc", "cd"]],
                &[1.5, 2.5, 5.0],
                &[["a", "c"], ["c", "b"], ["bc", "cd"], ["cd", "bc"]]
            ),
            [3.75000, 0.40000, 5.00000, 0.20000]
        );
        debug_assert_eq!(
            calc_equation(
                &[["a", "b"]],
                &[0.5],
                &[["a", "b"], ["b", "a"], ["a", "c"], ["x", "y"]]
            ),
            [0.50000, 2.00000, -1.00000, -1.00000]
        );
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(i1: I1, i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: IntoIterator<Item = T1>,
        I2: IntoIterator<Item = T2>,
    {
        let mut v1: Vec<_> = i1.into_iter().collect();
        let mut v2: Vec<_> = i2.into_iter().collect();
        v1.sort_unstable();
        v2.sort_unstable();
        debug_assert_eq!(v1, v2);
    }
}
