mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn judge_circle(moves: &str) -> bool {
    moves.len() & 1 == 0
        && moves.bytes().fold([0, 0], |mut acc, b| {
            match b {
                b'U' => acc[0] += 1,
                b'D' => acc[0] -= 1,
                b'R' => acc[1] += 1,
                b'L' => acc[1] -= 1,
                _ => (),
            };
            acc
        }) == [0, 0]
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(judge_circle("UD"));
        debug_assert!(!judge_circle("LL"));
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
