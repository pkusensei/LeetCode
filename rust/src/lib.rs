mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_pairs_of_connectable_servers(edges: &[[i32; 3]], signal_speed: i32) -> Vec<i32> {
    let n = 1 + edges.len();
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        acc[a].push((b, e[2]));
        acc[b].push((a, e[2]));
        acc
    });
    (0..n)
        .map(|node| dfs(&adj, signal_speed, node, n, 0))
        .collect()
}

fn dfs(adj: &[Vec<(usize, i32)>], signal: i32, node: usize, prev: usize, dist: i32) -> i32 {
    let mut sum = 0;
    let mut subtree = vec![];
    for &(next, weight) in adj[node].iter() {
        if next != prev {
            let v = dfs(adj, signal, next, node, dist + weight);
            sum += v;
            if dist == 0 {
                subtree.push(v);
            }
        }
    }
    if dist == 0 {
        subtree.iter().map(|v| (sum - v) * v).sum::<i32>() / 2
    } else {
        sum + i32::from(dist % signal == 0)
    }
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
            count_pairs_of_connectable_servers(
                &[[0, 1, 1], [1, 2, 5], [2, 3, 13], [3, 4, 9], [4, 5, 2]],
                1
            ),
            [0, 4, 6, 6, 4, 0]
        );
        assert_eq!(
            count_pairs_of_connectable_servers(
                &[
                    [0, 6, 3],
                    [6, 5, 3],
                    [0, 3, 1],
                    [3, 2, 7],
                    [3, 1, 6],
                    [3, 4, 2]
                ],
                3
            ),
            [2, 0, 0, 0, 0, 0, 2]
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            count_pairs_of_connectable_servers(
                &[
                    [1, 0, 1],
                    [2, 1, 1],
                    [3, 2, 4],
                    [4, 0, 3],
                    [5, 4, 1],
                    [6, 5, 3]
                ],
                2
            ),
            [2, 0, 2, 0, 1, 0, 0]
        );
    }
}
