mod helper;
mod trie;

use std::collections::{HashSet, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn contain_virus(grid: &mut [&mut [i32]]) -> i32 {
    let mut res = 0;
    loop {
        let mut seen = HashSet::new();
        let mut blocks = vec![];
        for (y, row) in grid.iter().enumerate() {
            for (x, &v) in row.iter().enumerate() {
                if v == 1 && seen.insert((x, y)) {
                    let block = bfs(grid, x, y);
                    seen.extend(block.inside.iter());
                    blocks.push(block);
                }
            }
        }
        let Some(b) = blocks.iter().max_by_key(|b| b.front.len()) else {
            break;
        };
        if b.front.is_empty() {
            break;
        }
        res += b.side;
        for &(x, y) in b.inside.iter() {
            grid[y][x] = -1;
        }
        for block in blocks
            .iter()
            .filter(|block| block.front.len() < b.front.len())
        {
            for &(x, y) in block.front.iter() {
                grid[y][x] = 1;
            }
        }
    }
    res
}

fn bfs<T: AsRef<[i32]>>(grid: &[T], x: usize, y: usize) -> Block {
    let mut queue = VecDeque::from([(x, y)]);
    let (mut inside, mut front) = (HashSet::from([(x, y)]), HashSet::new());
    let mut side = 0;
    while let Some((x, y)) = queue.pop_front() {
        for (nx, ny) in neighbors((x, y)) {
            if let Some(&v) = grid.get(ny).and_then(|r| r.as_ref().get(nx)) {
                if v == 1 && inside.insert((nx, ny)) {
                    queue.push_back((nx, ny));
                } else if v == 0 {
                    side += 1;
                    front.insert((nx, ny));
                }
            }
        }
    }
    Block {
        inside,
        front,
        side,
    }
}

#[derive(Debug, Clone)]
struct Block {
    inside: HashSet<Coord>,
    front: HashSet<Coord>,
    side: i32,
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            contain_virus(&mut [
                &mut [0, 1, 0, 0, 0, 0, 0, 1],
                &mut [0, 1, 0, 0, 0, 0, 0, 1],
                &mut [0, 0, 0, 0, 0, 0, 0, 1],
                &mut [0, 0, 0, 0, 0, 0, 0, 0]
            ]),
            10
        );
        debug_assert_eq!(
            contain_virus(&mut [&mut [1, 1, 1], &mut [1, 0, 1], &mut [1, 1, 1]]),
            4
        );
        debug_assert_eq!(
            contain_virus(&mut [
                &mut [1, 1, 1, 0, 0, 0, 0, 0, 0],
                &mut [1, 0, 1, 0, 1, 1, 1, 1, 1],
                &mut [1, 1, 1, 0, 0, 0, 0, 0, 0]
            ]),
            13
        );
    }

    #[test]
    fn test() {
        debug_assert_eq!(
            contain_virus(&mut [
                &mut [0, 1, 0, 1, 1, 1, 1, 1, 1, 0],
                &mut [0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
                &mut [0, 0, 1, 1, 1, 0, 0, 0, 1, 0],
                &mut [0, 0, 0, 1, 1, 0, 0, 1, 1, 0],
                &mut [0, 1, 0, 0, 1, 0, 1, 1, 0, 1],
                &mut [0, 0, 0, 1, 0, 1, 0, 1, 1, 1],
                &mut [0, 1, 0, 0, 1, 0, 0, 1, 1, 0],
                &mut [0, 1, 0, 1, 0, 0, 0, 1, 1, 0],
                &mut [0, 1, 1, 0, 0, 1, 1, 0, 0, 1],
                &mut [1, 0, 1, 1, 0, 1, 0, 1, 0, 1]
            ]),
            38
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
