mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_points_inside_square(points: &[[i32; 2]], s: &str) -> i32 {
    use itertools::{Itertools, izip};
    use std::collections::BTreeMap;
    let arr = izip!(points.iter(), s.bytes())
        .map(|(p, b)| {
            let [x, y] = p[..] else { unreachable!() };
            (x.abs().max(y.abs()), b - b'a')
        })
        .sorted_unstable_by_key(|&(dist, _tag)| dist)
        .collect_vec();
    let mut map = BTreeMap::new();
    let mut mask: i32 = 0;
    for (i, &(dist, tag)) in arr.iter().enumerate() {
        mask |= 1 << tag;
        map.insert(dist, (mask, 1 + i as u32));
    }
    let mut res = 0;
    for &(mask, count) in map.values() {
        if mask.count_ones() == count {
            res = res.max(count)
        } else {
            break;
        }
    }
    res as i32
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
            max_points_inside_square(&[[2, 2], [-1, -2], [-4, 4], [-3, 1], [3, -3]], "abdca"),
            2
        );
        assert_eq!(
            max_points_inside_square(&[[1, 1], [-2, -2], [-2, 2]], "abb"),
            1
        );
        assert_eq!(
            max_points_inside_square(&[[1, 1], [-1, -1], [2, -2]], "ccd"),
            0
        );
    }

    #[test]
    fn test() {}
}
