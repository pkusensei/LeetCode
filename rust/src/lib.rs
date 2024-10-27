mod dsu;
mod helper;
mod trie;

use std::collections::{HashSet, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn can_visit_all_rooms(rooms: &[&[i32]]) -> bool {
    let mut queue = VecDeque::from([0]);
    let mut seen = HashSet::from([0]);
    while let Some(curr) = queue.pop_front() {
        for &num in rooms[curr] {
            if seen.insert(num) {
                queue.push_back(num as usize);
            }
        }
    }
    seen.len() == rooms.len()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(can_visit_all_rooms(&[&[1], &[2], &[3], &[]]));
        debug_assert!(!can_visit_all_rooms(&[&[1, 3], &[3, 0, 1], &[2], &[0]]));
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
