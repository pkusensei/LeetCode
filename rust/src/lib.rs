mod dsu;
mod helper;
mod trie;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

#[allow(unused_imports)]
use helper::*;

pub fn reachable_nodes(edges: &[[i32; 3]], max_moves: i32, n: i32) -> i32 {
    let n = n as usize;
    let mut graph = vec![vec![]; n];
    for e in edges {
        graph[e[0] as usize].push((e[1] as usize, e[2]));
        graph[e[1] as usize].push((e[0] as usize, e[2]));
    }
    let mut heap = BinaryHeap::from([(Reverse(0), 0)]);
    let mut dist = vec![1 + max_moves; 1 + n];
    dist[0] = 0;
    let mut used = HashMap::new(); // or edge length travelled
    let mut res = 0;
    while let Some((Reverse(moves), curr)) = heap.pop() {
        if moves > dist[curr] {
            continue;
        }
        res += 1;
        for &(next, weight) in graph[curr].iter() {
            used.insert((curr, next), weight.min(max_moves - moves));
            let total = moves + weight + 1; // total dist from start 0
            if total < dist[next].min(1 + max_moves) {
                heap.push((Reverse(total), next));
                dist[next] = total;
            }
        }
    }
    for e in edges {
        res += e[2].min(
            used.get(&(e[0] as usize, e[1] as usize)).unwrap_or(&0)
                + used.get(&(e[1] as usize, e[0] as usize)).unwrap_or(&0),
        )
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            reachable_nodes(&[[0, 1, 10], [0, 2, 1], [1, 2, 2]], 6, 3),
            13
        );
        debug_assert_eq!(
            reachable_nodes(&[[0, 1, 4], [1, 2, 6], [0, 2, 8], [1, 3, 1]], 10, 4),
            23
        );
        debug_assert_eq!(
            reachable_nodes(
                &[[1, 2, 4], [1, 4, 5], [1, 3, 1], [2, 3, 4], [3, 4, 5]],
                17,
                5
            ),
            1
        );
    }

    #[test]
    fn test() {
        debug_assert_eq!(
            reachable_nodes(
                &[[2, 4, 2], [3, 4, 5], [2, 3, 1], [0, 2, 1], [0, 3, 5]],
                14,
                5
            ),
            18
        );
    }

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

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64>,
        T2: Into<f64>,
    {
        const EP: f64 = 1e-5;
        debug_assert!((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP);
    }
}
