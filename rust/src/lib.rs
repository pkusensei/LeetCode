mod helper;
mod trie;

use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap},
};

#[allow(unused_imports)]
use helper::*;

pub fn min_groups(intervals: &[[i32; 2]]) -> i32 {
    let mut map = BTreeMap::new();
    for v in intervals.iter() {
        *map.entry(v[0]).or_insert(0) += 1;
        *map.entry(v[1] + 1).or_insert(0) -= 1;
    }
    let (mut res, mut curr) = (0, 0);
    for v in map.into_values() {
        curr += v;
        res = res.max(curr);
    }
    res
}

fn with_pq(intervals: &mut [[i32; 2]]) -> usize {
    intervals.sort_unstable();
    let mut heap = BinaryHeap::new();
    for &[start, end] in intervals.iter() {
        // Ideally (without overlapping) an end should pop before next start
        // If it misses this chance, it has an overlap and stays in heap
        if heap.peek().is_some_and(|&Reverse(soon)| soon < start) {
            heap.pop();
        }
        heap.push(Reverse(end));
    }
    heap.len()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(with_pq(&mut [[5, 10], [6, 8], [1, 5], [2, 3], [1, 10]]), 3);
        debug_assert_eq!(with_pq(&mut [[1, 3], [5, 6], [8, 10], [11, 13]]), 1);
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
