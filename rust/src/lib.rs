mod helper;
mod trie;

use std::collections::{HashSet, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn update_board(mut board: Vec<Vec<char>>, click: [i32; 2]) -> Vec<Vec<char>> {
    let [row, col] = click;
    if board[row as usize][col as usize] == 'M' {
        // 'M' or 'E'
        board[row as usize][col as usize] = 'X';
        return board;
    }
    let mut mines = HashSet::new();
    for (y, r) in board.iter().enumerate() {
        for (x, &ch) in r.iter().enumerate() {
            if ch == 'M' {
                mines.insert([y, x]);
            }
        }
    }
    let (rows, cols) = get_dimensions(&board);
    let mut queue = VecDeque::from([(row, col)]);
    while let Some((c_row, c_col)) = queue.pop_front() {
        if board[c_row as usize][c_col as usize] != 'E' {
            continue;
        }
        let neighbors: Vec<_> = around(c_row, c_col)
            .filter(|(r, c)| *r < rows && *c < cols)
            .collect();
        let count = neighbors
            .iter()
            .filter(|(r, c)| mines.contains(&[*r, *c]))
            .count();
        if count > 0 {
            board[c_row as usize][c_col as usize] = char::from(count as u8 + b'0');
        } else {
            board[c_row as usize][c_col as usize] = 'B';
            queue.extend(neighbors.into_iter().map(|(a, b)| (a as i32, b as i32)));
        }
    }
    board
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            update_board(
                vec![
                    vec!['E', 'E', 'E', 'E', 'E'],
                    vec!['E', 'E', 'M', 'E', 'E'],
                    vec!['E', 'E', 'E', 'E', 'E'],
                    vec!['E', 'E', 'E', 'E', 'E']
                ],
                [3, 0]
            ),
            [
                ['B', '1', 'E', '1', 'B'],
                ['B', '1', 'M', '1', 'B'],
                ['B', '1', '1', '1', 'B'],
                ['B', 'B', 'B', 'B', 'B']
            ]
        );
        debug_assert_eq!(
            update_board(
                vec![
                    vec!['B', '1', 'E', '1', 'B'],
                    vec!['B', '1', 'M', '1', 'B'],
                    vec!['B', '1', '1', '1', 'B'],
                    vec!['B', 'B', 'B', 'B', 'B']
                ],
                [1, 2]
            ),
            [
                ['B', '1', 'E', '1', 'B'],
                ['B', '1', 'X', '1', 'B'],
                ['B', '1', '1', '1', 'B'],
                ['B', 'B', 'B', 'B', 'B']
            ]
        )
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
