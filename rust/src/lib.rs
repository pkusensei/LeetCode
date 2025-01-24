mod dsu;
mod helper;
mod trie;

use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap},
};

#[allow(unused_imports)]
use helper::*;

pub fn busiest_servers(k: i32, arrival: &[i32], load: &[i32]) -> Vec<i32> {
    // server <-> count, endtime
    let mut empty: BTreeMap<_, _> = (0..k).map(|i| (i, (0, 0))).collect();
    let mut heap = BinaryHeap::new();
    for (i, (&arr, &load)) in (0..).zip(arrival.iter().zip(load.iter())) {
        while heap.peek().is_some_and(|&Reverse((t, _, _))| t <= arr) {
            let Reverse((time, id, count)) = heap.pop().unwrap();
            empty.insert(id, (count, time));
        }
        if empty.is_empty() {
            continue;
        }
        let (&k, &(count, _)) = empty.range(i % k..).next().or(empty.iter().next()).unwrap();
        empty.remove(&k);
        heap.push(Reverse((arr + load, k, 1 + count)));
    }
    while let Some(Reverse((time, id, count))) = heap.pop() {
        empty.insert(id, (count, time));
    }
    let max = empty.values().map(|&(count, _)| count).max().unwrap_or(0);
    empty
        .into_iter()
        .filter_map(|(k, (count, _))| if count == max { Some(k) } else { None })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    #[test]
    fn basics() {
        sort_eq!(busiest_servers(3, &[1, 2, 3, 4, 5], &[5, 2, 3, 3, 3]), [1]);
        sort_eq!(busiest_servers(3, &[1, 2, 3, 4], &[1, 2, 1, 2]), [0]);
        sort_eq!(busiest_servers(3, &[1, 2, 3], &[12, 11, 10]), [1, 0, 2]);
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
