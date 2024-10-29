mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn loud_and_rich(richer: &[[i32; 2]], quiet: &[i32]) -> Vec<i32> {
    let n = quiet.len();
    let graph: HashMap<_, Vec<_>> = richer.iter().fold(HashMap::new(), |mut acc, v| {
        acc.entry(v[1] as usize).or_default().push(v[0] as usize);
        acc
    });
    let mut res = vec![-1; n];
    for i in 0..n {
        dfs(&graph, quiet, i, &mut res);
    }
    res
}

fn dfs(graph: &HashMap<usize, Vec<usize>>, quiet: &[i32], curr: usize, res: &mut [i32]) -> usize {
    if res[curr] > -1 {
        return res[curr] as usize;
    }
    if let Some(v) = graph.get(&curr) {
        let r = v
            .iter()
            .map(|&next| dfs(graph, quiet, next, res))
            .chain(std::iter::once(curr))
            .min_by_key(|i| quiet[*i])
            .unwrap_or(curr);
        res[curr] = r as i32;
        r
    } else {
        res[curr] = curr as i32;
        curr
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            loud_and_rich(
                &[[1, 0], [2, 1], [3, 1], [3, 7], [4, 3], [5, 3], [6, 3]],
                &[3, 2, 5, 4, 6, 1, 7, 0],
            ),
            [5, 5, 2, 5, 4, 5, 6, 7]
        );
        debug_assert_eq!(loud_and_rich(&[], &[0]), [0]);
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
}
