mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn tictactoe(moves: &[[i32; 2]]) -> String {
    let mut board = [[0; 3]; 3];
    let mut p = 1;
    for m in moves {
        board[m[0] as usize][m[1] as usize] = p;
        p = -p;
    }
    let mut win = false;
    win |= board.iter().any(|r| r.iter().sum::<i32>().abs() == 3);
    win |= (0..3).any(|col| (0..3).map(|row| board[row][col]).sum::<i32>().abs() == 3);
    win |= (board[0][0] + board[1][1] + board[2][2]).abs() == 3;
    win |= (board[2][0] + board[1][1] + board[0][2]).abs() == 3;
    if win {
        if moves.len() & 1 == 1 {
            "A".into()
        } else {
            "B".into()
        }
    } else if moves.len() == 9 {
        "Draw".into()
    } else {
        "Pending".into()
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(tictactoe(&[[0, 0], [2, 0], [1, 1], [2, 1], [2, 2]]), "A");
        assert_eq!(
            tictactoe(&[[0, 0], [1, 1], [0, 1], [0, 2], [1, 0], [2, 0]]),
            "B"
        );
        assert_eq!(
            tictactoe(&[
                [0, 0],
                [1, 1],
                [2, 0],
                [1, 0],
                [1, 2],
                [2, 1],
                [0, 1],
                [0, 2],
                [2, 2]
            ]),
            "Draw"
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
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
