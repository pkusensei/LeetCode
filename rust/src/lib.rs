mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_of_pairs(n: i32, x: i32, y: i32) -> Vec<i32> {
    use itertools::Itertools;
    let n = 1 + n as usize;
    let [x, y] = [x, y].map(|v| v as usize);
    let mut adj = vec![vec![i32::MAX; n]; n];
    for i in 1..n {
        adj[i][i] = 0;
    }
    for i in 1..n - 1 {
        adj[i][1 + i] = 1;
        adj[1 + i][i] = 1;
    }
    adj[x][y] = adj[x][y].min(1);
    adj[y][x] = adj[y][x].min(1);
    for mid in 1..n {
        for a in 1..n {
            for b in 1..n {
                if adj[a][mid] < i32::MAX && adj[mid][b] < i32::MAX {
                    adj[a][b] = adj[a][b].min(adj[a][mid] + adj[mid][b]);
                }
            }
        }
    }
    let map = adj.into_iter().flatten().counts();
    (1..n)
        .map(|k| *map.get(&(k as i32)).unwrap_or(&0) as i32)
        .collect()
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
    fn basics() {}

    #[test]
    fn test() {}
}
