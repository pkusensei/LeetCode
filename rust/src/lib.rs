mod helper;

use std::collections::{HashSet, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn count_battleships(board: &[&[char]]) -> i32 {
    let mut seen = HashSet::new();
    let mut res = 0;
    for (y, r) in board.iter().enumerate() {
        for (x, &ch) in r.iter().enumerate() {
            if ch == 'X' && bfs(board, (x, y), &mut seen) {
                res += 1
            }
        }
    }
    res
}

fn bfs(board: &[&[char]], curr: Coord, seen: &mut HashSet<Coord>) -> bool {
    if seen.contains(&curr) {
        return false;
    }
    let mut queue = VecDeque::from([curr]);
    while let Some(c) = queue.pop_front() {
        if !seen.insert(c) {
            continue;
        }
        queue.extend(neighbors(c).filter(|&(x, y)| {
            board
                .get(y)
                .is_some_and(|r| r.get(x).is_some_and(|&ch| ch == 'X'))
        }));
    }
    true
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            count_battleships(&[
                &['X', '.', '.', 'X'],
                &['.', '.', '.', 'X'],
                &['.', '.', '.', 'X']
            ]),
            2
        );
        debug_assert_eq!(count_battleships(&[&['.']]), 0);
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
