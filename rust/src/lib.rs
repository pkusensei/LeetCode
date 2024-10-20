mod helper;
mod trie;

use std::collections::{HashSet, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn sliding_puzzle(board: [[i32; 3]; 2]) -> i32 {
    const TARGET: [[u8; 3]; 2] = [[1, 2, 3], [4, 5, 0]];
    let mut b = [[0u8; 3]; 2];
    let (mut x, mut y) = (0, 0);
    for (by, row) in b.iter_mut().enumerate() {
        for (bx, n) in row.iter_mut().enumerate() {
            *n = board[by][bx] as u8;
            if 0 == *n {
                (x, y) = (bx, by);
            }
        }
    }
    let mut queue = VecDeque::from([(b, x, y, 0)]);
    let mut seen = HashSet::from([b]);
    while let Some((curr, x, y, dist)) = queue.pop_front() {
        if TARGET == curr {
            return dist;
        }
        for (nb, (nx, ny)) in next(curr, x, y) {
            if seen.insert(nb) {
                queue.push_back((nb, nx, ny, 1 + dist));
            }
        }
    }
    -1
}

fn next(board: [[u8; 3]; 2], x: usize, y: usize) -> impl Iterator<Item = ([[u8; 3]; 2], Coord)> {
    neighbors((x, y)).filter_map(move |(nx, ny)| {
        let mut b = board;
        if let Some(v) = b.get_mut(ny).and_then(|r| r.get_mut(nx)) {
            let temp = *v;
            *v = 0;
            b[y][x] = temp;
            Some((b, (nx, ny)))
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(sliding_puzzle([[1, 2, 3], [4, 0, 5]]), 1);
        debug_assert_eq!(sliding_puzzle([[1, 2, 3], [5, 4, 0]]), -1);
        debug_assert_eq!(sliding_puzzle([[4, 1, 2], [5, 0, 3]]), 5);
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
