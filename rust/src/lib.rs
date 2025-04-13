mod dsu;
mod helper;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn count_paths(n: i32, edges: &[[i32; 2]]) -> i64 {
    let n = 1 + n as usize;
    let sieve = primes(n);
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        acc[a].push(b);
        acc[b].push(a);
        acc
    });
    let mut res = 0;
    dfs(&adj, &sieve, 1, n, &mut res);
    res
}

fn dfs(
    adj: &[Vec<usize>],
    sieve: &HashSet<usize>,
    node: usize,
    prev: usize,
    res: &mut i64,
) -> [i64; 2] {
    let is_prime = sieve.contains(&node);
    // [not prime, prime]
    let mut count = [!is_prime, is_prime].map(i64::from);
    for &next in adj[node].iter() {
        if next != prev {
            let [not_prime, prime] = dfs(adj, sieve, next, node, res);
            *res += not_prime * count[1] + prime * count[0];
            if is_prime {
                count[1] += not_prime;
            } else {
                count[0] += not_prime;
                count[1] += prime;
            }
        }
    }
    count
}

fn primes(n: usize) -> HashSet<usize> {
    let mut sieve = vec![true; n];
    sieve[..2].fill(false);
    for p in 2..n {
        if sieve[p] {
            for val in (2 * p..n).step_by(p) {
                sieve[val] = false;
            }
        }
    }
    sieve
        .into_iter()
        .enumerate()
        .filter_map(|(i, v)| if v { Some(i) } else { None })
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
    fn basics() {
        assert_eq!(count_paths(5, &[[1, 2], [1, 3], [2, 4], [2, 5]]), 4);
        assert_eq!(count_paths(6, &[[1, 2], [1, 3], [2, 4], [3, 5], [3, 6]]), 6);
    }

    #[test]
    fn test() {}
}
