mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_boomerangs(points: &[[i32; 2]]) -> i32 {
    if points.len() < 3 {
        return 0;
    }
    let mut res = 0;
    for p1 in points.iter() {
        let curr: i32 = points
            .iter()
            .filter(|&p| p != p1)
            .fold(HashMap::new(), |mut acc, p2| {
                let d = (p1[0] - p2[0]).pow(2) + (p1[1] - p2[1]).pow(2);
                *acc.entry(d).or_insert(0) += 1;
                acc
            })
            .into_values()
            .map(|v| v * (v - 1))
            .sum();
        res += curr;
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(number_of_boomerangs(&[[0, 0], [1, 0], [2, 0]]), 2);
        debug_assert_eq!(number_of_boomerangs(&[[1, 1], [2, 2], [3, 3]]), 2);
        debug_assert_eq!(number_of_boomerangs(&[[1, 1]]), 0)
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
