mod dsu;
mod helper;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn min_trio_degree(n: i32, edges: &[[i32; 2]]) -> i32 {
    let n = n as usize;
    let (adj, degs) = edges.iter().fold(
        (vec![HashSet::new(); n], vec![0; n]),
        |(mut adj, mut degs), e| {
            adj[e[0] as usize - 1].insert(e[1] as usize - 1);
            adj[e[1] as usize - 1].insert(e[0] as usize - 1);
            degs[e[0] as usize - 1] += 1;
            degs[e[1] as usize - 1] += 1;
            (adj, degs)
        },
    );
    let mut res = i32::MAX;
    for (a, neigbors) in adj.iter().enumerate() {
        for (i, &b) in neigbors.iter().enumerate() {
            for &c in neigbors.iter().skip(1 + i) {
                if adj[b].contains(&c) {
                    res = res.min([a, b, c].map(|v| degs[v]).iter().sum::<i32>() - 6);
                }
            }
        }
    }
    if res == i32::MAX {
        -1
    } else {
        res
    }
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
        assert_eq!(
            min_trio_degree(6, &[[1, 2], [1, 3], [3, 2], [4, 1], [5, 2], [3, 6]]),
            3
        );
        assert_eq!(
            min_trio_degree(
                7,
                &[
                    [1, 3],
                    [4, 1],
                    [4, 3],
                    [2, 5],
                    [5, 6],
                    [6, 7],
                    [7, 5],
                    [2, 6]
                ]
            ),
            0
        );
    }

    #[test]
    fn test() {}
}
