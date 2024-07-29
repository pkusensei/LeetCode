use std::collections::HashMap;

pub fn is_valid_sudoku(board: &[&[char]; 9]) -> bool {
    for r in board {
        let row: HashMap<char, i32> =
            r.iter()
                .filter(|ch| ch.is_ascii_digit())
                .fold(HashMap::new(), |mut acc, &ch| {
                    *acc.entry(ch).or_insert(0) += 1;
                    acc
                });
        if row.values().any(|&v| v > 1) {
            return false;
        }
    }
    for x in 0..9 {
        let col = (0..9)
            .map(|y| board[y][x])
            .fold(HashMap::new(), |mut acc, ch| {
                if ch.is_ascii_digit() {
                    *acc.entry(ch).or_insert(0) += 1;
                }
                acc
            });
        if col.values().any(|&v| v > 1) {
            return false;
        }
    }

    check_three_rows(board, &[0, 1, 2])
        && check_three_rows(board, &[3, 4, 5])
        && check_three_rows(board, &[6, 7, 8])
}

fn check_three_rows(board: &[&[char]], ys: &[usize; 3]) -> bool {
    check_square(board, &[0, 1, 2], ys)
        && check_square(board, &[3, 4, 5], ys)
        && check_square(board, &[6, 7, 8], ys)
}

fn check_square(board: &[&[char]], xs: &[usize; 3], ys: &[usize; 3]) -> bool {
    let mut res = HashMap::new();
    for &y in ys {
        for &x in xs {
            let ch = board[y][x];
            if ch.is_ascii_digit() {
                *res.entry(ch).or_insert(0) += 1
            }
        }
    }
    res.values().all(|&v| v == 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            is_valid_sudoku(&[
                &['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                &['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                &['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                &['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                &['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                &['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                &['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                &['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                &['.', '.', '.', '.', '8', '.', '.', '7', '9']
            ]),
            true
        );
        debug_assert_eq!(
            is_valid_sudoku(&[
                &['8', '3', '.', '.', '7', '.', '.', '.', '.'],
                &['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                &['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                &['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                &['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                &['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                &['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                &['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                &['.', '.', '.', '.', '8', '.', '.', '7', '9']
            ]),
            false
        );
    }

    #[test]
    fn test() {}
}
