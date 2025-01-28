mod dsu;
mod helper;
mod trie;

use std::collections::VecDeque;

#[allow(unused_imports)]
use helper::*;

#[derive(Debug, Clone, Default)]
struct FrontMiddleBackQueue {
    fst: VecDeque<i32>,
    snd: VecDeque<i32>,
}

impl FrontMiddleBackQueue {
    fn new() -> Self {
        Default::default()
    }

    fn push_front(&mut self, val: i32) {
        self.fst.push_front(val);
        if self.fst.len() > self.snd.len() {
            self.snd.push_front(self.fst.pop_back().unwrap());
        }
    }

    fn push_middle(&mut self, val: i32) {
        if self.fst.len() < self.snd.len() {
            self.fst.push_back(val);
        } else {
            self.snd.push_front(val);
        }
    }

    fn push_back(&mut self, val: i32) {
        self.snd.push_back(val);
        if self.fst.len() + 1 < self.snd.len() {
            self.fst.push_back(self.snd.pop_front().unwrap());
        }
    }

    fn pop_front(&mut self) -> i32 {
        let res = self.fst.pop_front().or_else(||self.snd.pop_front()).unwrap_or(-1);
        if self.fst.len() + 1 < self.snd.len() {
            self.fst.push_back(self.snd.pop_front().unwrap());
        }
        res
    }

    fn pop_middle(&mut self) -> i32 {
        if self.fst.len() == self.snd.len() {
            self.fst.pop_back().unwrap_or(-1)
        } else {
            self.snd.pop_front().unwrap_or(-1)
        }
    }

    fn pop_back(&mut self) -> i32 {
        let res = self.snd.pop_back().unwrap_or(-1);
        if self.fst.len() > self.snd.len() {
            self.snd.push_front(self.fst.pop_back().unwrap());
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    #[test]
    fn basics() {
        let mut q = FrontMiddleBackQueue::new();
        q.push_front(1); // [1]
        q.push_back(2); // [1, 2]
        q.push_middle(3); // [1, 3, 2]
        q.push_middle(4); // [1, 4, 3, 2]
        assert_eq!(q.pop_front(), 1); // return 1 -> [4, 3, 2]
        assert_eq!(q.pop_middle(), 3); // return 3 -> [4, 2]
        assert_eq!(q.pop_middle(), 4); // return 4 -> [2]
        assert_eq!(q.pop_back(), 2); // return 2 -> []
        assert_eq!(q.pop_front(), -1); // return -1
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
