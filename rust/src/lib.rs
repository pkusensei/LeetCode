mod dsu;
mod helper;
mod trie;

use std::collections::VecDeque;

#[allow(unused_imports)]
use helper::*;

// O(V+E)
pub fn largest_path_value(colors: &str, edges: &[[i32; 2]]) -> i32 {
    let n = colors.len();
    let mut adj = vec![vec![]; n]; // [a -> b]
    let mut prevs = vec![vec![]; n]; // record parent node(s)
    let mut indegs = vec![0; n];
    for e in edges.iter() {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        adj[a].push(b);
        prevs[b].push(a);
        indegs[b] += 1;
    }
    let mut dp = vec![[0; 26]; n];
    let mut queue: VecDeque<_> = indegs
        .iter()
        .enumerate()
        .filter_map(|(i, &v)| if v == 0 { Some(i) } else { None })
        .collect();
    if queue.is_empty() {
        return -1;
    }
    let mut res = 0;
    while let Some(curr) = queue.pop_front() {
        let color = usize::from(colors.as_bytes()[curr] - b'a');
        if prevs[curr].is_empty() {
            dp[curr][color] = 1; // root node
        } else {
            for &prev in prevs[curr].iter() {
                for c in 0..26 {
                    dp[curr][c] = dp[curr][c].max(dp[prev][c] + i32::from(c == color));
                }
            }
        }
        res = res.max(dp[curr][color]);
        for &next in adj[curr].iter() {
            indegs[next] -= 1;
            if indegs[next] <= 0 {
                queue.push_back(next);
            }
        }
    }
    if indegs.iter().any(|&v| v > 0) {
        return -1; // cycle
    }
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
        assert_eq!(
            largest_path_value("abaca", &[[0, 1], [0, 2], [2, 3], [3, 4]]),
            3
        );
        assert_eq!(largest_path_value("a", &[[0, 0]]), -1);
    }

    #[test]
    fn test() {
        assert_eq!(
            largest_path_value(
                "nnllnzznn",
                &[
                    [0, 1],
                    [1, 2],
                    [2, 3],
                    [2, 4],
                    [3, 5],
                    [4, 6],
                    [3, 6],
                    [5, 6],
                    [6, 7],
                    [7, 8]
                ]
            ),
            5
        );
    }
}
