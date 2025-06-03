mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_max_weight(n: i32, edges: Vec<Vec<i32>>, _threshold: i32) -> i32 {
    let n = n as usize;
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        acc[b].push((a, e[2]));
        acc
    });
    let mut left = 1;
    let mut right = 1_000_001;
    while left < right {
        let mid = left + (right - left) / 2;
        let mut seen = vec![false; n];
        dfs(&adj, mid, 0, &mut seen);
        if seen.into_iter().all(|v| v) {
            right = mid;
        } else {
            left = 1 + mid;
        }
    }
    if left < 1_000_001 { left } else { -1 }
}

fn dfs(adj: &[Vec<(usize, i32)>], mid: i32, node: usize, seen: &mut [bool]) {
    seen[node] = true;
    for &(next, w) in &adj[node] {
        if !seen[next] && w <= mid {
            dfs(adj, mid, next, seen);
        }
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
    fn basics() {}

    #[test]
    fn test() {}
}
