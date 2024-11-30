mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn valid_arrangement(pairs: &[[i32; 2]]) -> Vec<Vec<i32>> {
    let mut graph = pairs
        .iter()
        .fold(HashMap::<i32, Vec<_>>::new(), |mut acc, v| {
            acc.entry(v[0]).or_default().push(v[1]);
            acc
        });
    let start = find_ends(pairs).map(|v| v[0]).unwrap_or(pairs[0][0]);
    let mut res = vec![];
    dfs(&mut graph, start, &mut res);
    res.reverse();
    res.windows(2).map(|w| w.to_vec()).collect()
}

fn dfs(graph: &mut HashMap<i32, Vec<i32>>, curr: i32, path: &mut Vec<i32>) {
    while let Some(next) = graph.get_mut(&curr).and_then(|v| v.pop()) {
        dfs(graph, next, path);
    }
    path.push(curr);
}

fn find_ends(pairs: &[[i32; 2]]) -> Option<[i32; 2]> {
    let counts = pairs.iter().fold(HashMap::new(), |mut acc, v| {
        acc.entry(v[0]).or_insert((0, 0)).0 += 1;
        acc.entry(v[1]).or_insert((0, 0)).1 += 1;
        acc
    });
    let start = counts
        .iter()
        .find_map(|(&k, &v)| if v.0 - v.1 == 1 { Some(k) } else { None });
    let end = counts
        .iter()
        .find_map(|(&k, &v)| if v.0 - v.1 == -1 { Some(k) } else { None });
    if let (Some(a), Some(b)) = (start, end) {
        Some([a, b])
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            valid_arrangement(&[[5, 1], [4, 5], [11, 9], [9, 4]]),
            [[11, 9], [9, 4], [4, 5], [5, 1]]
        );
        debug_assert_eq!(
            valid_arrangement(&[[1, 3], [3, 2], [2, 1]]),
            [[1, 3], [3, 2], [2, 1]]
        );
        debug_assert_eq!(
            valid_arrangement(&[[1, 2], [1, 3], [2, 1]]),
            [[1, 2], [2, 1], [1, 3]]
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
        if (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() > EP {
            dbg!(a, b);
        }
    }
}
