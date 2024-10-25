mod helper;
mod trie;

use std::collections::{HashSet, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn racecar(target: i32) -> i32 {
    let mut queue = VecDeque::from([[0, 1, 0]]); // [pos, vel, dist]
    let mut seen = HashSet::from([[0, 1]]);
    while let Some([pos, vel, dist]) = queue.pop_front() {
        if pos == target {
            return dist;
        }
        if !(0..=2 * target).contains(&pos) {
            continue;
        }
        if seen.insert([pos + vel, 2 * vel]) {
            queue.push_back([pos + vel, 2 * vel, 1 + dist]);
        }
        let flip = -vel.signum();
        if seen.insert([pos, flip]) {
            queue.push_back([pos, flip, 1 + dist]);
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(racecar(3), 2);
        debug_assert_eq!(racecar(6), 5);
    }

    #[test]
    fn test() {
        debug_assert_eq!(racecar(4), 5);
    }

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
