mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_visited_cells(grid: &[&[i32]]) -> i32 {
    use std::{cmp::Reverse, collections::BinaryHeap};
    let [rows, cols] = get_dimensions(grid);
    let mut dp = vec![vec![i64::MAX; cols]; rows];
    dp[0][0] = 1;
    let mut heap = BinaryHeap::from([Reverse((1, 0, 0))]);
    while let Some(Reverse((dist, r, c))) = heap.pop() {
        if r == rows - 1 && c == cols - 1 {
            return dist as i32;
        }
        for k in 1 + c..(c + grid[r][c] as usize + 1).min(cols) {
            if dp[r][k] > 1 + dist {
                dp[r][k] = 1 + dist;
                heap.push(Reverse((1 + dist, r, k)));
                if r == rows - 1 && k == cols - 1 {
                    return 1 + dist as i32;
                }
            }
        }
        for k in 1 + r..(r + grid[r][c] as usize + 1).min(rows) {
            if dp[k][c] > 1 + dist {
                dp[k][c] = 1 + dist;
                heap.push(Reverse((1 + dist, k, c)));
                if k == rows - 1 && c == cols - 1 {
                    return 1 + dist as i32;
                }
            }
        }
    }
    -1
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
            minimum_visited_cells(&[&[3, 4, 2, 1], &[4, 2, 3, 1], &[2, 1, 0, 0], &[2, 4, 0, 0]]),
            4
        );
        assert_eq!(
            minimum_visited_cells(&[&[3, 4, 2, 1], &[4, 2, 1, 1], &[2, 1, 1, 0], &[3, 4, 1, 0]]),
            3
        );
        assert_eq!(minimum_visited_cells(&[&[2, 1, 0], &[1, 0, 0]]), -1);
    }

    #[test]
    fn test() {}
}
