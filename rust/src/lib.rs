mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn game_of_life(board: &mut [Vec<i32>]) {
    let (rows, cols) = get_dimensions(board);
    for y in 0..rows {
        for x in 0..cols {
            dbg!(around(x as i32, y as i32).count());
            let count = around(x as i32, y as i32)
                .filter(|&(c, r)| {
                    board
                        .get(r)
                        .is_some_and(|row| row.get(c).is_some_and(|&c| c & 1 == 1))
                })
                .count();
            match (board[y][x] & 1 == 1, count) {
                (true, 0..2) => (),
                (true, 2 | 3) => board[y][x] += 2,
                (true, 4..) => (),
                (false, 3) => board[y][x] += 2,
                _ => (),
            }
        }
    }
    for row in board.iter_mut() {
        for cell in row.iter_mut() {
            *cell /= 2
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        {
            let mut board = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
            game_of_life(&mut board);
            debug_assert_eq!(board, [[0, 0, 0], [1, 0, 1], [0, 1, 1], [0, 1, 0]]);
        }
        {
            let mut board = vec![vec![1, 1], vec![1, 0]];
            game_of_life(&mut board);
            debug_assert_eq!(board, [[1, 1], [1, 1]]);
        }
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(i1: I1, i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: IntoIterator<Item = T1>,
        I2: IntoIterator<Item = T2>,
    {
        let mut v1: Vec<_> = i1.into_iter().collect();
        let mut v2: Vec<_> = i2.into_iter().collect();
        v1.sort_unstable();
        v2.sort_unstable();
        debug_assert_eq!(v1, v2);
    }
}
