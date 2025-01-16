mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn num_points(darts: &[[i32; 2]], r: i32) -> i32 {
    let mut res = 1;
    let r = f64::from(r);
    for (i1, p1) in darts.iter().enumerate() {
        let [x1, y1] = [0, 1].map(|i| f64::from(p1[i]));
        for p2 in darts.iter().skip(1 + i1) {
            let [x2, y2] = [0, 1].map(|i| f64::from(p2[i]));
            let dist = ((x1 - x2).powi(2) + (y1 - y2).powi(2)) / 4.0;
            if dist > r.powi(2) {
                continue;
            }
            let x0 = (x1 + x2) / 2.0 + (y2 - y1) * (r.powi(2) - dist).sqrt() / (dist * 4.0).sqrt();
            let y0 = (y1 + y2) / 2.0 - (x2 - x1) * (r.powi(2) - dist).sqrt() / (dist * 4.0).sqrt();
            let curr: i32 = darts
                .iter()
                .map(|p| {
                    let [a, b] = [0, 1].map(|i| f64::from(p[i]));
                    i32::from((a - x0).powi(2) + (b - y0).powi(2) <= r.powi(2) + 1e-5)
                })
                .sum();
            res = res.max(curr);
            let x0 = (x1 + x2) / 2.0 - (y2 - y1) * (r.powi(2) - dist).sqrt() / (dist * 4.0).sqrt();
            let y0 = (y1 + y2) / 2.0 + (x2 - x1) * (r.powi(2) - dist).sqrt() / (dist * 4.0).sqrt();
            let curr: i32 = darts
                .iter()
                .map(|p| {
                    let [a, b] = [0, 1].map(|i| f64::from(p[i]));
                    i32::from((a - x0).powi(2) + (b - y0).powi(2) <= r.powi(2) + 1e-5)
                })
                .sum();
            res = res.max(curr);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(num_points(&[[-2, 0], [2, 0], [0, 2], [0, -2]], 2), 4);
        assert_eq!(
            num_points(&[[-3, 0], [3, 0], [2, 6], [5, 4], [0, 9], [7, 8]], 5),
            5
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
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
