mod helper;
mod trie;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet, VecDeque},
};

#[allow(unused_imports)]
use helper::*;

pub fn cut_off_tree(forest: &[&[i32]]) -> i32 {
    let mut trees = BinaryHeap::new();
    for (y, row) in forest.iter().enumerate() {
        for (x, &tr) in row.iter().enumerate() {
            if tr > 1 {
                trees.push((Reverse(tr), x, y));
            }
        }
    }
    let mut res = 0;
    let mut start = (0, 0);
    while let Some((Reverse(_), x, y)) = trees.pop() {
        let Some(steps) = bfs(forest, start, (x, y)) else {
            return -1;
        };
        res += steps;
        start = (x, y);
    }
    res
}

fn bfs(forest: &[&[i32]], start: Coord, goal: Coord) -> Option<i32> {
    let mut queue = VecDeque::from([(start, 0)]);
    let mut seen = HashSet::new();
    while let Some((curr, dist)) = queue.pop_front() {
        if curr == goal {
            return Some(dist);
        }
        for (x, y) in neighbors(curr) {
            if forest
                .get(y)
                .is_some_and(|r| r.get(x).is_some_and(|&tr| tr > 0))
                && seen.insert((x, y))
            {
                queue.push_back(((x, y), dist + 1));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(cut_off_tree(&[&[1, 2, 3], &[0, 0, 4], &[7, 6, 5]]), 6);
        debug_assert_eq!(cut_off_tree(&[&[1, 2, 3], &[0, 0, 0], &[7, 6, 5]]), -1);
        debug_assert_eq!(cut_off_tree(&[&[2, 3, 4], &[0, 0, 5], &[8, 7, 6]]), 6);
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
