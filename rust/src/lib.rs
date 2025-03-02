mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn check_valid(matrix: Vec<Vec<i32>>) -> bool {
    use std::collections::HashSet;
    let n = matrix.len();
    let set: HashSet<_> = (1..=n as i32).collect();
    for r in matrix.iter() {
        if r.iter().copied().collect::<HashSet<_>>() != set {
            return false;
        }
    }
    for c in 0..n {
        if matrix.iter().map(|r| r[c]).collect::<HashSet<_>>() != set {
            return false;
        }
    }
    true
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
