mod helper;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

#[allow(unused_imports)]
use helper::*;

pub fn trap_rain_water(height_map: &[&[i32]]) -> i32 {
        let (rows, cols) = get_dimensions(height_map);
        if rows < 3 || cols < 3 {
            return 0;
        }
        let mut heap = BinaryHeap::new();
        let mut seen = HashSet::new();
        for (y, r) in height_map.iter().enumerate() {
            for (x, &height) in r.iter().enumerate() {
                if x == 0 || y == 0 || x == cols - 1 || y == rows - 1 {
                    heap.push((Reverse(height), x, y));
                    seen.insert((x, y));
                }
            }
        }

        let mut height = 0;
        let mut res = 0;
        // min-heap
        // so that it always the __lowest__ blocking cell
        // once a blocking cell finds a neighbor with a smaller height
        // that neighbor can store water of (blocking-neighbor)
        while let Some((Reverse(h), x, y)) = heap.pop() {
            height = height.max(h);
            for (nx, ny) in neighbors((x, y)) {
                let Some(&nh) = height_map.get(ny).and_then(|r| r.get(nx)) else {
                    continue;
                };
                if !seen.insert((nx, ny)) {
                    continue;
                }
                heap.push((Reverse(nh), nx, ny));
                if nh < height {
                    res += height - nh;
                }
            }
        }
        res
}

#[cfg(test)]
mod tests {
    use std::{fmt::Debug, ops::DerefMut};

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            trap_rain_water(&[
                &[1, 4, 3, 1, 3, 2],
                &[3, 2, 1, 3, 2, 4],
                &[2, 3, 3, 2, 3, 1]
            ]),
            4
        );
        debug_assert_eq!(
            trap_rain_water(&[
                &[3, 3, 3, 3, 3],
                &[3, 2, 2, 2, 3],
                &[3, 2, 1, 2, 3],
                &[3, 2, 2, 2, 3],
                &[3, 3, 3, 3, 3]
            ]),
            10
        );
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: DerefMut<Target = [T1]>,
        I2: DerefMut<Target = [T2]>,
    {
        i1.sort_unstable();
        i2.sort_unstable();
        debug_assert_eq!(*i1, *i2);
    }
}
