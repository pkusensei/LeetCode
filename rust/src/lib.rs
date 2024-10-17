mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn cherry_pickup(grid: &[&[i32]]) -> i32 {
    let n = grid.len();
    // let mut dp = vec![vec![vec![None; n]; n]; n];
    // solve(&mut dp, grid, 0, 0, 0).unwrap_or(0)
    let mut dp = vec![vec![i32::MIN; n]; n];
    dp[0][0] = grid[0][0];

    for step in 1..=2 * (n - 1) {
        let mut dp2 = vec![vec![i32::MIN; n]; n];
        for r1 in step.saturating_sub(n - 1)..=step.min(n - 1) {
            for r2 in step.saturating_sub(n - 1)..=step.min(n - 1) {
                let (c1, c2) = (step - r1, step - r2);
                if grid[r1][c1] == -1 || grid[r2][c2] == -1 {
                    continue;
                }
                let mut val = grid[r1][c1];
                if r1 != r2 {
                    val += grid[r2][c2];
                }
                for pr1 in r1.saturating_sub(1)..=r1 {
                    for pr2 in r2.saturating_sub(1)..=r2 {
                        dp2[r1][r2] = dp2[r1][r2].max(val + dp[pr1][pr2]);
                    }
                }
            }
        }
        dp = dp2;
    }
    dp[n - 1][n - 1].max(0)
}

// TLEs
fn solve(
    dp: &mut [Vec<Vec<Option<i32>>>],
    grid: &[&[i32]],
    r1: usize,
    c1: usize,
    c2: usize,
) -> Option<i32> {
    let n = grid.len();
    let r2 = r1 + c1 - c2; // r1+c1 == r2+c2
    if [r1, c1, r2, c2].iter().any(|&v| v == n) || grid[r1][c1] == -1 || grid[r2][c2] == -1 {
        None
    } else if n - 1 == r1 && n - 1 == c1 {
        Some(grid[r1][c1])
    } else if let Some(v) = dp[r1][c1][c2] {
        Some(v)
        // Instead of i32::MIN and -999999 trickery
        // an Option<i32> conveys much more precise semantics
        // But it reduces memo space and leads to TLE
    } else {
        let mut res = grid[r1][c1];
        if c1 != c2 {
            res += grid[r2][c2];
        }
        let next = [
            solve(dp, grid, 1 + r1, c1, c2),     // both down
            solve(dp, grid, r1, 1 + c1, 1 + c2), // both right
            solve(dp, grid, 1 + r1, c1, 1 + c2), // one down, one right
            solve(dp, grid, r1, 1 + c1, c2),     // one right, one down
        ]
        .into_iter()
        .flatten()
        .max();
        if let Some(v) = next {
            res += v;
            dp[r1][c1][c2] = Some(res);
            Some(res)
        } else {
            // All next paths are None => into -1 or out of bounds
            // this path is blocked/exhausted
            // It's impossible to advance further => return None
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(cherry_pickup(&[&[0, 1, -1], &[1, 0, -1], &[1, 1, 1]]), 5);
        debug_assert_eq!(cherry_pickup(&[&[1, 1, -1], &[1, -1, 1], &[-1, 1, 1]]), 0);
    }

    #[test]
    fn test() {
        debug_assert_eq!(
            cherry_pickup(&[
                &[1, 1, -1, 1, 1, 1, 0, 1, 1, -1, -1, 1, 1, -1, 1, 1, 1, 1, 0, 1],
                &[1, 1, 1, 0, 1, 1, 0, 1, 0, 1, -1, 1, 1, 1, 1, 1, 1, 0, 1, 1],
                &[1, 1, 1, 1, 1, 1, 1, 0, 1, 1, -1, -1, -1, 1, 1, 1, -1, 1, -1, 1],
                &[1, 1, 1, -1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, -1, -1, 1, 1, 1],
                &[1, -1, -1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                &[1, 1, 1, 0, 1, 1, 1, 1, 1, -1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1],
                &[0, 1, 1, -1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1],
                &[1, 1, -1, 1, 1, 1, -1, 1, 0, 1, 1, 1, 1, 1, 1, -1, 1, 1, 1, 1],
                &[0, -1, 1, 1, 1, -1, 1, 1, 1, -1, 1, 1, 1, 1, 1, 1, -1, 1, 1, 1],
                &[1, 1, -1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, -1, 0, 1, 0, -1, 1],
                &[0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, -1, 0],
                &[1, 1, 1, 1, 1, 1, -1, 1, 0, 1, 1, 1, 1, -1, 1, 1, 1, 0, 1, 1],
                &[1, 1, 1, 1, -1, -1, 1, -1, 1, 1, 1, 1, 1, 1, 1, 1, 1, -1, 1, 1],
                &[-1, 1, 1, 1, 1, -1, 1, 1, 1, 1, 1, 1, -1, 1, 0, 0, 1, 0, 1, 1],
                &[0, 1, -1, 1, 1, -1, 1, 1, 1, -1, 1, 1, 1, 1, 1, 1, 0, -1, 1, 1],
                &[1, 1, 1, -1, 1, 1, 1, -1, 1, -1, 1, 1, 1, 1, 1, 1, 1, 1, 1, -1],
                &[1, -1, 1, 1, 1, 1, 1, 1, 1, 1, 0, -1, 1, 1, 1, 1, 1, 1, 1, -1],
                &[1, 1, 1, -1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, -1, 0, 1, 1],
                &[1, 1, -1, 0, -1, 1, 1, -1, -1, 1, 1, -1, 1, 1, 1, 1, -1, -1, 0, 1],
                &[-1, 0, 0, 1, 1, 1, 1, 1, -1, -1, 1, 1, 1, 0, 1, 0, 0, 1, -1, 1]
            ]),
            71
        );
    }

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
