mod dsu;
mod helper;
mod trie;

use std::collections::{HashSet, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn minimum_operations_to_make_equal(x: i32, y: i32) -> i32 {
        if x <= y {
            return y - x;
        }
        let upper = x + x - y;
        let mut seen = HashSet::from([x]);
        let mut queue = VecDeque::from([(x, 0)]);
        while let Some((num, step)) = queue.pop_front() {
            if num == y {
                return step.min(x - y);
            }
            if num % 11 == 0 {
                let val = num / 11;
                if (1..=upper).contains(&val) && seen.insert(val) {
                    queue.push_back((val, 1 + step));
                }
            }
            if num % 5 == 0 {
                let val = num / 5;
                if (1..=upper).contains(&val) && seen.insert(val) {
                    queue.push_back((val, 1 + step));
                }
            }
            for d in [-1, 1] {
                let val = num + d;
                if (1..=upper).contains(&val) && seen.insert(val) {
                    queue.push_back((val, 1 + step));
                }
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
    fn basics() {
        assert_eq!(minimum_operations_to_make_equal(26, 1), 3);
        assert_eq!(minimum_operations_to_make_equal(54, 2), 4);
        assert_eq!(minimum_operations_to_make_equal(25, 30), 5);
    }

    #[test]
    fn test() {}
}
