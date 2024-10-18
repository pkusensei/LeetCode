mod helper;
mod trie;

use std::collections::{HashSet, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn open_lock(deadends: &[&str], target: &str) -> i32 {
    let ends: HashSet<_> = deadends.iter().map(|s| s.as_bytes()).collect();
    let target = target.as_bytes();
    let start = [b'0'; 4];
    let mut queue = VecDeque::from([(start, 0)]);
    let mut seen = HashSet::from([start]);
    while let Some((curr, dist)) = queue.pop_front() {
        if ends.contains(curr.as_ref()) {
            continue;
        }
        if curr == target {
            return dist;
        }
        for next in step(curr) {
            if seen.insert(next) {
                queue.push_back((next, 1 + dist));
            }
        }
    }
    -1
}

fn step(start: [u8; 4]) -> impl Iterator<Item = [u8; 4]> {
    (0..8).map(move |n| {
        let mut curr = start;
        let idx = n / 2;
        let b = curr[idx] - b'0';
        if n & 1 == 0 {
            curr[idx] = b'0' + b.checked_sub(1).unwrap_or(9);
        } else {
            curr[idx] = b'0' + (b + 1) % 10;
        }
        curr
    })
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        // debug_assert_eq!(
        //     open_lock(&["0201", "0101", "0102", "1212", "2002"], "0202"),
        //     6
        // );
        debug_assert_eq!(open_lock(&["8888"], "0009"), 1);
        debug_assert_eq!(
            open_lock(
                &["8887", "8889", "8878", "8898", "8788", "8988", "7888", "9888"],
                "8888"
            ),
            -1
        );
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
