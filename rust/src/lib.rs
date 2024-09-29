mod helper;
mod trie;

use std::collections::{HashSet, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn find_circle_num(is_connected: &[&[i32]]) -> i32 {
    let n = is_connected.len();
    let mut seen = HashSet::new();
    let mut res = 0;
    for i in 0..n {
        if seen.contains(&i) {
            continue;
        }
        let mut queue = VecDeque::from([i]);
        while let Some(curr) = queue.pop_front() {
            if !seen.insert(curr) {
                continue;
            }
            for (j, &num) in is_connected[curr].iter().enumerate() {
                if num > 0 {
                    queue.push_back(j);
                }
            }
        }
        res += 1
    }
    debug_assert_eq!(seen.len(), n);
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(find_circle_num(&[&[1, 1, 0], &[1, 1, 0], &[0, 0, 1]]), 2);
        debug_assert_eq!(find_circle_num(&[&[1, 0, 0], &[0, 1, 0], &[0, 0, 1]]), 3);
    }

    #[test]
    fn test() {
        debug_assert_eq!(
            find_circle_num(&[&[1, 0, 0, 1], &[0, 1, 1, 0], &[0, 1, 1, 1], &[1, 0, 1, 1]]),
            1
        );
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
