mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_length(s: &str, num_ops: i32) -> i32 {
    let mut left = 1;
    let mut right = s.len();
    while left < right {
        let mid = left.midpoint(right);
        if check(s.as_bytes(), num_ops as usize, mid) {
            right = mid;
        } else {
            left = 1 + mid;
        }
    }
    left as i32
}

fn check(s: &[u8], ops: usize, mid: usize) -> bool {
    if mid == 1 {
        let sum: usize = s
            .iter()
            .enumerate()
            .map(|(i, &b)| (i & 1 ^ usize::from(b & 1)))
            .sum();
        sum.min(s.len() - sum) <= ops
    } else {
        s.chunk_by(|a, b| a == b)
            .map(|ch| ch.len() / (1 + mid))
            .sum::<usize>()
            <= ops
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
    fn basics() {
        assert_eq!(min_length("000001", 1), 2);
        assert_eq!(min_length("0000", 2), 1);
        assert_eq!(min_length("0101", 0), 1);
    }

    #[test]
    fn test() {
        assert_eq!(min_length("1001", 1), 2);
    }
}
