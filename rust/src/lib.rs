mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_time_to_visit_all_points(points: &[[i32; 2]]) -> i32 {
    if points.len() <= 1 {
        return 0;
    }
    let [mut x1, mut y1] = [points[0][0], points[0][1]];
    let mut res = 0;
    for p in points[1..].iter() {
        let [x2, y2] = [p[0], p[1]];
        let dx = x1.abs_diff(x2);
        let dy = y1.abs_diff(y2);
        let min = dx.min(dy);
        let max = dx.max(dy);
        res += min + max - min;
        x1 = x2;
        y1 = y2;
    }
    res as _
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(min_time_to_visit_all_points(&[[1, 1], [3, 4], [-1, 0]]), 7);
        assert_eq!(min_time_to_visit_all_points(&[[3, 2], [-2, 2]]), 5);
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
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
