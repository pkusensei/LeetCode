mod helper;
mod trie;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

#[allow(unused_imports)]
use helper::*;

pub fn find_cheapest_price(n: i32, flights: &[[i32; 3]], src: i32, dst: i32, k: i32) -> i32 {
    let graph: HashMap<i32, Vec<[i32; 2]>> = flights.iter().fold(HashMap::new(), |mut acc, v| {
        acc.entry(v[0]).or_default().push([v[1], v[2]]);
        acc
    });
    let mut heap = BinaryHeap::from([(Reverse((0, 0)), src)]);
    let mut stops = vec![i32::MAX; n as usize];
    while let Some((Reverse((cost, stop)), curr)) = heap.pop() {
        if stop > 1 + k || stop > stops[curr as usize] {
            continue;
        }
        stops[curr as usize] = stop;
        if dst == curr {
            return cost;
        }
        let Some(nodes) = graph.get(&curr) else {
            continue;
        };
        for next in nodes.iter() {
            heap.push((Reverse((cost + next[1], 1 + stop)), next[0]));
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            find_cheapest_price(
                4,
                &[
                    [0, 1, 100],
                    [1, 2, 100],
                    [2, 0, 100],
                    [1, 3, 600],
                    [2, 3, 200]
                ],
                0,
                3,
                1
            ),
            700
        );
        debug_assert_eq!(
            find_cheapest_price(3, &[[0, 1, 100], [1, 2, 100], [0, 2, 500]], 0, 2, 1),
            200
        );
        debug_assert_eq!(
            find_cheapest_price(3, &[[0, 1, 100], [1, 2, 100], [0, 2, 500]], 0, 2, 0),
            500
        );
    }

    #[test]
    fn test() {
        debug_assert_eq!(
            find_cheapest_price(4, &[[0, 1, 1], [0, 2, 5], [1, 2, 1], [2, 3, 1]], 0, 3, 1),
            6
        );
        debug_assert_eq!(
            find_cheapest_price(
                4,
                &[
                    [0, 3, 59],
                    [2, 0, 83],
                    [2, 3, 32],
                    [0, 2, 97],
                    [3, 1, 16],
                    [1, 3, 16]
                ],
                3,
                0,
                3
            ),
            -1
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
}
