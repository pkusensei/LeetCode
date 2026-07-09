mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::sync::atomic::{
    AtomicBool,
    Ordering::{Acquire, Release},
};
use std::time::Duration;

#[allow(unused_imports)]
use helper::*;

struct FooBar {
    flag: AtomicBool,
    n: usize,
}

impl FooBar {
    fn new(n: usize) -> Self {
        FooBar {
            flag: AtomicBool::new(false),
            n,
        }
    }

    fn foo<F>(&self, print_foo: F)
    where
        F: Fn(),
    {
        for _ in 0..self.n {
            while self.flag.load(Acquire) {
                std::thread::sleep(Duration::from_millis(1));
            }
            // printFoo() outputs "foo". Do not change or remove this line.
            print_foo();
            self.flag.store(true, Release);
        }
    }

    fn bar<F>(&self, print_bar: F)
    where
        F: Fn(),
    {
        for _ in 0..self.n {
            while !self.flag.load(Acquire) {
                std::thread::sleep(Duration::from_millis(1));
            }
            // printBar() outputs "bar". Do not change or remove this line.
            print_bar();
            self.flag.store(false, Release);
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
