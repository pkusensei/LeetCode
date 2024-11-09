mod dsu;
mod helper;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn min_area_rect(points: &[[i32; 2]]) -> i32 {
    let points: HashSet<_> = points.iter().map(|p| [p[0], p[1]]).collect();
    let mut res = None::<i32>;
    for (i, &p1) in points.iter().enumerate() {
        let [x1, y1] = p1;
        for &p2 in points.iter().skip(1 + i) {
            let [x2, y2] = p2;
            if x1 != x2 && y1 != y2 && points.contains(&[x1, y2]) && points.contains(&[x2, y1]) {
                let temp = ((x1 - x2) * (y1 - y2)).abs();
                if let Some(ref mut v) = res {
                    *v = (*v).min(temp)
                } else {
                    res = Some(temp);
                }
            }
        }
    }
    res.unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(min_area_rect(&[[1, 1], [1, 3], [3, 1], [3, 3], [2, 2]]), 4);
        debug_assert_eq!(
            min_area_rect(&[[1, 1], [1, 3], [3, 1], [3, 3], [4, 1], [4, 3]]),
            2
        );
    }

    #[test]
    fn test() {}

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

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64>,
        T2: Into<f64>,
    {
        const EP: f64 = 1e-5;
        debug_assert!((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP);
    }
}
