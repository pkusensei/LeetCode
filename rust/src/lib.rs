mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_distinct_freq_pair(mut nums: Vec<i32>) -> Vec<i32> {
    use itertools::Itertools;
    nums.sort_unstable();
    let chunks = nums.chunk_by(|a, b| a == b).collect_vec();
    for (i, ch1) in chunks.iter().enumerate() {
        for ch2 in chunks.iter().skip(1 + i) {
            if ch1.len() != ch2.len() {
                return vec![ch1[0], ch2[0]];
            }
        }
    }
    vec![-1, -1]
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
