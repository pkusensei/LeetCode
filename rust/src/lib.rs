mod dsu;
mod helper;
mod trie;

use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(unused_imports)]
use helper::*;

pub fn count_paths(n: i32, roads: &[[i32; 3]]) -> i32 {
    const MOD: i32 = 1_000_000_007;
    let n = n as usize;
    let adj = roads.iter().fold(vec![vec![]; n], |mut acc, road| {
        let (a, b, t) = (road[0] as usize, road[1] as usize, i64::from(road[2]));
        acc[a].push((b, t));
        acc[b].push((a, t));
        acc
    });
    let mut dists = vec![i64::MAX; n];
    dists[0] = 0;
    let mut dp = vec![0; n];
    dp[0] = 1;
    let mut heap = BinaryHeap::from([(Reverse(0), 0)]);
    while let Some((Reverse(cost), node)) = heap.pop() {
        if cost > dists[node] {
            continue;
        }
        for &(next, t) in adj[node].iter() {
            let nc = t + cost;
            if nc < dists[next] {
                dists[next] = nc;
                heap.push((Reverse(nc), next));
                dp[next] = dp[node]
            } else if nc == dists[next] {
                dp[next] += dp[node];
                dp[next] %= MOD;
            }
        }
    }
    dp[n - 1]
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
            count_paths(
                7,
                &[
                    [0, 6, 7],
                    [0, 1, 2],
                    [1, 2, 3],
                    [1, 3, 3],
                    [6, 3, 3],
                    [3, 5, 1],
                    [6, 5, 1],
                    [2, 5, 1],
                    [0, 4, 5],
                    [4, 6, 2]
                ]
            ),
            4
        );
        assert_eq!(count_paths(2, &[[1, 0, 10]]), 1);
    }

    #[test]
    fn test() {
        assert_eq!(
            count_paths(
                12,
                &[
                    [1, 0, 2348],
                    [2, 1, 2852],
                    [2, 0, 5200],
                    [3, 1, 12480],
                    [2, 3, 9628],
                    [4, 3, 7367],
                    [4, 0, 22195],
                    [5, 4, 5668],
                    [1, 5, 25515],
                    [0, 5, 27863],
                    [6, 5, 836],
                    [6, 0, 28699],
                    [2, 6, 23499],
                    [6, 3, 13871],
                    [1, 6, 26351],
                    [5, 7, 6229],
                    [2, 7, 28892],
                    [1, 7, 31744],
                    [3, 7, 19264],
                    [6, 7, 5393],
                    [2, 8, 31998],
                    [8, 7, 3106],
                    [3, 8, 22370],
                    [8, 4, 15003],
                    [8, 6, 8499],
                    [8, 5, 9335],
                    [8, 9, 5258],
                    [9, 2, 37256],
                    [3, 9, 27628],
                    [7, 9, 8364],
                    [1, 9, 40108],
                    [9, 5, 14593],
                    [2, 10, 45922],
                    [5, 10, 23259],
                    [9, 10, 8666],
                    [10, 0, 51122],
                    [10, 3, 36294],
                    [10, 4, 28927],
                    [11, 4, 25190],
                    [11, 9, 4929],
                    [11, 8, 10187],
                    [11, 6, 18686],
                    [2, 11, 42185],
                    [11, 3, 32557],
                    [1, 11, 45037]
                ]
            ),
            166
        );
    }
}
