mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;
use itertools::izip;

pub fn buddy_strings(s: String, goal: String) -> bool {
    if s.len() != goal.len() {
        return false;
    }
    if s == goal {
        let mut seen = 0;
        for b in s.bytes() {
            let i = b - b'a';
            if seen & (1 << i) > 0 {
                return true;
            }
            seen |= 1 << i;
        }
        false
    } else {
        let mut it = izip!(s.bytes(), goal.bytes()).filter(|(a, b)| a != b);
        let (Some(a), Some(b)) = (it.next(), it.next()) else {
            return false;
        };
        it.next().is_none() && a.0 == b.1 && a.1 == b.0
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
