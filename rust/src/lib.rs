mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_moves(grid: &[&[i32]]) -> i32 {
    let (rows, cols) = get_dimensions(grid);
    // (0..rows)
    //     .map(|r| dfs(grid, cols, r, 0, &mut vec![vec![-1; cols]; rows]))
    //     .max()
    //     .unwrap_or(0)
    // with_bfs(grid, rows, cols)
    let (mut prev, mut curr) = (vec![1; rows], vec![0; rows]);
    let mut res = 0;
    for c in 1..cols {
        for r in 0..rows {
            for nr in r.saturating_sub(1)..=r + 1 {
                if grid
                    .get(nr)
                    .is_some_and(|v| v.get(c - 1).is_some_and(|&num| num < grid[r][c]))
                    && prev[nr] > 0
                {
                    curr[r] = curr[r].max(1 + prev[nr]);
                }
            }
            res = res.max(curr[r] - 1);
        }
        prev = curr;
        curr = vec![0; rows];
    }
    res
}

fn dfs(grid: &[&[i32]], cols: usize, r: usize, c: usize, dp: &mut [Vec<i32>]) -> i32 {
    if c == cols - 1 {
        return 0;
    }
    if dp[r][c] > -1 {
        return dp[r][c];
    }
    let val = grid[r][c];
    let mut res = 0;
    for nr in r.saturating_sub(1)..=r + 1 {
        if grid
            .get(nr)
            .is_some_and(|v| v.get(1 + c).is_some_and(|&num| num > val))
        {
            res = res.max(1 + dfs(grid, cols, nr, 1 + c, dp));
        }
    }
    dp[r][c] = res;
    res
}

fn with_bfs(grid: &[&[i32]], rows: usize, cols: usize) -> i32 {
    let mut visited = vec![vec![false; cols]; rows];
    // interesting choice
    let mut queue: std::collections::VecDeque<_> = (0..rows).map(|r| (r, 0, 0)).collect();
    let mut res = 0;
    while !queue.is_empty() {
        let mut n = queue.len();
        while n > 0 {
            n -= 1;
            let Some((r, c, count)) = queue.pop_front() else {
                break;
            };
            res = res.max(count);
            for nr in r.saturating_sub(1)..=r + 1 {
                if grid
                    .get(nr)
                    .is_some_and(|v| v.get(1 + c).is_some_and(|&num| num > grid[r][c]))
                    && !visited[nr][1 + c]
                {
                    visited[nr][1 + c] = true;
                    queue.push_back((nr, 1 + c, 1 + count));
                }
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            max_moves(&[
                &[2, 4, 3, 5],
                &[5, 4, 9, 3],
                &[3, 4, 2, 11],
                &[10, 9, 13, 15]
            ]),
            3
        );
        debug_assert_eq!(max_moves(&[&[3, 2, 4], &[2, 1, 9], &[1, 1, 7]]), 0);
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
