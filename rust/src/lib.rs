mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::sync::atomic::{
    AtomicU8,
    Ordering::{Acquire, Release},
};
use std::time::Duration;

#[allow(unused_imports)]
use helper::*;

struct Foo {
    num: AtomicU8,
}

impl Foo {
    fn new() -> Self {
        Foo {
            num: AtomicU8::new(0),
        }
    }

    fn first<F>(&self, print_first: F)
    where
        F: FnOnce(),
    {
        print_first();
        self.num.store(1, Release);
    }

    fn second<F>(&self, print_second: F)
    where
        F: FnOnce(),
    {
        while self.num.load(Acquire) != 1 {
            std::thread::sleep(Duration::from_millis(5));
        }
        print_second();
        self.num.store(2, Release);
    }

    fn third<F>(&self, print_third: F)
    where
        F: FnOnce(),
    {
        while self.num.load(Acquire) != 2 {
            std::thread::sleep(Duration::from_millis(5));
        }
        print_third();
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
