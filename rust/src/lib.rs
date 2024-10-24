mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn largest_triangle_area(points: &[[i32; 2]]) -> f64 {
    let n = points.len();
    let mut res = 0f64;
    for i1 in 0..n {
        for i2 in i1..n {
            for i3 in i2..n {
                let (a, b, c) = (points[i1], points[i2], points[i3]);
                let r = a[0] * (b[1] - c[1]) + b[0] * (c[1] - a[1]) + c[0] * (a[1] - b[1]);
                res = res.max(0.5 * f64::from(r.abs()));
            }
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
        debug_assert_eq!(
            largest_triangle_area(&[[0, 0], [0, 1], [1, 0], [0, 2], [2, 0]]),
            2.0
        );
        debug_assert_eq!(largest_triangle_area(&[[1, 0], [0, 0], [0, 1]]), 0.5);
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
}
