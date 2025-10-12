mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::sync::LazyLock;

#[allow(unused_imports)]
use helper::*;

pub fn sum_of_ancestors(n: i32, edges: Vec<Vec<i32>>, nums: Vec<i32>) -> i64 {
    let n = n as usize;
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        acc[a].push(b);
        acc[b].push(a);
        acc
    });
    let facts: Vec<i32> = nums.iter().map(|&v| factors(v)).collect();
    let max = *facts.iter().max().unwrap_or(&1);
    dfs(&adj, &facts, 0, n, &mut vec![0; 1 + max as usize])
}

fn dfs(adj: &[Vec<usize>], facts: &[i32], node: usize, prev: usize, freq: &mut [i64]) -> i64 {
    let f = facts[node];
    let mut res = freq[f as usize];
    freq[f as usize] += 1;
    for &next in &adj[node] {
        if next != prev {
            res += dfs(adj, facts, next, node, freq);
        }
    }
    freq[f as usize] -= 1;
    res
}

fn factors(mut num: i32) -> i32 {
    for &p in S.iter() {
        while num >= p.pow(2) && num % p.pow(2) == 0 {
            num /= p.pow(2);
        }
    }
    num
}

static S: LazyLock<Vec<i32>> = LazyLock::new(sieve);

fn sieve() -> Vec<i32> {
    const MAX: usize = 100_000_usize.isqrt();
    let mut res = vec![true; 1 + MAX];
    res[..2].fill(false);
    res[2] = true;
    for p in 2..=MAX.isqrt() {
        if res[p] {
            for val in (p * p..=MAX).step_by(p) {
                res[val] = false;
            }
        }
    }
    res.into_iter()
        .enumerate()
        .filter_map(|(p, v)| if v { Some(p as i32) } else { None })
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
