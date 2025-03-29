mod dsu;
mod helper;
mod trie;

use std::collections::{HashSet, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn collect_the_coins(coins: &[i32], edges: &[[i32; 2]]) -> i32 {
    let n = coins.len();
    let mut degs = vec![0; n];
    let mut adj = vec![vec![]; n];
    for e in edges.iter() {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        degs[a] += 1;
        degs[b] += 1;
        adj[a].push(b);
        adj[b].push(a);
    }
    let mut queue: VecDeque<_> = (0..n)
        .filter(|&node| degs[node] == 1 && coins[node] == 0)
        .collect();
    let mut del = HashSet::new();
    while let Some(node) = queue.pop_front() {
        del.insert(node);
        for &next in adj[node].iter() {
            degs[next] -= 1;
            if degs[next] == 1 && coins[next] == 0 {
                queue.push_back(next);
            }
        }
    }
    let mut queue: VecDeque<_> = (0..n)
        .filter_map(|node| {
            if degs[node] == 1 && coins[node] == 1 {
                Some((node, 0))
            } else {
                None
            }
        })
        .collect();
    while let Some((node, depth)) = queue.pop_front() {
        del.insert(node);
        if depth == 0 {
            for &next in adj[node].iter() {
                degs[next] -= 1;
                if degs[next] == 1 {
                    queue.push_back((next, 1));
                }
            }
        }
    }
    ((n - del.len()) as i32 - 1).max(0) * 2
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
            collect_the_coins(
                &[1, 0, 0, 0, 0, 1],
                &[[0, 1], [1, 2], [2, 3], [3, 4], [4, 5]]
            ),
            2
        );
        assert_eq!(
            collect_the_coins(
                &[0, 0, 0, 1, 1, 0, 0, 1],
                &[[0, 1], [0, 2], [1, 3], [1, 4], [2, 5], [5, 6], [5, 7]]
            ),
            2
        );
    }

    #[test]
    fn test() {}
}
