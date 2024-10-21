mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn escape_ghosts(ghosts: &[[i32; 2]], target: [i32; 2]) -> bool {
    let dist = manhattan_dist(0, 0, target[0], target[1]);
    ghosts
        .iter()
        .all(|n| manhattan_dist(n[0], n[1], target[0], target[1]) > dist)
}

const fn manhattan_dist(x1: i32, y1: i32, x2: i32, y2: i32) -> u32 {
    x1.abs_diff(x2) + y1.abs_diff(y2)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(escape_ghosts(&[[1, 0], [0, 3]], [0, 1]));
        debug_assert!(!escape_ghosts(&[[1, 0]], [2, 0]));
        debug_assert!(!escape_ghosts(&[[2, 0]], [1, 0]));
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
