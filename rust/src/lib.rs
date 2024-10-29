mod dsu;
mod helper;
mod trie;

use std::collections::BTreeMap;

#[allow(unused_imports)]
use helper::*;

pub fn rectangle_area(rectangles: &[[i32; 4]]) -> i32 {
    const MOD: i64 = 1_000_000_007;
    // Align y values onto x's
    // and attach +1 to left side, -1 to right side
    // so that when a rectangle is "out of scope"
    // all of its vertices have count==0
    let rects: BTreeMap<i32, Vec<_>> = rectangles.iter().fold(BTreeMap::new(), |mut acc, v| {
        acc.entry(v[0]).or_default().push((v[1], v[3], 1));
        acc.entry(v[2]).or_default().push((v[1], v[3], -1));
        acc
    });
    let mut counts = BTreeMap::new();
    let mut prev = -1;
    let mut res = 0;
    for (x, ys) in rects {
        // Sweep thru left to right
        // When encountering x2, deal everything that's aligned on x1
        // |
        // |   |
        // |   |
        // |
        // x1  x2
        // Filling the "taller" area between x1 and x2
        // If the y vertices on x2 are all covered in overlap area
        // They have no impact on final number
        // since line sweeping only cares when total_count==0
        if prev >= 0 && x > prev {
            let mut bottom = -1;
            let mut total_count = 0;
            let mut y_range = 0;
            for (&y, &count) in counts.iter() {
                if count == 0 {
                    continue;
                }
                if bottom == -1 {
                    bottom = y;
                }
                total_count += count;
                if total_count == 0 {
                    y_range += y - bottom;
                    bottom = -1;
                }
            }
            res = (res + ((x - prev) as i64 * y_range as i64) % MOD) % MOD;
        }
        prev = x;
        for y in ys {
            *counts.entry(y.0).or_insert(0) += y.2;
            *counts.entry(y.1).or_insert(0) -= y.2;
        }
    }
    res as i32
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            rectangle_area(&[[0, 0, 2, 2], [1, 0, 2, 3], [1, 0, 3, 1]]),
            6
        );
        debug_assert_eq!(rectangle_area(&[[0, 0, 1000000000, 1000000000]]), 49);
    }

    #[test]
    fn test() {
        debug_assert_eq!(rectangle_area(&[[0, 0, 4, 4], [1, 1, 3, 3]]), 16);
    }

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }
}
