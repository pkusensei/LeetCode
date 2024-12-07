mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn critical_connections(n: i32, connections: &[[i32; 2]]) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut adj = vec![vec![]; n];
    for c in connections.iter() {
        adj[c[0] as usize].push(c[1] as usize);
        adj[c[1] as usize].push(c[0] as usize);
    }
    let mut ranks = vec![-2; n];
    let mut res = vec![];
    dfs(&adj, &mut ranks, n, 0, 0, &mut res);
    res
}

fn dfs(
    adj: &[Vec<usize>],
    ranks: &mut [i32],
    n: usize,
    node: usize,
    rank: i32,
    res: &mut Vec<Vec<i32>>,
) -> i32 {
    if ranks[node] >= 0 {
        return ranks[node];
    }
    ranks[node] = rank;
    let mut min_rank = rank;
    for &neighbor in adj[node].iter() {
        // avoid goes back to parent immediately => init value == -2
        // a neighbor with a highrt rank => a cycle is found
        if ranks[neighbor] == rank - 1 || ranks[neighbor] > rank {
            continue;
        }
        let nr = dfs(adj, ranks, n, neighbor, 1 + rank, res);
        min_rank = min_rank.min(nr);
        // bridges => edges not in a cycle
        if nr > rank {
            res.push(vec![node as i32, neighbor as i32]);
        }
    }
    min_rank
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            critical_connections(4, &[[0, 1], [1, 2], [2, 0], [1, 3]]),
            [[1, 3]]
        );
        assert_eq!(critical_connections(2, &[[0, 1]]), [[0, 1]]);
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
        if (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() > EP {
            dbg!(a, b);
        }
    }
}
