mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn num_rook_captures(board: [[char; 8]; 8]) -> i32 {
    let mut start = [0, 0];
    for (y, row) in board.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            if ch == 'R' {
                start = [x, y];
            }
        }
    }
    let [x, y] = start;
    let mut res = 0;
    for i in (0..y).rev() {
        if board[i][x] == 'p' {
            res += 1;
            break;
        } else if board[i][x] == 'B' {
            break;
        }
    }
    for i in y..8 {
        if board[i][x] == 'p' {
            res += 1;
            break;
        } else if board[i][x] == 'B' {
            break;
        }
    }
    for i in (0..x).rev() {
        if board[y][i] == 'p' {
            res += 1;
            break;
        } else if board[y][i] == 'B' {
            break;
        }
    }
    for i in x..8 {
        if board[y][i] == 'p' {
            res += 1;
            break;
        } else if board[y][i] == 'B' {
            break;
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
            num_rook_captures([
                ['.', '.', '.', '.', '.', '.', '.', '.'],
                ['.', '.', '.', 'p', '.', '.', '.', '.'],
                ['.', '.', '.', 'R', '.', '.', '.', 'p'],
                ['.', '.', '.', '.', '.', '.', '.', '.'],
                ['.', '.', '.', '.', '.', '.', '.', '.'],
                ['.', '.', '.', 'p', '.', '.', '.', '.'],
                ['.', '.', '.', '.', '.', '.', '.', '.'],
                ['.', '.', '.', '.', '.', '.', '.', '.']
            ]),
            3
        );
        debug_assert_eq!(
            num_rook_captures([
                ['.', '.', '.', '.', '.', '.', '.', '.'],
                ['.', 'p', 'p', 'p', 'p', 'p', '.', '.'],
                ['.', 'p', 'p', 'B', 'p', 'p', '.', '.'],
                ['.', 'p', 'B', 'R', 'B', 'p', '.', '.'],
                ['.', 'p', 'p', 'B', 'p', 'p', '.', '.'],
                ['.', 'p', 'p', 'p', 'p', 'p', '.', '.'],
                ['.', '.', '.', '.', '.', '.', '.', '.'],
                ['.', '.', '.', '.', '.', '.', '.', '.']
            ]),
            0
        );
        debug_assert_eq!(
            num_rook_captures([
                ['.', '.', '.', '.', '.', '.', '.', '.'],
                ['.', '.', '.', 'p', '.', '.', '.', '.'],
                ['.', '.', '.', 'p', '.', '.', '.', '.'],
                ['p', 'p', '.', 'R', '.', 'p', 'B', '.'],
                ['.', '.', '.', '.', '.', '.', '.', '.'],
                ['.', '.', '.', 'B', '.', '.', '.', '.'],
                ['.', '.', '.', 'p', '.', '.', '.', '.'],
                ['.', '.', '.', '.', '.', '.', '.', '.']
            ]),
            3
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
