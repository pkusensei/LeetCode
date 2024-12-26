mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

const MOD: i32 = 1_000_000_007;

pub fn paths_with_max_score(board: Vec<String>) -> Vec<i32> {
    let n = board.len();
    let mut board: Vec<_> = board.into_iter().map(|s| s.into_bytes()).collect();
    // board[0][0] = b'0';
    // dfs(&board, n, [0, 0], &mut vec![vec![None; n]; n])
    //     .map(|v| v.to_vec())
    //     .unwrap_or(vec![0; 2])
    let mut dp = vec![vec![[i32::MIN, 0]; 1 + n]; 1 + n];
    dp[n - 1][n - 1] = [0, 1];
    for row in (0..n).rev() {
        for col in (0..n).rev() {
            if b"XS".contains(&board[row][col]) {
                continue;
            }
            for (dr, dc) in [(1, 0), (1, 1), (0, 1)] {
                let prev = dp[row + dr][col + dc][0];
                if dp[row][col][0] < prev {
                    dp[row][col] = [prev, 0];
                }
                if dp[row][col][0] == prev {
                    dp[row][col][1] += dp[row + dr][col + dc][1];
                }
                dp[row][col][1] %= MOD;
            }
            if row > 0 || col > 0 {
                dp[row][col][0] += (board[row][col] - b'0') as i32
            }
        }
    }
    if dp[0][0][1] > 0 {
        dp[0][0].to_vec()
    } else {
        vec![0; 2]
    }
}

// TLE's
fn dfs(
    board: &[Vec<u8>],
    n: usize,
    [row, col]: [usize; 2],
    memo: &mut [Vec<Option<[i32; 2]>>],
) -> Option<[i32; 2]> {
    if row == n - 1 && col == n - 1 {
        return Some([0, 1]);
    }
    if memo[row][col].is_some() {
        return memo[row][col];
    }
    let mut curr_score = -1;
    let mut curr_path = 0;
    for [nr, nc] in [(1, 0), (1, 1), (0, 1)].map(|(dr, dc)| [row + dr, col + dc]) {
        if let Some([score, path]) = match board.get(nr).and_then(|r| r.get(nc)) {
            None | Some(b'X') => None,
            Some(_) => dfs(board, n, [nr, nc], memo),
        } {
            match curr_score.cmp(&score) {
                std::cmp::Ordering::Less => {
                    curr_path = path;
                    curr_score = score;
                }
                std::cmp::Ordering::Equal => curr_path = (curr_path + path) % MOD,
                std::cmp::Ordering::Greater => (),
            }
        }
    }
    if curr_path > 0 {
        let res = Some([curr_score + (board[row][col] - b'0') as i32, curr_path]);
        memo[row][col] = res;
        res
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            paths_with_max_score(vec!["E23".into(), "2X2".into(), "12S".into()]),
            [7, 1]
        );
        assert_eq!(
            paths_with_max_score(vec!["E12".into(), "1X1".into(), "21S".into()]),
            [4, 2]
        );
        assert_eq!(
            paths_with_max_score(vec!["E11".into(), "XXX".into(), "11S".into()]),
            [0, 0]
        );
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

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
