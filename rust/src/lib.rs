mod dsu;
mod helper;
mod trie;

use std::collections::BTreeMap;

#[allow(unused_imports)]
use helper::*;

pub fn matrix_rank_transform(matrix: &[&[i32]]) -> Vec<Vec<i32>> {
    let [rows, cols] = get_dimensions(matrix);
    let mut map = BTreeMap::<_, Vec<_>>::new();
    for (r, row) in matrix.iter().enumerate() {
        for (c, &v) in row.iter().enumerate() {
            map.entry(v).or_default().push([r, c]);
        }
    }
    let mut res = vec![vec![0; cols]; rows];
    let mut rank = vec![0; rows + cols];
    for pos in map.into_values() {
        let mut parent: Vec<_> = (0..rows + cols).collect();
        for &[row, col] in pos.iter() {
            let r1 = find(&mut parent, row);
            let r2 = find(&mut parent, col + rows);
            parent[r1] = r2; // Union row and col
                             // Root points to highest rank
            rank[r2] = rank[r1].max(rank[r2]);
        }
        let mut rank2 = rank.clone();
        for [row, col] in pos {
            let r = find(&mut parent, row);
            res[row][col] = 1 + rank[r];
            // Writes this rank onto row and col
            rank2[row] = 1 + rank[r];
            rank2[col + rows] = 1 + rank[r];
        }
        rank = rank2;
    }
    res
}

fn find(parent: &mut [usize], x: usize) -> usize {
    if parent[x] != x {
        parent[x] = find(parent, parent[x]);
    }
    parent[x]
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
    fn basics() {
        assert_eq!(matrix_rank_transform(&[&[1, 2], &[3, 4]]), [[1, 2], [2, 3]]);
        assert_eq!(matrix_rank_transform(&[&[7, 7], &[7, 7]]), [[1, 1], [1, 1]]);
        assert_eq!(
            matrix_rank_transform(&[&[20, -21, 14], &[-19, 4, 19], &[22, -47, 24], &[-19, 4, 19]]),
            [[4, 2, 3], [1, 3, 4], [5, 1, 6], [1, 3, 4]]
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            matrix_rank_transform(&[
                &[-37, -50, -3, 44],
                &[-37, 46, 13, -32],
                &[47, -42, -3, -40],
                &[-17, -22, -39, 24]
            ]),
            [[2, 1, 4, 6], [2, 6, 5, 4], [5, 2, 4, 3], [4, 3, 1, 5]]
        );
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
