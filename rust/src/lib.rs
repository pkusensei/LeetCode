mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_sub_trees(n: i32, edges: &[[i32; 2]], labels: &str) -> Vec<i32> {
    let n = n as usize;
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        acc[e[0] as usize].push(e[1] as usize);
        acc[e[1] as usize].push(e[0] as usize);
        acc
    });
    let mut res = vec![0; n];
    dfs(&adj, labels.as_bytes(), 0, None, &mut res);
    res
}

fn dfs(
    adj: &[Vec<usize>],
    labels: &[u8],
    node: usize,
    parent: Option<usize>,
    res: &mut [i32],
) -> [i32; 26] {
    let mut arr = [0; 26];
    let idx = (labels[node] - b'a') as usize;
    arr[idx] += 1;
    for &ch in adj[node].iter() {
        if parent.is_some_and(|p| p == ch) {
            continue;
        }
        for (v1, v2) in arr.iter_mut().zip(dfs(adj, labels, ch, Some(node), res)) {
            *v1 += v2;
        }
    }
    res[node] += arr[idx];
    arr
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            count_sub_trees(
                7,
                &[[0, 1], [0, 2], [1, 4], [1, 5], [2, 3], [2, 6]],
                "abaedcd"
            ),
            [2, 1, 1, 1, 1, 1, 1]
        );
        assert_eq!(
            count_sub_trees(4, &[[0, 1], [1, 2], [0, 3]], "bbbb"),
            [4, 2, 1, 1]
        );
        assert_eq!(
            count_sub_trees(5, &[[0, 1], [0, 2], [1, 3], [0, 4]], "aabab"),
            [3, 2, 1, 1, 1]
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
