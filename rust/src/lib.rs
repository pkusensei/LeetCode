mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_profit(n: i32, edges: &[[i32; 2]], score: &[i32]) -> i32 {
    let n = n as usize;
    let mut indegs = vec![0; n];
    let mut adj = vec![vec![]; n];
    for e in edges.iter() {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        adj[a].push(b);
        indegs[b] += 1;
    }
    dfs(&adj, score, &mut indegs, 0, &mut vec![-1; 1 << n])
}

fn dfs(
    adj: &[Vec<usize>],
    score: &[i32],
    indegs: &mut [i32],
    mask: usize,
    memo: &mut [i32],
) -> i32 {
    let n = score.len();
    if n == mask.count_ones() as usize {
        return 0;
    }
    if memo[mask] > -1 {
        return memo[mask];
    }
    let mut res = 0;
    for i in 0..n {
        if (mask >> i) & 1 == 0 && indegs[i] == 0 {
            for &node in &adj[i] {
                indegs[node] -= 1;
            }
            let next = mask | (1 << i);
            res =
                res.max(next.count_ones() as i32 * score[i] + dfs(adj, score, indegs, next, memo));
            for &node in &adj[i] {
                indegs[node] += 1;
            }
        }
    }
    memo[mask] = res;
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
    fn basics() {
        assert_eq!(max_profit(3, &[[0, 1], [0, 2]], &[1, 6, 3]), 25);
    }

    #[test]
    fn test() {}
}
