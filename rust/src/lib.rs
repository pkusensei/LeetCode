mod helper;
mod trie;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

#[allow(unused_imports)]
use helper::*;

pub fn network_delay_time(times: &[[i32; 3]], n: i32, k: i32) -> i32 {
    let graph: HashMap<i32, Vec<[i32; 2]>> = times.iter().fold(HashMap::new(), |mut acc, v| {
        acc.entry(v[0]).or_default().push([v[1], v[2]]);
        acc
    });
    let Some(next) = graph.get(&k) else { return -1 };
    let mut nodes = HashMap::new();
    let mut heap = BinaryHeap::from([(Reverse(0), k)]);
    for v in next {
        heap.push((Reverse(v[1]), v[0]));
    }
    while let Some((Reverse(dist), node)) = heap.pop() {
        if nodes.get(&node).is_some_and(|&v| v <= dist) {
            continue;
        }
        nodes.insert(node, dist);
        let Some(next) = graph.get(&node) else {
            continue;
        };
        for v in next {
            heap.push((Reverse(v[1] + dist), v[0]));
        }
    }
    if nodes.len() == n as usize {
        nodes.into_values().max().unwrap_or(-1)
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            network_delay_time(&[[2, 1, 1], [2, 3, 1], [3, 4, 1]], 4, 2),
            2
        );
        debug_assert_eq!(network_delay_time(&[[1, 2, 1]], 2, 1), 1);
        debug_assert_eq!(network_delay_time(&[[1, 2, 1]], 2, 2), -1)
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
