mod helper;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn is_rectangle_cover(rectangles: &[[i32; 4]]) -> bool {
    let area = rectangles
        .iter()
        .map(|&[x1, y1, x2, y2]| (x2 - x1) * (y2 - y1))
        .sum::<i32>();
    let xmin = rectangles.iter().map(|a| a[0]).min().unwrap();
    let xmax = rectangles.iter().map(|a| a[2]).max().unwrap();
    let ymin = rectangles.iter().map(|a| a[1]).min().unwrap();
    let ymax = rectangles.iter().map(|a| a[3]).max().unwrap();
    if (ymax - ymin) * (xmax - xmin) != area {
        return false;
    }
    rectangles
        .iter()
        .fold(HashMap::new(), |mut acc, &[x1, y1, x2, y2]| {
            *acc.entry([x1, y1]).or_insert(0) += 1;
            *acc.entry([x1, y2]).or_insert(0) += 1;
            *acc.entry([x2, y1]).or_insert(0) += 1;
            *acc.entry([x2, y2]).or_insert(0) += 1;
            acc
        })
        .into_iter()
        .all(|(point, count)| {
            if [[xmin, ymin], [xmin, ymax], [xmax, ymin], [xmax, ymax]].contains(&point) {
                count == 1
            } else {
                count & 1 == 0
            }
        })
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(is_rectangle_cover(&[
            [1, 1, 3, 3],
            [3, 1, 4, 2],
            [3, 2, 4, 4],
            [1, 3, 2, 4],
            [2, 3, 3, 4]
        ]));
        debug_assert!(!is_rectangle_cover(&[
            [1, 1, 2, 3],
            [1, 3, 2, 4],
            [3, 1, 4, 2],
            [3, 2, 4, 4]
        ]));
        debug_assert!(!is_rectangle_cover(&[
            [1, 1, 3, 3],
            [3, 1, 4, 2],
            [1, 3, 2, 4],
            [2, 2, 4, 4]
        ]));
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(i1: I1, i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: IntoIterator<Item = T1>,
        I2: IntoIterator<Item = T2>,
    {
        let mut v1: Vec<_> = i1.into_iter().collect();
        let mut v2: Vec<_> = i2.into_iter().collect();
        v1.sort_unstable();
        v2.sort_unstable();
        debug_assert_eq!(v1, v2);
    }
}
