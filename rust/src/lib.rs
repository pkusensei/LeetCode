mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_active_sections_after_trade(s: String) -> i32 {
        use itertools::Itertools;
        let nums = s
            .as_bytes()
            .chunk_by(|a, b| a == b)
            .map(|ch| {
                if ch[0] == b'1' {
                    ch.len() as i32
                } else {
                    -(ch.len() as i32)
                }
            })
            .collect_vec();
        let mut max_window = 0;
        for w in nums.windows(3).filter(|w| w[1] > 0) {
            max_window = max_window.max((w[0] + w[2]).abs());
        }
        s.bytes().map(|b| i32::from(b - b'0')).sum::<i32>() + max_window
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
