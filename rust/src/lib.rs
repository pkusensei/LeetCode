mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn max_output(n: i32, edges: &[[i32; 2]], price: &[i32]) -> i64 {
    let n = n as usize;
    if n == 1 {
        return 0;
    }
    if n == 2 {
        return i64::from(*price.iter().max().unwrap());
    }
    let mut adj = vec![vec![]; n];
    let mut degs = vec![0; n];
    for e in edges.iter() {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        adj[a].push(b);
        adj[b].push(a);
        degs[a] += 1;
        degs[b] += 1;
    }
    let mut res = 0;
    let mut memo = HashMap::new();
    for node in degs
        .into_iter()
        .enumerate()
        .filter_map(|(i, v)| if v > 1 { Some(i) } else { None })
    {
        res = res.max(dfs(&adj, price, node, n, &mut memo))
    }
    res
}

fn dfs(
    adj: &[Vec<usize>],
    price: &[i32],
    node: usize,
    prev: usize,
    memo: &mut HashMap<[usize; 2], i64>,
) -> i64 {
    if let Some(&v) = memo.get(&[node, prev]) {
        return v;
    }
    let mut res = 0;
    for &next in adj[node].iter() {
        if prev != next {
            res = res.max(dfs(adj, price, next, node, memo));
        }
    }
    res += i64::from(price[node]);
    memo.insert([node, prev], res);
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
            max_output(
                6,
                &[[0, 1], [1, 2], [1, 3], [3, 4], [3, 5]],
                &[9, 8, 7, 6, 10, 5]
            ),
            24
        );
        assert_eq!(max_output(3, &[[0, 1], [1, 2]], &[1, 1, 1]), 2);
    }

    #[test]
    fn test() {}
}
