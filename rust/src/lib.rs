mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn is_rectangle_overlap(rec1: [i32; 4], rec2: [i32; 4]) -> bool {
    overlap(rec1[0], rec1[2], rec2[0], rec2[2]) && overlap(rec1[1], rec1[3], rec2[1], rec2[3])
}

fn overlap(x1: i32, x2: i32, x3: i32, x4: i32) -> bool {
    !(x2 <= x3 || x4 <= x1)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(is_rectangle_overlap([0, 0, 2, 2], [1, 1, 3, 3]));
        debug_assert!(!is_rectangle_overlap([0, 0, 1, 1], [1, 0, 2, 1]));
        debug_assert!(!is_rectangle_overlap([0, 0, 1, 1], [2, 2, 3, 3]));
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
