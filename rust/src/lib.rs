mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> Vec<i32> {
    let (colors1, count1) = build(&edges1);
    let (_, count2) = build(&edges2);
    let mut res = vec![];
    for c in colors1 {
        res.push(count1[usize::from(c)] + count2[0].max(count2[1]));
    }
    res
}

fn build(edges: &[Vec<i32>]) -> (Vec<bool>, [i32; 2]) {
    let n = 1 + edges.len();
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        acc[a].push(b);
        acc[b].push(a);
        acc
    });
    let mut colors = vec![false; n];
    let count = dfs(&adj, 0, n, false, &mut colors);
    (colors, count)
}

fn dfs(adj: &[Vec<usize>], node: usize, prev: usize, color: bool, colors: &mut [bool]) -> [i32; 2] {
    colors[node] = color;
    let mut count = [0, 0];
    count[usize::from(color)] += 1;
    for &next in &adj[node] {
        if next != prev {
            let [a, b] = dfs(adj, next, node, !color, colors);
            count[0] += a;
            count[1] += b;
        }
    }
    count
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
