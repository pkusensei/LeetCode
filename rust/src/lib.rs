mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn racecar(target: i32) -> i32 {
    use std::collections::{HashSet, VecDeque};
    let mut queue = VecDeque::from([[0, 1, 0]]);
    let mut seen = HashSet::from([[0, 1]]);
    while let Some([pos, speed, num]) = queue.pop_front() {
        if pos == target {
            return num;
        }
        if !(0..=2 * target).contains(&pos) {
            continue;
        }
        if seen.insert([pos + speed, 2 * speed]) {
            queue.push_back([pos + speed, 2 * speed, 1 + num]);
        }
        let rev = if speed > 0 { -1 } else { 1 };
        if seen.insert([pos, rev]) {
            queue.push_back([pos, rev, 1 + num]);
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
        assert_eq!(
            latest_day_to_cross(
                3,
                3,
                &[
                    [1, 2],
                    [2, 1],
                    [3, 3],
                    [2, 2],
                    [1, 1],
                    [1, 3],
                    [2, 3],
                    [3, 2],
                    [3, 1]
                ]
            ),
            3
        );
    }

    #[test]
    fn test() {}
}
