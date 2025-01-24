mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut degs = vec![0; n];
    let mut adj = vec![vec![false; n]; n];
    for r in roads.iter() {
        let a = r[0] as usize;
        let b = r[1] as usize;
        degs[a] += 1;
        degs[b] += 1;
        adj[a][b] = true;
        adj[b][a] = true;
    }
    let mut res = 0;
    for (a, deg1) in degs.iter().enumerate() {
        for (b, deg2) in degs.iter().enumerate().skip(1 + a) {
            let curr = deg1 + deg2 - i32::from(adj[a][b]);
            res = res.max(curr);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    #[test]
    fn basics() {}

    #[test]
    fn test() {}

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
