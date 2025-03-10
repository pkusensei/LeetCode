mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

use std::collections::VecDeque;

pub fn maximum_minutes(grid: &[&[i32]]) -> i32 {
    let (board, queue) = init(grid);
    let fire_board = spread(board.clone(), queue);
    let mut left = -1;
    let mut right = 1_000_000_000;
    while left < right {
        let mid = (right + 1 + left) / 2;
        if walk(&board, &fire_board, mid) {
            left = mid;
        } else {
            right = mid - 1;
        }
    }
    left
}

fn walk(board: &[Vec<i32>], fire_board: &[Vec<i32>], mid: i32) -> bool {
    let [rows, cols] = get_dimensions(board);
    let mut curr_board = board.to_vec();
    curr_board[0][0] = 0;
    let mut queue = VecDeque::from([([0, 0], mid)]);
    while let Some(([r, c], step)) = queue.pop_front() {
        if r == rows - 1 && c == cols - 1 && (fire_board[r][c] < 0 || step <= fire_board[r][c]) {
            return true;
        }
        if fire_board[r][c] > 0 && step >= fire_board[r][c] {
            continue;
        }
        let step_next = 1 + step;
        for [nr, nc] in neighbors([r, c]) {
            if curr_board
                .get(nr)
                .is_some_and(|row| row.get(nc).is_some_and(|&v| v == -1))
            {
                curr_board[nr][nc] = step_next;
                queue.push_back(([nr, nc], step_next));
            }
        }
    }
    false
}

fn spread(mut fire_board: Vec<Vec<i32>>, mut queue: VecDeque<[usize; 2]>) -> Vec<Vec<i32>> {
    let mut t = 0;
    while !queue.is_empty() {
        let _n = queue.len();
        t += 1;
        for _ in 0.._n {
            let [r, c] = queue.pop_front().unwrap();
            for [nr, nc] in neighbors([r, c]) {
                if fire_board
                    .get(nr)
                    .is_some_and(|row| row.get(nc).is_some_and(|&v| v == -1))
                {
                    fire_board[nr][nc] = t;
                    queue.push_back([nr, nc]);
                }
            }
        }
    }
    fire_board
}

fn init(grid: &[&[i32]]) -> (Vec<Vec<i32>>, VecDeque<[usize; 2]>) {
    let [rows, cols] = get_dimensions(grid);
    let mut board = vec![vec![-1; cols]; rows];
    let mut queue = VecDeque::new();
    for (r, row) in grid.iter().enumerate() {
        for (c, &v) in row.iter().enumerate() {
            match v {
                1 => {
                    board[r][c] = 0;
                    queue.push_back([r, c]);
                }
                2 => board[r][c] = -2,
                _ => (),
            }
        }
    }
    (board, queue)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {{
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        }};
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        }};
    }

    #[test]
    fn basics() {
        assert_eq!(
            maximum_minutes(&[
                &[0, 2, 0, 0, 0, 0, 0],
                &[0, 0, 0, 2, 2, 1, 0],
                &[0, 2, 0, 0, 1, 2, 0],
                &[0, 0, 2, 2, 2, 0, 2],
                &[0, 0, 0, 0, 0, 0, 0]
            ]),
            3
        );
        assert_eq!(
            maximum_minutes(&[&[0, 0, 0, 0], &[0, 1, 2, 0], &[0, 2, 0, 0]]),
            -1
        );
        assert_eq!(
            maximum_minutes(&[&[0, 0, 0], &[2, 2, 0], &[1, 2, 0]]),
            1_000_000_000
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            maximum_minutes(&[
                &[0, 2, 0, 0, 1],
                &[0, 2, 0, 2, 2],
                &[0, 2, 0, 0, 0],
                &[0, 0, 2, 2, 0],
                &[0, 0, 0, 0, 0]
            ]),
            0
        )
    }
}
