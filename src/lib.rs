pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    match n {
        1 => vec![vec!["Q".to_string()]],
        4..=9 => {
            let boards = solve(n as _, 0, vec![]);
            boards
                .into_iter()
                .map(|board| {
                    board
                        .into_iter()
                        .map(|col| {
                            let mut s: Vec<_> = vec!['.'; n as _];
                            s[col] = 'Q';
                            s.into_iter().collect()
                        })
                        .collect()
                })
                .collect()
        }
        _ => vec![],
    }
}

fn solve(n: usize, row: usize, board: Vec<usize>) -> Vec<Vec<usize>> {
    // board: Vec<usize>
    // index as row, board[row] == col
    if row == n {
        return vec![board];
    }

    let mut res = vec![];
    for col in 0..n {
        if check(&board, row, col) {
            let mut temp = board.clone();
            temp.push(col);
            res.extend(solve(n, row + 1, temp))
        }
    }
    res
}

fn check(board: &[usize], row: usize, col: usize) -> bool {
    row == board.len()
        && board
            .iter()
            .copied()
            .enumerate()
            .all(|(r, c)| c != col && r.abs_diff(row) != c.abs_diff(col))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            solve_n_queens(4),
            [
                [".Q..", "...Q", "Q...", "..Q."],
                ["..Q.", "Q...", "...Q", ".Q.."]
            ]
        );
        debug_assert_eq!(solve_n_queens(1), [["Q"]]);
    }

    #[test]
    fn test() {}
}
