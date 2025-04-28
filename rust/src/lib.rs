mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_distance(points: &[[i32; 2]]) -> i32 {
    use std::collections::BTreeMap;
    let mut coords = vec![];
    let mut a_map = BTreeMap::new();
    let mut b_map = BTreeMap::new();
    for [a, b] in points.iter().map(|p| {
        let [x, y] = p[..] else { unreachable!() };
        [x - y, x + y]
    }) {
        coords.push([a, b]);
        *a_map.entry(a).or_insert(0) += 1;
        *b_map.entry(b).or_insert(0) += 1;
    }
    let mut res = i32::MAX;
    let dec = |v: &mut i32| *v -= 1;
    let inc = |v: &mut i32| *v += 1;
    for [a, b] in coords {
        a_map.entry(a).and_modify(dec);
        b_map.entry(b).and_modify(dec);
        let [a_min, b_min] = [&a_map, &b_map].map(|m| {
            m.iter()
                .skip_while(|(_, v)| **v == 0)
                .next()
                .map(|(k, _)| *k)
                .unwrap_or(0)
        });
        let [a_max, b_max] = [&a_map, &b_map].map(|m| {
            m.iter()
                .rev()
                .skip_while(|(_, v)| **v == 0)
                .next()
                .map(|(k, _)| *k)
                .unwrap_or(0)
        });
        res = res.min((a_max - a_min).max(b_max - b_min));
        a_map.entry(a).and_modify(inc);
        b_map.entry(b).and_modify(inc);
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
        assert_eq!(minimum_distance(&[[3, 10], [5, 15], [10, 2], [4, 4]]), 12);
        assert_eq!(minimum_distance(&[[1, 1], [1, 1], [1, 1]]), 0);
    }

    #[test]
    fn test() {}
}
