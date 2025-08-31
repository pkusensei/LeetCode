mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_operations(s: &str, k: i32) -> i32 {
    use std::collections::{BTreeSet, VecDeque};
    let n = s.len() as i32;
    let zeros = s.bytes().filter(|&v| v == b'0').count() as i32;
    let mut queue = VecDeque::from([(zeros, 0)]);
    let mut odd_set: BTreeSet<_> = (0..=n).filter(|&v| v != zeros && v & 1 == 1).collect();
    let mut even_set: BTreeSet<_> = (0..=n).filter(|&v| v != zeros && v & 1 == 0).collect();
    while let Some((zeros, step)) = queue.pop_front() {
        if zeros == 0 {
            return step;
        }
        let min_ones = (k - zeros).max(0);
        let max_ones = k.min(n - zeros);
        let lower = zeros + 2 * min_ones - k;
        let upper = zeros + 2 * max_ones - k;
        let set = if lower & 1 == 1 {
            &mut odd_set
        } else {
            &mut even_set
        };
        let mut del = vec![];
        for &v in set.range(lower..=upper) {
            queue.push_back((v, 1 + step));
            del.push(v);
        }
        for v in del {
            set.remove(&v);
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
        assert_eq!(min_operations("110", 1), 1);
        assert_eq!(min_operations("0101", 3), 2);
        assert_eq!(min_operations("101", 2), -1);
    }

    #[test]
    fn test() {}
}
