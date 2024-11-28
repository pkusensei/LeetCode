mod dsu;
mod helper;
mod trie;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

#[allow(unused_imports)]
use helper::*;

pub fn minimum_obstacles(grid: &[&[i32]]) -> i32 {
    let (rows, cols) = get_dimensions(grid);
    let mut dists = vec![vec![i32::MAX; cols]; rows];
    dists[0][0] = grid[0][0];
    // dijkstra(grid, cols, rows, &mut dists)
    bfs(grid, cols, rows, &mut dists)
}

fn dijkstra(grid: &[&[i32]], cols: usize, rows: usize, dists: &mut [Vec<i32>]) -> i32 {
    let mut heap = BinaryHeap::from([(Reverse(grid[0][0]), 0, 0)]);
    while let Some((Reverse(cost), x, y)) = heap.pop() {
        if x == cols - 1 && y == rows - 1 {
            return cost;
        }
        if cost > dists[y][x] {
            continue;
        }
        for (nx, ny) in neighbors((x, y)) {
            if let Some(v) = grid.get(ny).and_then(|r| r.get(nx)) {
                let nc = cost + v;
                if nc < dists[ny][nx] {
                    heap.push((Reverse(nc), nx, ny));
                    dists[ny][nx] = nc;
                }
            }
        }
    }
    -1
}

fn bfs(grid: &[&[i32]], cols: usize, rows: usize, dists: &mut [Vec<i32>]) -> i32 {
    let mut queue = VecDeque::from([(grid[0][0], 0, 0)]);
    while let Some((dist, x, y)) = queue.pop_front() {
        for (nx, ny) in neighbors((x, y)) {
            if let Some(&v) = grid.get(ny).and_then(|r| r.get(nx)) {
                if dists[ny][nx] < i32::MAX {
                    continue;
                }
                if v == 1 {
                    dists[ny][nx] = 1 + dist;
                    queue.push_back((1 + dist, nx, ny));
                } else {
                    dists[ny][nx] = dist;
                    queue.push_front((dist, nx, ny));
                }
            }
        }
    }
    dists[rows - 1][cols - 1]
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(minimum_obstacles(&[&[0, 1, 1], &[1, 1, 0], &[1, 1, 0]]), 2);
        debug_assert_eq!(
            minimum_obstacles(&[&[0, 1, 0, 0, 0], &[0, 1, 0, 1, 0], &[0, 0, 0, 1, 0]]),
            0
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

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
