mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn min_area_free_rect(points: &[[i32; 2]]) -> f64 {
    let points: Vec<_> = points.iter().map(|v| [v[0], v[1]].map(i64::from)).collect();
    let mut res = f64::MAX;
    let mut seen: HashMap<_, Vec<[i64; 2]>> = HashMap::new();
    for (idx, &[x1, y1]) in points.iter().enumerate() {
        for &[x2, y2] in points.iter().skip(1 + idx) {
            let center_x = x1 + x2; // divided by 2 is f64 => cannot be key of HashMap
            let center_y = y1 + y2;
            let diameter = (x1 - x2).pow(2) + (y1 - y2).pow(2); // same for sqrt() here
            for &[x3, y3] in seen
                .get(&(center_x, center_y, diameter))
                .map(|v| v.as_slice())
                .unwrap_or_default() 
            {
                let s1 = (x3 - x1).pow(2) + (y3 - y1).pow(2);
                let s2 = (x3 - x2).pow(2) + (y3 - y2).pow(2);
                res = res.min(((s1 * s2) as f64).sqrt());
            }
            seen.entry((center_x, center_y, diameter))
                .or_default()
                .push([x1, y1]);
        }
    }
    if res == f64::MAX {
        0.0
    } else {
        res
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        float_eq(
            min_area_free_rect(&[[1, 2], [2, 1], [1, 0], [0, 1]]),
            2.00000,
        );
        float_eq(
            min_area_free_rect(&[[0, 1], [2, 1], [1, 1], [1, 0], [2, 0]]),
            1.00000,
        );
        float_eq(
            min_area_free_rect(&[[0, 3], [1, 2], [3, 1], [1, 3], [2, 1]]),
            0.0,
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
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
