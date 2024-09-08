mod helper;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn longest_increasing_path(matrix: &[&[i32]]) -> i32 {
    let (rows, cols) = get_dimensions(matrix);
    let mut seen = HashMap::new();
    let mut res = 0;
    for y in 0..rows {
        for x in 0..cols {
            res = res.max(dfs(matrix, (x, y), &mut seen))
        }
    }
    res
}

fn dfs(matrix: &[&[i32]], coord: Coord, seen: &mut HashMap<Coord, i32>) -> i32 {
    if let Some(&v) = seen.get(&coord) {
        return v;
    }
    let mut res = 0;
    for (x, y) in neighbors(coord) {
        let curr = if matrix
            .get(y)
            .and_then(|r| r.get(x))
            .is_some_and(|&v| v > matrix[coord.1][coord.0])
        {
            1 + dfs(matrix, (x, y), seen)
        } else {
            1
        };
        res = res.max(curr)
    }
    seen.insert(coord, res);
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            longest_increasing_path(&[&[9, 9, 4], &[6, 6, 8], &[2, 1, 1]]),
            4
        );
        debug_assert_eq!(
            longest_increasing_path(&[&[3, 4, 5], &[3, 2, 6], &[2, 2, 1]]),
            4
        );
        debug_assert_eq!(longest_increasing_path(&[&[1]]), 1);
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(i1: I1, i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: IntoIterator<Item = T1>,
        I2: IntoIterator<Item = T2>,
    {
        let mut v1: Vec<_> = i1.into_iter().collect();
        let mut v2: Vec<_> = i2.into_iter().collect();
        v1.sort_unstable();
        v2.sort_unstable();
        debug_assert_eq!(v1, v2);
    }
}
