mod dsu;
mod helper;
mod trie;

use std::collections::VecDeque;

#[allow(unused_imports)]
use helper::*;

pub fn snakes_and_ladders(board: &mut [&[i32]]) -> i32 {
    let n = board.len();
    board.reverse();
    let mut queue = VecDeque::from([(0, 0, 0)]);
    let mut seen = vec![vec![false; n]; n];
    seen[0][0] = true;
    while let Some((x, y, dist)) = queue.pop_front() {
        if coord_to_label(n, x, y) == n * n {
            return dist;
        }
        for (nx, ny) in next(board, x, y) {
            if !seen[ny][nx] {
                seen[ny][nx] = true;
                queue.push_back((nx, ny, 1 + dist));
            }
        }
    }
    -1
}

fn next<'a>(board: &'a [&[i32]], x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> + 'a {
    let n = board.len();
    let label = coord_to_label(n, x, y);
    (1 + label..=(label + 6).min(n * n)).map(move |node| {
        let (nx, ny) = label_to_coord(n, node);
        if board[ny][nx] == -1 {
            (nx, ny)
        } else {
            label_to_coord(n, board[ny][nx] as usize)
        }
    })
}

const fn coord_to_label(n: usize, x: usize, y: usize) -> usize {
    if y & 1 == 0 {
        n * y + x + 1
    } else {
        n * (1 + y) - x
    }
}

const fn label_to_coord(n: usize, label: usize) -> (usize, usize) {
    let ny = (label - 1) / n;
    let nx = if ny & 1 == 0 {
        (label - 1) % n
    } else {
        n - 1 - (label - 1) % n
    };
    (nx, ny)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            snakes_and_ladders(&mut [
                &[-1, -1, -1, -1, -1, -1],
                &[-1, -1, -1, -1, -1, -1],
                &[-1, -1, -1, -1, -1, -1],
                &[-1, 35, -1, -1, 13, -1],
                &[-1, -1, -1, -1, -1, -1],
                &[-1, 15, -1, -1, -1, -1]
            ]),
            4
        );
        debug_assert_eq!(snakes_and_ladders(&mut [&[-1, -1], &[-1, 3]]), 1);
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
        T1: Into<f64>,
        T2: Into<f64>,
    {
        const EP: f64 = 1e-5;
        debug_assert!((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP);
    }
}
