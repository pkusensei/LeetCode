mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn word_squares(words: &[&str]) -> Vec<Vec<String>> {
    use itertools::Itertools;
    use std::collections::HashSet;
    let mut res = HashSet::new();
    let words = words.iter().map(|s| s.as_bytes()).collect_vec();
    for (topi, top) in words.iter().enumerate() {
        let left = find_col(&words, top[0], 1 << topi);
        for (left_w, left_end, left_mask) in &left {
            let right = find_col(&words, top[3], *left_mask);
            for (right_w, right_end, right_mask) in &right {
                for (boti, botw) in words.iter().enumerate() {
                    if right_mask & (1 << boti) == 0
                        && botw.starts_with(&[*left_end])
                        && botw.ends_with(&[*right_end])
                    {
                        res.insert(
                            [top, left_w, right_w, botw]
                                .iter()
                                .map(|v| String::from_utf8(v.to_vec()).unwrap())
                                .collect(),
                        );
                    }
                }
            }
        }
    }
    res.into_iter().sorted_unstable().collect()
}

fn find_col<'a>(words: &[&'a [u8]], start: u8, mask: i32) -> Vec<(&'a [u8], u8, i32)> {
    let mut res = vec![];
    for (i, w) in words.iter().enumerate() {
        if mask & (1 << i) == 0 && w.starts_with(&[start]) {
            res.push((*w, w[3], mask | (1 << i)));
        }
    }
    res
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
            word_squares(&["able", "area", "echo", "also"]),
            [
                ["able", "area", "echo", "also"],
                ["area", "able", "also", "echo"]
            ]
        );
        assert!(word_squares(&["code", "cafe", "eden", "edge"]).is_empty());
    }

    #[test]
    fn test() {}
}
