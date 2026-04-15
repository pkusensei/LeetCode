mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

pub fn closest_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
    let n = words.len() as i32;
    words
        .iter()
        .enumerate()
        .filter_map(|(i, s)| {
            if s == &target {
                let v = (i as i32 - start_index).abs();
                Some(v.min(n - v))
            } else {
                None
            }
        })
        .min()
        .unwrap_or(-1)
}

#[allow(unused_imports)]
use helper::*;

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
