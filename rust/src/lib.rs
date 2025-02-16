mod dsu;
mod helper;
mod trie;

use std::collections::BTreeMap;

#[allow(unused_imports)]
use helper::*;

pub fn split_painting(segments: &[[i32; 3]]) -> Vec<Vec<i64>> {
    let mut map = BTreeMap::new();
    for seg in segments.iter() {
        let [start, end, color] = seg[..] else {
            unreachable!()
        };
        *map.entry(start).or_insert(0) += i64::from(color);
        *map.entry(end).or_insert(0) -= i64::from(color);
    }
    let mut prefix = 0;
    for v in map.values_mut() {
        *v += prefix;
        prefix = *v;
    }
    let mut res = vec![];
    let (mut prev_node, mut prev_color) = map.pop_first().unwrap();
    for (node, color) in map {
        if prev_color != 0 {
            res.push(vec![i64::from(prev_node), i64::from(node), prev_color]);
        }
        prev_node = node;
        prev_color = color;
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
        assert_eq!(
            split_painting(&mut [[1, 4, 5], [4, 7, 7], [1, 7, 9]]),
            [[1, 4, 14], [4, 7, 16]]
        );
        assert_eq!(
            split_painting(&mut [[1, 7, 9], [6, 8, 15], [8, 10, 7]]),
            [[1, 6, 9], [6, 7, 24], [7, 8, 15], [8, 10, 7]]
        );
        assert_eq!(
            split_painting(&mut [[1, 4, 5], [1, 4, 7], [4, 7, 1], [4, 7, 11]]),
            [[1, 4, 12], [4, 7, 12]]
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            split_painting(&[
                [4, 16, 12],
                [9, 10, 15],
                [18, 19, 13],
                [3, 13, 20],
                [12, 16, 3],
                [2, 10, 10],
                [3, 11, 4],
                [13, 16, 6]
            ]),
            [
                [2, 3, 10],
                [3, 4, 34],
                [4, 9, 46],
                [9, 10, 61],
                [10, 11, 36],
                [11, 12, 32],
                [12, 13, 35],
                [13, 16, 21],
                [18, 19, 13]
            ]
        );
    }
}
