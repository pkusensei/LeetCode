mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_k_divisible_components(n: i32, edges: Vec<Vec<i32>>, values: Vec<i32>, k: i32) -> i32 {
    let n = n as usize;
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        acc[a].push(b);
        acc[b].push(a);
        acc
    });
    dfs(&adj, &values, k, 0, n)[0]
}

// [count of component, sum]
fn dfs(adj: &[Vec<usize>], values: &[i32], k: i32, node: usize, prev: usize) -> [i32; 2] {
    let mut count = 0;
    let mut sum = values[node];
    for &next in &adj[node] {
        if next != prev {
            let [subc, subs] = dfs(adj, values, k, next, node);
            count += subc;
            sum += subs;
        }
    }
    let rem = sum % k;
    if rem % k == 0 {
        [1 + count, 0]
    } else {
        [count, rem]
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

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
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
