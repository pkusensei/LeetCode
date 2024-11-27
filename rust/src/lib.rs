mod dsu;
mod helper;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn num_tile_possibilities(tiles: &str) -> i32 {
    let mut res = HashSet::new();
    backtrack(
        tiles.as_bytes(),
        &mut Vec::with_capacity(tiles.len()),
        &mut res,
    );
    res.len() as i32
}

fn backtrack(tiles: &[u8], curr: &mut Vec<u8>, res: &mut HashSet<Vec<u8>>) {
    match tiles {
        [] => {
            if !curr.is_empty() {
                res.insert(curr.clone());
            }
        }
        [head, tail @ ..] => {
            backtrack(tail, curr, res);
            let n = curr.len();
            for i in 0..=n {
                curr.insert(i, *head);
                backtrack(tail, curr, res);
                curr.remove(i);
            }
        }
    }
}

fn with_count(tiles: &str) -> i32 {
    fn dfs(count: &mut [i32; 26]) -> i32 {
        let mut res = 0;
        for i in 0..26 {
            if count[i] > 0 {
                count[i] -= 1;
                // res+1 => use current letter as single length str
                // res+dfs(count) => dfs to build on current config
                res += 1 + dfs(count);
                // backtracking
                count[i] += 1;
            }
        }
        res
    }

    let mut count = tiles.bytes().fold([0; 26], |mut acc, b| {
        acc[usize::from(b - b'A')] += 1;
        acc
    });
    dfs(&mut count)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(with_count("AAB"), 8);
        debug_assert_eq!(with_count("AAABBC"), 188);
        debug_assert_eq!(with_count("V"), 1);
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
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
