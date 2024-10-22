mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn valid_tic_tac_toe(board: &[&str]) -> bool {
        let board: Vec<&[u8]> = board.iter().map(|s| s.as_bytes()).collect();
        let (xcount, ocount) =
            board
                .iter()
                .flat_map(|row| row.iter())
                .fold((0, 0), |(xcount, ocount), &b| {
                    if b == b'X' {
                        (1 + xcount, ocount)
                    } else if b == b'O' {
                        (xcount, 1 + ocount)
                    } else {
                        (xcount, ocount)
                    }
                });
        if !(0..=1).contains(&(xcount - ocount)) {
            return false;
        }
        if check(&board, b'X') && xcount != 1 + ocount {
            return false;
        }
        if check(&board, b'O') && xcount != ocount {
            return false;
        }
        true
}

fn check(board: &[&[u8]], byte: u8) -> bool {
    if board.iter().any(|s| s.iter().all(|&b| b == byte)) {
        return true;
    }
    if (0..3)
        .map(|i| [board[0][i], board[1][i], board[2][i]])
        .any(|col| col.into_iter().all(|b| byte == b))
    {
        return true;
    }
    if (0..3).map(|i| board[i][i]).all(|b| byte == b) {
        return true;
    }
    if (0..3).map(|i| board[i][2 - i]).all(|b| byte == b) {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(!valid_tic_tac_toe(&["O  ", "   ", "   "]));
        debug_assert!(!valid_tic_tac_toe(&["XOX", " X ", "   "]));
        debug_assert!(valid_tic_tac_toe(&["XOX", "O O", "XOX"]));
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
