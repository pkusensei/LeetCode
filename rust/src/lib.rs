mod helper;
mod trie;

use std::collections::{HashSet, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn max_area_of_island(grid: &[&[i32]]) -> i32 {
    let mut res = 0;
    let mut seen = HashSet::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, &num) in row.iter().enumerate() {
            if num == 1 && !seen.contains(&(x, y)) {
                res = res.max(bfs(grid, x, y, &mut seen));
            }
        }
    }
    res
}

fn bfs(grid: &[&[i32]], x: usize, y: usize, seen: &mut HashSet<(usize, usize)>) -> i32 {
    let mut res = 0;
    seen.insert((x, y));
    let mut queue = VecDeque::from([(x, y)]);
    while let Some((x, y)) = queue.pop_front() {
        res += 1;
        for (nx, ny) in neighbors((x, y)).filter(|&(nx, ny)| {
            grid.get(ny)
                .is_some_and(|r| r.get(nx).is_some_and(|&num| num == 1))
        }) {
            if seen.insert((nx, ny)) {
                queue.push_back((nx, ny));
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
        debug_assert_eq!(
            max_area_of_island(&[
                &[0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                &[0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                &[0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                &[0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
                &[0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
                &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
                &[0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                &[0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0]
            ]),
            6
        );
        debug_assert_eq!(max_area_of_island(&[&[0, 0, 0, 0, 0, 0, 0, 0]]), 0)
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
