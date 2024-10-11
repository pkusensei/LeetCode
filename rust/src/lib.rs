mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn knight_probability(n: i32, k: i32, row: i32, col: i32) -> f64 {
    #[rustfmt::skip]
    const DELTAS: [[i32; 2]; 8] = [[2, 1],[1, 2],[2, -1],[-1, 2],
                                   [-2, 1],[1, -2],[-2, -1],[-1, -2],];
    let mut queue = HashMap::from([([row, col], 1.0)]);
    let mut res = 1.0;
    for _ in 0..k {
        res = 0.0;
        let mut next = HashMap::new();
        for (&[r, c], &p) in queue.iter() {
            for (nr, nc) in DELTAS
                .map(|d| (r + d[0], c + d[1]))
                .into_iter()
                .filter(|(nr, nc)| (0..n).contains(nr) && (0..n).contains(nc))
            {
                *next.entry([nr, nc]).or_insert(0.0) += p / 8.0;
                res += p / 8.0;
            }
        }
        queue = next
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(knight_probability(3, 2, 0, 0), 0.06250);
        debug_assert_eq!(knight_probability(1, 0, 0, 0), 1.00000);
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
