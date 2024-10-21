mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn moves_to_chessboard(board: &[&[i32]]) -> i32 {
    let n = board.len();
    for row in board.iter().skip(1) {
        for (x, &num) in row.iter().enumerate().skip(1) {
            // For any row, it must satisfy
            // row0[x] == row[x] or row[0][x] != row[x] for all x's
            // 1100 == 1100 || 1100 != 0011
            if (board[0][0] ^ row[0]) ^ (board[0][x] ^ num) == 1 {
                return -1;
            }
        }
    }
    let (mut row_ones, mut col_ones) = (0, 0);
    let (mut row_moves, mut col_moves) = (0, 0);
    for idx in 0..n {
        row_ones += board[0][idx]; // first row
        col_ones += board[idx][0]; // first col
        if board[idx][0] == idx as i32 & 1 {
            row_moves += 1;
        }
        if board[0][idx] == idx as i32 & 1 {
            col_moves += 1;
        }
    }
    let n = n as i32;
    if row_ones < n / 2 || col_ones < n / 2 || row_ones > (1 + n) / 2 || col_ones > (1 + n) / 2 {
        return -1;
    }
    if n & 1 == 1 {
        // number of moves has to be even
        if row_moves & 1 == 1 {
            row_moves = n - row_moves
        }
        if col_moves & 1 == 1 {
            col_moves = n - col_moves
        }
    } else {
        row_moves = row_moves.min(n - row_moves);
        col_moves = col_moves.min(n - col_moves);
    }
    (row_moves + col_moves) / 2
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            moves_to_chessboard(&[&[0, 1, 1, 0], &[0, 1, 1, 0], &[1, 0, 0, 1], &[1, 0, 0, 1]]),
            2
        );
        debug_assert_eq!(moves_to_chessboard(&[&[0, 1], &[1, 0]]), 0);
        debug_assert_eq!(moves_to_chessboard(&[&[1, 0], &[1, 0]]), -1);
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
