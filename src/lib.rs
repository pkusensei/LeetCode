use std::collections::{BinaryHeap, HashMap};

pub fn minimum_cost(
    source: &str,
    target: &str,
    original: Vec<char>,
    changed: Vec<char>,
    cost: Vec<i32>,
) -> i64 {
    let mut edges = HashMap::new();
    for ((from, to), cost) in original.into_iter().zip(changed).zip(cost) {
        let cost = cost as i64;
        edges
            .entry((from, to))
            .and_modify(|v| {
                if *v > cost {
                    *v = cost
                }
            })
            .or_insert(cost);
    }
    let mut res = 0;
    let mut graph = HashMap::new();
    for (from, to) in source.chars().zip(target.chars()) {
        let Some(dist) = dijkstra(&edges, from, to, &graph) else {
            return -1;
        };
        graph.entry((from, to)).or_insert(dist);
        res += dist;
    }
    res
}

fn dijkstra(
    edges: &HashMap<(char, char), i64>,
    start: char,
    goal: char,
    graph: &HashMap<(char, char), i64>,
) -> Option<i64> {
    if let Some(&d) = graph.get(&(start, goal)) {
        return Some(d);
    }

    let mut dist: HashMap<_, _> = ('a'..='z').map(|c| (c, i64::MAX)).collect();
    let mut heap = BinaryHeap::new();
    dist.insert(start, 0);
    heap.push(State {
        cost: 0,
        pos: start,
    });

    while let Some(State { cost, pos }) = heap.pop() {
        if pos == goal {
            return Some(cost);
        }
        if cost > dist[&pos] {
            continue;
        }
        for edge in edges.iter().filter(|(k, _)| k.0 == pos) {
            let next = State {
                cost: cost + edge.1,
                pos: edge.0 .1,
            };
            if next.cost < dist[&next.pos] {
                heap.push(next);
                dist.insert(next.pos, next.cost);
            }
        }
    }

    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State {
    cost: i64,
    pos: char,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // BinaryHeap is a max priority queue
        // flip self and other
        other.cost.cmp(&self.cost)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            minimum_cost(
                "abcd",
                "acbe",
                vec!['a', 'b', 'c', 'c', 'e', 'd'],
                vec!['b', 'c', 'b', 'e', 'b', 'e'],
                vec![2, 5, 5, 1, 2, 20]
            ),
            28
        );
        debug_assert_eq!(
            minimum_cost("aaaa", "bbbb", vec!['a', 'c'], vec!['c', 'b'], vec![1, 2]),
            12
        );
        debug_assert_eq!(
            minimum_cost("abcd", "acbe", vec!['a'], vec!['e'], vec![10000]),
            -1
        );
    }

    #[test]
    fn test() {}
}
