mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_value(events: &mut [[i32; 3]], k: i32) -> i32 {
    let (n, k) = (events.len(), k as usize);
    events.sort_unstable_by_key(|e| e[0]); // sort by start
    let nexts: Vec<_> = (0..n)
        .map(|idx| events.partition_point(|e| e[0] <= events[idx][1]))
        .collect();
    let mut dp = vec![vec![0; 1 + n]; 1 + k];
    for idx in (0..n).rev() {
        for count in 1..=k {
            let next = nexts[idx];
            let pick = events[idx][2] + dp[count - 1][next];
            let skip = dp[count][1 + idx];
            dp[count][idx] = pick.max(skip)
        }
    }
    dp[k][0]
    // dfs(events, 0, k, &mut vec![vec![-1; 1 + k]; n])
}

fn dfs(events: &[[i32; 3]], idx: usize, k: usize, memo: &mut [Vec<i32>]) -> i32 {
    let n = events.len();
    if k == 0 || idx >= n {
        return 0;
    }
    if memo[idx][k] > -1 {
        return memo[idx][k];
    }
    let mut res = dfs(events, 1 + idx, k, memo); // skip
    let end = events[idx][1];
    let val = events[idx][2];
    let next = events.partition_point(|v| v[0] <= end);
    res = res.max(val + dfs(events, next, k - 1, memo));
    memo[idx][k] = res;
    res
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        };
    }

    #[test]
    fn basics() {
        assert_eq!(max_value(&mut [[1, 2, 4], [3, 4, 3], [2, 3, 1]], 2), 7);
        assert_eq!(max_value(&mut [[1, 2, 4], [3, 4, 3], [2, 3, 10]], 2), 10);
        assert_eq!(
            max_value(&mut [[1, 1, 1], [2, 2, 2], [3, 3, 3], [4, 4, 4]], 3),
            9
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            max_value(
                &mut [
                    [11, 17, 56],
                    [24, 40, 53],
                    [5, 62, 67],
                    [66, 69, 84],
                    [56, 89, 15]
                ],
                2
            ),
            151
        );
    }
}
