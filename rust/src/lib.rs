mod helper;

use std::collections::{BinaryHeap, HashMap};

#[allow(unused_imports)]
use helper::*;

pub fn max_probability(n: i32, edges: &[[i32; 2]], succ_prob: &[f64], start: i32, end: i32) -> f64 {
    let graph: HashMap<i32, Vec<(i32, f64)>> =
        edges
            .iter()
            .zip(succ_prob)
            .fold(HashMap::new(), |mut acc, (&[n1, n2], &prob)| {
                acc.entry(n1).or_default().push((n2, prob.log2()));
                acc.entry(n2).or_default().push((n1, prob.log2()));
                acc
            });
    dijkstra(n, &graph, start, end)
        .map(|p| 2f64.powf(p))
        .unwrap_or(0.0)
}

fn dijkstra(n: i32, graph: &HashMap<i32, Vec<(i32, f64)>>, start: i32, end: i32) -> Option<f64> {
    let mut dist: Vec<_> = (0..n).map(|_| f64::MIN).collect();
    dist[start as usize] = 0.0;
    let mut heap = BinaryHeap::from([State {
        node: start,
        prob: 0.0,
    }]);
    while let Some(State { node, prob }) = heap.pop() {
        if node == end {
            return Some(prob);
        }
        if prob < dist[node as usize] {
            continue;
        }
        if let Some(adj) = graph.get(&node) {
            for edge in adj {
                let next = State {
                    node: edge.0,
                    prob: prob + edge.1,
                };
                if next.prob > dist[next.node as usize] {
                    heap.push(next);
                    dist[next.node as usize] = next.prob
                }
            }
        }
    }
    None
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct State {
    node: i32,
    prob: f64,
}

impl Eq for State {}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.prob
            .partial_cmp(&other.prob)
            .unwrap_or(self.node.cmp(&other.node))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            max_probability(3, &[[0, 1], [1, 2], [0, 2]], &[0.5, 0.5, 0.2], 0, 2),
            0.25000
        );
        debug_assert_eq!(
            max_probability(3, &[[0, 1], [1, 2], [0, 2]], &[0.5, 0.5, 0.3], 0, 2),
            0.30000
        );
        debug_assert_eq!(max_probability(3, &[[0, 1]], &[0.5], 0, 2), 0.0);
    }

    #[test]
    fn test() {}
}
