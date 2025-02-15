mod dsu;
mod helper;
mod trie;

use std::collections::{BinaryHeap, HashSet};

#[allow(unused_imports)]
use helper::*;

pub fn min_cost(max_time: i32, edges: &[[i32; 3]], passing_fees: &[i32]) -> i32 {
    let n = passing_fees.len();
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        let (x, y, time) = (e[0] as usize, e[1] as usize, e[2]);
        acc[x].push((y, time));
        acc[y].push((x, time));
        acc
    });
    let mut heap = BinaryHeap::from([State {
        cost: passing_fees[0],
        time: 0,
        node: 0,
    }]);
    let mut seen = HashSet::from([(0, 0)]);
    let mut min_cost = i32::MAX;
    while let Some(State { cost, time, node }) = heap.pop() {
        if time > max_time {
            continue;
        }
        if node == n - 1 {
            min_cost = min_cost.min(cost);
        }
        for &(next_node, delta_time) in adj[node].iter() {
            let next_time = time + delta_time;
            if seen.insert((next_node, next_time)) && next_time <= max_time {
                heap.push(State {
                    cost: cost + passing_fees[next_node],
                    time: next_time,
                    node: next_node,
                });
            }
        }
    }
    if min_cost == i32::MAX {
        -1
    } else {
        min_cost
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State {
    cost: i32,
    time: i32,
    node: usize,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
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
            min_cost(
                30,
                &[
                    [0, 1, 10],
                    [1, 2, 10],
                    [2, 5, 10],
                    [0, 3, 1],
                    [3, 4, 10],
                    [4, 5, 15]
                ],
                &[5, 1, 2, 20, 20, 3]
            ),
            11
        );
        assert_eq!(
            min_cost(
                29,
                &[
                    [0, 1, 10],
                    [1, 2, 10],
                    [2, 5, 10],
                    [0, 3, 1],
                    [3, 4, 10],
                    [4, 5, 15]
                ],
                &[5, 1, 2, 20, 20, 3],
            ),
            48
        );
        assert_eq!(
            min_cost(
                25,
                &[
                    [0, 1, 10],
                    [1, 2, 10],
                    [2, 5, 10],
                    [0, 3, 1],
                    [3, 4, 10],
                    [4, 5, 15]
                ],
                &[5, 1, 2, 20, 20, 3]
            ),
            -1
        );
    }

    #[test]
    fn test() {}
}
