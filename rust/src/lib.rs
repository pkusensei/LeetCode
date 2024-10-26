mod helper;
mod trie;

use std::collections::{HashMap, HashSet, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn largest_island(grid: &[&[i32]]) -> i32 {
    let mut seen = HashSet::new();
    let mut fronts: HashMap<Coord, Vec<i32>> = HashMap::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, &n) in row.iter().enumerate() {
            if n == 1 && seen.insert((x, y)) {
                let (island, front) = bfs(grid, (x, y), &mut seen);
                for c in front {
                    fronts.entry(c).or_default().push(island);
                }
            }
        }
    }
    let n = grid.len();
    if seen.len() == n * n {
        return (n * n) as i32;
    }
    1 + fronts.values().map(|v| v.iter().sum()).max().unwrap_or(0)
}

fn bfs(grid: &[&[i32]], (x, y): Coord, seen: &mut HashSet<Coord>) -> (i32, HashSet<Coord>) {
    let mut front = HashSet::new();
    let mut queue = VecDeque::from([(x, y)]);
    seen.insert((x, y));
    let mut count = 0;
    while let Some((x, y)) = queue.pop_front() {
        count += 1;
        for (nx, ny) in neighbors((x, y)) {
            match grid.get(ny).and_then(|r| r.get(nx)) {
                Some(&1) if seen.insert((nx, ny)) => queue.push_back((nx, ny)),
                Some(&0) => {
                    front.insert((nx, ny));
                }
                _ => (),
            }
        }
    }
    (count, front)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(largest_island(&[&[1, 0], &[0, 1]]), 3);
        debug_assert_eq!(largest_island(&[&[1, 1], &[1, 0]]), 4);
        debug_assert_eq!(largest_island(&[&[1, 1], &[1, 1]]), 4);
    }

    #[test]
    fn test() {
        debug_assert_eq!(
            largest_island(&[
                &[0, 0, 0, 0, 0, 0, 0],
                &[0, 1, 1, 1, 1, 0, 0],
                &[0, 1, 0, 0, 1, 0, 0],
                &[1, 0, 1, 0, 1, 0, 0],
                &[0, 1, 0, 0, 1, 0, 0],
                &[0, 1, 0, 0, 1, 0, 0],
                &[0, 1, 1, 1, 1, 0, 0]
            ]),
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
}
