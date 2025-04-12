mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_edge_reversals(n: i32, edges: &[[i32; 2]]) -> Vec<i32> {
    let n = n as usize;
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        acc[a].push((b, 1));
        acc[b].push((a, -1));
        acc
    });
    let mut res = vec![0; n];
    res[0] = dfs(&adj, 0, n);
    reroot(&adj, 0, n, &mut res);
    res
}

fn dfs(adj: &[Vec<(usize, i8)>], node: usize, prev: usize) -> i32 {
    let mut res = 0;
    for &(next, val) in adj[node].iter() {
        if prev == next {
            continue;
        }
        res += i32::from(val == -1) + dfs(adj, next, node);
    }
    res
}

fn reroot(adj: &[Vec<(usize, i8)>], node: usize, prev: usize, res: &mut [i32]) {
    let curr = res[node];
    for &(next, val) in adj[node].iter() {
        if prev == next {
            continue;
        }
        // if val==-1 {curr-1}
        // else {curr+1}
        res[next] = curr + i32::from(val);
        reroot(adj, next, node, res);
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
            min_edge_reversals(4, &[[2, 0], [2, 1], [1, 3]]),
            [1, 1, 0, 2]
        );
        assert_eq!(min_edge_reversals(3, &[[1, 2], [2, 0]]), [2, 0, 1]);
    }

    #[test]
    fn test() {}
}
