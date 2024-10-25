mod helper;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn flipgame(fronts: &[i32], backs: &[i32]) -> i32 {
    let mut nums: Vec<i32> = fronts.iter().chain(backs.iter()).copied().collect();
    let dels: HashSet<_> = fronts
        .iter()
        .zip(backs.iter())
        .filter_map(|(a, b)| if a == b { Some(*a) } else { None })
        .collect();
    nums.retain(|n| !dels.contains(n));
    nums.sort_unstable();
    nums.first().copied().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(flipgame(&[1, 2, 4, 4, 7], &[1, 3, 4, 1, 3]), 2);
        debug_assert_eq!(flipgame(&[1], &[1]), 0);
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
