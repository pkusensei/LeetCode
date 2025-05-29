mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_rectangle_area(points: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashSet;
    let set: HashSet<_> = points.iter().map(|p| [p[0], p[1]]).collect();
    let mut res = 0;
    for (i, p1) in points.iter().enumerate() {
        let [x1, y1] = p1[..] else { unreachable!() };
        for p2 in points.iter().skip(1 + i) {
            let [x2, y2] = p2[..] else { unreachable!() };
            if x1 == x2 || y1 == y2 {
                continue;
            };
            if set.contains(&[x1, y2])
                && set.contains(&[x2, y1])
                && set.iter().all(|&[x, y]| {
                    [[x1, y1], [x2, y2], [x2, y1], [x1, y2]].contains(&[x, y])
                        || ((x - x1) * (x - x2) > 0 || (y - y1) * (y - y2) > 0)
                })
            {
                res = res.max((x1 - x2).abs() * (y1 - y2).abs());
            }
        }
    }
    if res > 0 { res } else { -1 }
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
