use std::collections::{HashSet, VecDeque};

pub fn solve(board: &mut [Vec<char>]) {
    let row = board.len();
    let col = board.first().map(|r| r.len()).unwrap_or_default();
    if row * col == 0 {
        return;
    }

    let mut region = HashSet::new();
    for (y, r) in board.iter().enumerate() {
        for (x, &ch) in r.iter().enumerate() {
            if ch == 'O' && (x == 0 || x == col - 1 || y == 0 || y == row - 1) {
                region.extend(bfs(board, (y, x)));
            }
        }
    }
    for r in board.iter_mut() {
        for c in r.iter_mut() {
            *c = 'X';
        }
    }
    for (row, col) in region {
        board[row][col] = 'O'
    }
}

fn bfs(board: &[Vec<char>], curr: Coord) -> HashSet<Coord> {
    let mut seen = HashSet::from([curr]);
    let mut queue = VecDeque::from([curr]);
    while let Some(coord) = queue.pop_front() {
        for neighbor in neighbors(coord).filter(|&(row, col)| {
            board
                .get(row)
                .is_some_and(|r| r.get(col).is_some_and(|&c| c == 'O'))
        }) {
            if seen.insert(neighbor) {
                queue.push_back(neighbor);
            }
        }
    }
    seen
}

type Coord = (usize, usize);

fn neighbors((a, b): Coord) -> impl Iterator<Item = Coord> {
    [
        (a.saturating_sub(1), b),
        (a + 1, b),
        (a, b.saturating_sub(1)),
        (a, b + 1),
    ]
    .into_iter()
    .filter(move |&p| p != (a, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        {
            let mut board = vec![
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'O', 'O', 'X'],
                vec!['X', 'X', 'O', 'X'],
                vec!['X', 'O', 'X', 'X'],
            ];
            solve(&mut board);
            debug_assert_eq!(
                board,
                [
                    ['X', 'X', 'X', 'X'],
                    ['X', 'X', 'X', 'X'],
                    ['X', 'X', 'X', 'X'],
                    ['X', 'O', 'X', 'X'],
                ]
            );
        }
        {
            let mut board = vec![vec!['X']];
            solve(&mut board);
            debug_assert_eq!(board, [['X']]);
        }
    }

    #[test]
    fn test() {
        {
            let mut board = vec![
                vec!['X', 'O', 'X', 'O', 'X', 'O'],
                vec!['O', 'X', 'O', 'X', 'O', 'X'],
                vec!['X', 'O', 'X', 'O', 'X', 'O'],
                vec!['O', 'X', 'O', 'X', 'O', 'X'],
            ];
            solve(&mut board);
            debug_assert_eq!(
                board,
                [
                    ['X', 'O', 'X', 'O', 'X', 'O'],
                    ['O', 'X', 'X', 'X', 'X', 'X'],
                    ['X', 'X', 'X', 'X', 'X', 'O'],
                    ['O', 'X', 'O', 'X', 'O', 'X']
                ]
            )
        }
    }
}
