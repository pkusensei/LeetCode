mod dsu;
mod helper;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn max_k_divisible_components(n: i32, edges: &[[i32; 2]], values: &[i32], k: i32) -> i32 {
    let mut adj = vec![vec![]; n as usize];
    for e in edges.iter() {
        adj[e[0] as usize].push(e[1] as usize);
        adj[e[1] as usize].push(e[0] as usize);
    }
    let (_, count) = dfs(0, &adj, values, k.into(), &mut HashSet::new());
    // count node 0 in <= sum(values)%k==0
    1 + count
}

fn dfs(
    node: usize,
    adj: &[Vec<usize>],
    values: &[i32],
    k: i64,
    seen: &mut HashSet<usize>,
) -> (i64, i32) {
    let mut sum = i64::from(values[node]);
    let mut count = 0;
    seen.insert(node);
    for &neighbor in adj[node].iter() {
        if !seen.contains(&neighbor) {
            let (s, c) = dfs(neighbor, adj, values, k, seen);
            count += c; // propagate subtree count
            if s % k == 0 {
                // subtree sum is multiple of k => detach
                count += 1;
            } else {
                // merge it with current node
                sum += s;
            }
        }
    }
    (sum, count)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            max_k_divisible_components(5, &[[0, 2], [1, 2], [1, 3], [2, 4]], &[1, 8, 1, 4, 4], 6),
            2
        );
        assert_eq!(
            max_k_divisible_components(
                7,
                &[[0, 1], [0, 2], [1, 3], [1, 4], [2, 5], [2, 6]],
                &[3, 0, 6, 1, 5, 2, 1],
                3
            ),
            3
        );
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
