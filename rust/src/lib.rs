mod dsu;
mod helper;
mod trie;

use std::collections::BTreeSet;

#[allow(unused_imports)]
use helper::*;

#[derive(Debug, Clone)]
struct ExamRoom {
    nums: BTreeSet<i32>,
    n: i32,
}

impl ExamRoom {
    fn new(n: i32) -> Self {
        Self {
            nums: BTreeSet::new(),
            n,
        }
    }

    fn seat(&mut self) -> i32 {
        if self.nums.is_empty() {
            self.nums.insert(0);
            return 0;
        }
        let (mut width, mut res) = (0, 0);
        if let Some(&v) = self.nums.first() {
            if v > 0 {
                width = v;
                res = 0;
            }
        }
        let mut prev = -1;
        for &num in self.nums.iter() {
            if prev == -1 {
                prev = num;
                continue;
            }
            if (num - prev) / 2 > width {
                width = (num - prev) / 2;
                res = prev + 1 + (num - 1 - prev - 1) / 2; // this trips me up
            }
            prev = num;
        }
        if let Some(&v) = self.nums.last() {
            if self.n - 1 - v > width {
                // width = self.n - 1 - v;
                res = self.n - 1;
            }
        }
        self.nums.insert(res);
        res
    }

    fn leave(&mut self, p: i32) {
        self.nums.remove(&p);
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        let mut room = ExamRoom::new(10);
        debug_assert_eq!(room.seat(), 0); // return 0, no one is in the room, then the student sits at seat number 0.
        debug_assert_eq!(room.seat(), 9); // return 9, the student sits at the last seat number 9.
        debug_assert_eq!(room.seat(), 4); // return 4, the student sits at the last seat number 4.
        debug_assert_eq!(room.seat(), 2); // return 2, the student sits at the last seat number 2.
        room.leave(4);
        debug_assert_eq!(room.seat(), 5); // return 5, the student sits at the last seat number 5.
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
