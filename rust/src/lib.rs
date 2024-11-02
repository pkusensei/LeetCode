mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn possible_bipartition(n: i32, dislikes: &[[i32; 2]]) -> bool {
    let n = n as usize;
    let graph = dislikes.iter().fold(vec![vec![]; 1 + n], |mut acc, v| {
        acc[v[0] as usize].push(v[1] as usize);
        acc[v[1] as usize].push(v[0] as usize);
        acc
    });
    let mut colors = vec![-1; 1 + n];
    for i in 1..n {
        if colors[i] < 0 && !dfs(&graph, &mut colors, i, 0) {
            return false;
        }
    }
    true
}

fn dfs(graph: &[Vec<usize>], colors: &mut [i32], curr: usize, color: i32) -> bool {
    if colors[curr] > -1 {
        return colors[curr] == color;
    }
    colors[curr] = color;
    for &next in graph[curr].iter() {
        if !dfs(graph, colors, next, 1 - color) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(possible_bipartition(4, &[[1, 2], [1, 3], [2, 4]]));
        debug_assert!(!possible_bipartition(3, &[[1, 2], [1, 3], [2, 3]]));
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
        T1: Into<f64>,
        T2: Into<f64>,
    {
        const EP: f64 = 1e-5;
        debug_assert!((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP);
    }
}
