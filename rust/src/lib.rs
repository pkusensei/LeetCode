mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn can_reach(s: &str, min_jump: i32, max_jump: i32) -> bool {
    use std::collections::VecDeque;
    let (s, n) = (s.as_bytes(), s.len());
    if s[n - 1] == b'1' {
        return false;
    }
    let [minj, maxj] = [min_jump, max_jump].map(|v| v as usize);
    let mut queue = VecDeque::from([0]);
    let mut prev = 0;
    while let Some(node) = queue.pop_front() {
        if node == n - 1 {
            return true;
        }
        let left = node + minj;
        let right = (node + maxj).min(n - 1);
        for i in left.max(1 + prev)..=right {
            if s[i] == b'0' {
                queue.push_back(i);
            }
        }
        prev = prev.max(right);
    }
    false
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
        assert!(!can_reach("00111010", 3, 5));
    }

    #[test]
    fn test() {}
}
