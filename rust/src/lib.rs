mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_sessions(tasks: &[i32], session_time: i32) -> i32 {
    let n = tasks.len();
    dfs(
        tasks,
        session_time,
        0,
        0,
        &mut vec![vec![-1; 1 << n]; session_time as usize],
    )
}

fn dfs(tasks: &[i32], session_time: i32, mask: usize, curr: i32, memo: &mut [Vec<i32>]) -> i32 {
    let n = tasks.len();
    if mask.count_ones() as usize >= n {
        return 0;
    }
    if memo[curr as usize][mask] > -1 {
        return memo[curr as usize][mask];
    }
    let mut res = n as i32;
    for idx in 0..n {
        if mask & (1 << idx) > 0 {
            continue;
        }
        let t = tasks[idx];
        if t <= curr {
            res = res.min(dfs(tasks, session_time, mask | (1 << idx), curr - t, memo));
        }
        res = res.min(
            1 + dfs(
                tasks,
                session_time,
                mask | (1 << idx),
                session_time - t,
                memo,
            ),
        );
    }
    memo[curr as usize][mask] = res;
    res
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
        assert_eq!(min_sessions(&[1, 2, 3], 3), 2);
        assert_eq!(min_sessions(&[3, 1, 3, 1, 1], 8), 2);
        assert_eq!(min_sessions(&[1, 2, 3, 4, 5], 15), 1);
    }

    #[test]
    fn test() {
        assert_eq!(min_sessions(&[9, 8, 8, 4, 6], 14), 3);
    }
}
