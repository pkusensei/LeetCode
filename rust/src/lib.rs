mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn get_ancestors(n: i32, edges: &[[i32; 2]]) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut adj = vec![vec![]; n];
    let mut indegs = vec![0; n];
    for e in edges.iter() {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        adj[b].push(a);
        indegs[a] += 1;
    }
    let mut res = vec![vec![]; n];
    for (node, &v) in indegs.iter().enumerate() {
        if v == 0 {
            dfs(&adj, node, &mut res);
        }
    }
    res
}

fn dfs(adj: &[Vec<usize>], node: usize, res: &mut [Vec<i32>]) {
    if !res[node].is_empty() {
        return;
    }
    let mut curr = vec![];
    for &next in adj[node].iter() {
        curr.push(next as i32);
        dfs(adj, next, res);
        curr.extend_from_slice(&res[next]);
    }
    curr.sort_unstable();
    curr.dedup();
    res[node] = curr;
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
            get_ancestors(
                8,
                &[
                    [0, 3],
                    [0, 4],
                    [1, 3],
                    [2, 4],
                    [2, 7],
                    [3, 5],
                    [3, 6],
                    [3, 7],
                    [4, 6]
                ]
            ),
            [
                vec![],
                vec![],
                vec![],
                vec![0, 1],
                vec![0, 2],
                vec![0, 1, 3],
                vec![0, 1, 2, 3, 4],
                vec![0, 1, 2, 3]
            ]
        );
        assert_eq!(
            get_ancestors(
                5,
                &[
                    [0, 1],
                    [0, 2],
                    [0, 3],
                    [0, 4],
                    [1, 2],
                    [1, 3],
                    [1, 4],
                    [2, 3],
                    [2, 4],
                    [3, 4]
                ]
            ),
            [vec![], vec![0], vec![0, 1], vec![0, 1, 2], vec![0, 1, 2, 3]]
        );
    }

    #[test]
    fn test() {}
}
