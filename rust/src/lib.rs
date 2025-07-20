mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
    use itertools::Itertools;
    use std::collections::HashMap;
    let mut slopes = HashMap::new();
    let mut lines = HashMap::new();
    let mut mids = HashMap::new();
    let mut midlines = HashMap::new();
    for [p1, p2] in points.iter().array_combinations() {
        let [x1, y1] = p1[..] else { unreachable!() };
        let [x2, y2] = p2[..] else { unreachable!() };
        let mut dx = x2 - x1;
        let mut dy = y2 - y1;
        let gcd_ = gcd(dx.abs(), dy.abs());
        dx /= gcd_;
        dy /= gcd_;
        if dx < 0 || (dx == 0 && dy < 0) {
            dx *= -1;
            dy *= -1;
        }
        let intercept = dx * y1 - dy * x1;
        *slopes.entry([dx, dy]).or_insert(0) += 1;
        *lines.entry([dx, dy, intercept]).or_insert(0) += 1;
        *mids.entry([x1 + x2, y1 + y2]).or_insert(0) += 1;
        *midlines
            .entry([x1 + x2, y1 + y2, dx, dy, intercept])
            .or_insert(0) += 1;
    }
    // count_all(slopes) - count_all(segments on each line)
    // Then to subtract parallelograms
    // count_all(line with specific length) - count_all(specific length on line)
    let res =
        sum_up(slopes.into_values()) - sum_up(lines.into_values()) - sum_up(mids.into_values())
            + sum_up(midlines.into_values());
    res as i32
}

fn sum_up(it: impl Iterator<Item = usize>) -> usize {
    it.map(n_choose_2).sum()
}

const fn n_choose_2(n: usize) -> usize {
    n * (n - 1) / 2
}

const fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 { b } else { gcd(b % a, a) }
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
