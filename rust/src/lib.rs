mod dsu;
mod helper;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

struct Bitset {
    size: i32,
    bits: HashSet<i32>,
    unset: HashSet<i32>,
}

impl Bitset {
    fn new(size: i32) -> Self {
        Self {
            size,
            bits: HashSet::with_capacity(size as usize),
            unset: (0..size).collect(),
        }
    }

    fn fix(&mut self, idx: i32) {
        self.bits.insert(idx);
        self.unset.remove(&idx);
    }

    fn unfix(&mut self, idx: i32) {
        self.bits.remove(&idx);
        self.unset.insert(idx);
    }

    fn flip(&mut self) {
        std::mem::swap(&mut self.bits, &mut self.unset);
    }

    fn all(&self) -> bool {
        self.bits.len() == self.size as usize
    }

    fn one(&self) -> bool {
        self.bits.len() >= 1
    }

    fn count(&self) -> i32 {
        self.bits.len() as _
    }

    fn to_string(&self) -> String {
        (0..self.size)
            .map(|i| if self.bits.contains(&i) { '1' } else { '0' })
            .collect()
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

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
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
