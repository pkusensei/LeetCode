mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_operations(s: &str, k: i32) -> i32 {
    use std::collections::{BTreeSet, VecDeque};
    let n = s.len();
    let k = k as usize;
    let zeros = s.bytes().filter(|&b| b == b'0').count();
    let [mut odds, mut evens] = [const { BTreeSet::new() }; 2];
    for i in (0..=n).filter(|&v| v != zeros) {
        if i & 1 == 1 {
            odds.insert(i);
        } else {
            evens.insert(i);
        }
    }
    let mut queue = VecDeque::from([(zeros, 0)]);
    while let Some((zeros, step)) = queue.pop_front() {
        if zeros == 0 {
            return step;
        }
        let min_ones = k.saturating_sub(zeros); // k-z
        let max_ones = k.min(n - zeros); // min(k, n-z)
        // z+2*ones-k
        let [low, high] = [min_ones, max_ones].map(|v| zeros + 2 * v - k);
        let set = if low & 1 == 1 { &mut odds } else { &mut evens };
        let mut del = vec![];
        for &i in set.range(low..=high) {
            queue.push_back((i, 1 + step));
            del.push(i);
        }
        for i in del {
            set.remove(&i);
        }
    }
    -1
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
    fn basics() {
        assert_eq!(min_operations("101", 2), -1);
    }

    #[test]
    fn test() {}
}
