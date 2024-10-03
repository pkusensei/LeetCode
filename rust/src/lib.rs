mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn valid_square(p1: [i32; 2], p2: [i32; 2], p3: [i32; 2], p4: [i32; 2]) -> bool {
    let ps = [p1, p2, p3, p4];
    let mut dist = std::collections::HashSet::new();
    for i in 0..4 {
        for j in i + 1..4 {
            dist.insert((ps[i][0] - ps[j][0]).pow(2) + (ps[i][1] - ps[j][1]).pow(2));
        }
    }
    dist.len() == 2 && dist.into_iter().all(|v| v > 0)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(valid_square([0, 0], [1, 1], [1, 0], [0, 1]));
        debug_assert!(!valid_square([0, 0], [1, 1], [1, 0], [0, 12]));
        debug_assert!(valid_square([1, 0], [-1, 0], [0, 1], [0, -1]));
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
