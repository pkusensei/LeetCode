pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    solve(board);
}

fn solve(board: &mut Vec<Vec<char>>) -> bool {
    for row in 0..9 {
        for col in 0..9 {
            if board[row][col] == '.' {
                for num in '1'..='9' {
                    if check_num(board, num, row, col) {
                        board[row][col] = num;
                        if solve(board) {
                            return true;
                        } else {
                            board[row][col] = '.'
                        }
                    }
                }
                return false;
            }
        }
    }
    true
}

fn check_num(board: &[Vec<char>], num: char, row: usize, col: usize) -> bool {
    if (0..9).any(|x| board[row][x] == num) {
        return false;
    }
    if board.iter().any(|row| row[col] == num) {
        return false;
    }
    let start_row = row - row % 3;
    let start_col = col - col % 3;
    for i in 0..3 {
        for j in 0..3 {
            if board[i + start_row][j + start_col] == num {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let mut board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        solve_sudoku(&mut board);
        debug_assert_eq!(
            board,
            [
                ['5', '3', '4', '6', '7', '8', '9', '1', '2'],
                ['6', '7', '2', '1', '9', '5', '3', '4', '8'],
                ['1', '9', '8', '3', '4', '2', '5', '6', '7'],
                ['8', '5', '9', '7', '6', '1', '4', '2', '3'],
                ['4', '2', '6', '8', '5', '3', '7', '9', '1'],
                ['7', '1', '3', '9', '2', '4', '8', '5', '6'],
                ['9', '6', '1', '5', '3', '7', '2', '8', '4'],
                ['2', '8', '7', '4', '1', '9', '6', '3', '5'],
                ['3', '4', '5', '2', '8', '6', '1', '7', '9']
            ]
        );
    }

    #[test]
    fn test() {}
}
