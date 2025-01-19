mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn get_min_dist_sum(positions: &[[i32; 2]]) -> f64 {
    let n = positions.len() as f64;
    let [mut xc, mut yc] = positions
        .iter()
        .fold([0, 0], |acc, p| [acc[0] + p[0], acc[1] + p[1]])
        .map(|v| f64::from(v) / n);
    let mut res = f64::MAX;
    let mut step = 100.0;
    while step > 1e-6 {
        let mut zoom = true;
        for [dx, dy] in [[-1, 0], [1, 0], [0, -1], [0, 1]] {
            let nx = xc + step * f64::from(dx);
            let ny = yc + step * f64::from(dy);
            let dist = positions
                .iter()
                .map(|p| {
                    let [x, y] = [p[0], p[1]].map(f64::from);
                    ((x - nx).powi(2) + (y - ny).powi(2)).sqrt()
                })
                .sum::<f64>();
            if dist < res {
                res = dist;
                [xc, yc] = [nx, ny];
                zoom = false;
                break;
            }
        }
        if zoom {
            step /= 2.0;
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
        float_eq(get_min_dist_sum(&[[0, 1], [1, 0], [1, 2], [2, 1]]), 4.0);
        float_eq(get_min_dist_sum(&[[1, 1], [3, 3]]), 2.82843);
    }

    #[test]
    fn test() {
        float_eq(get_min_dist_sum(&[[1, 1], [0, 0], [2, 0]]), 2.73205);
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
