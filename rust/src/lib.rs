mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::sync::{Condvar, Mutex};

#[allow(unused_imports)]
use helper::*;

struct ZeroEvenOdd {
    n: i32,
    state: Mutex<(bool, i32)>,
    cv: Condvar,
}

impl ZeroEvenOdd {
    fn new(n: i32) -> Self {
        ZeroEvenOdd {
            n,
            state: Mutex::new((true, 1)),
            cv: Condvar::new(),
        }
    }

    // printNumber(x) prints the integer x
    fn zero<F>(&self, print_number: F)
    where
        F: Fn(i32),
    {
        for _ in 0..self.n {
            let mut lock = self
                .cv
                .wait_while(self.state.lock().unwrap(), |v| !v.0)
                .unwrap();
            print_number(0);
            lock.0 = false;
            self.cv.notify_all();
        }
    }

    fn even<F>(&self, print_number: F)
    where
        F: Fn(i32),
    {
        loop {
            let mut lock = self.state.lock().unwrap();
            while lock.1 & 1 == 1 || lock.0 {
                if lock.1 > self.n {
                    return;
                }
                lock = self.cv.wait(lock).unwrap();
            }
            if lock.1 > self.n {
                return;
            }
            print_number(lock.1);
            lock.0 = true;
            lock.1 += 1;
            self.cv.notify_all();
        }
    }

    fn odd<F>(&self, print_number: F)
    where
        F: Fn(i32),
    {
        loop {
            let mut lock = self.state.lock().unwrap();
            while lock.1 & 1 == 0 || lock.0 {
                if lock.1 > self.n {
                    return;
                }
                lock = self.cv.wait(lock).unwrap();
            }
            if lock.1 > self.n {
                return;
            }
            print_number(lock.1);
            lock.0 = true;
            lock.1 += 1;
            self.cv.notify_all();
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {{
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        }};
    }

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        }};
    }

    #[test]
    fn basics() {}

    #[test]
    fn test() {}
}
