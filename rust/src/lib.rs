mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashMap;
    let mut freq = HashMap::new();
    for (r1, row1) in img1.iter().enumerate() {
        for (c1, &v) in row1.iter().enumerate() {
            if v == 1 {
                for (r2, row2) in img2.iter().enumerate() {
                    for (c2, &v2) in row2.iter().enumerate() {
                        if v2 == 1 {
                            let [r1, c1, r2, c2] = [r1, c1, r2, c2].map(|v| v as i32);
                            *freq.entry([r1 - r2, c1 - c2]).or_insert(0) += 1;
                        }
                    }
                }
            }
        }
    }
    freq.into_values().max().unwrap_or(0)
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
