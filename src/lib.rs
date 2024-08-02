pub fn total_n_queens(n: i32) -> i32 {
    match n {
        1 => 1,
        4..=9 => solve(n as _, 0, vec![]),
        _ => 0,
    }
}

fn solve(n: usize, row: usize, board: Vec<usize>) -> i32 {
    // board: Vec<usize>
    // index as row, board[row] == col
    if row == n {
        return 1;
    }

    (0..n)
        .filter_map(|col| {
            if check(&board, row, col) {
                let mut temp = board.clone();
                temp.push(col);
                Some(solve(n, row + 1, temp))
            } else {
                None
            }
        })
        .sum()
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
        debug_assert_eq!(total_n_queens(4), 2);
        debug_assert_eq!(total_n_queens(1), 1);
    }

    #[test]
    fn test() {}
}
