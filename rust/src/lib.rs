mod helper;
mod trie;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

#[allow(unused_imports)]
use helper::*;

pub fn swim_in_water(grid: &[&[i32]]) -> i32 {
    let n = grid.len();
    let mut heap = BinaryHeap::from([(Reverse(grid[0][0]), 0, 0)]);
    let mut seen = HashSet::from([(0, 0)]);
    let mut res = 0;
    while let Some((Reverse(num), x, y)) = heap.pop() {
        res = res.max(num);
        if (n - 1, n - 1) == (x, y) {
            break;
        }
        for (nx, ny) in neighbors((x, y)) {
            if let Some(&h) = grid.get(ny).and_then(|r| r.get(nx)) {
                if seen.insert((nx, ny)) {
                    heap.push((Reverse(h), nx, ny));
                }
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(swim_in_water(&[&[0, 2], &[1, 3]]), 3);
        debug_assert_eq!(
            swim_in_water(&[
                &[0, 1, 2, 3, 4],
                &[24, 23, 22, 21, 5],
                &[12, 13, 14, 15, 16],
                &[11, 17, 18, 19, 20],
                &[10, 9, 8, 7, 6]
            ]),
            16
        );
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
