mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn grid_illumination(_n: i32, lamps: &[[i32; 2]], queries: &[[i32; 2]]) -> Vec<i32> {
    let mut lights: Vec<_> = lamps.iter().map(|v| [v[0], v[1]]).collect();
    let mut res = Vec::with_capacity(queries.len());
    for _q in queries.iter() {
        let [qx, qy] = [_q[0], _q[1]];
        res.push(i32::from(is_lit(&lights, qx, qy)));
        lights.retain(|&[x, y]| x.abs_diff(qx) > 1 || y.abs_diff(qy) > 1);
    }
    res
}

fn is_lit(lights: &[[i32; 2]], qx: i32, qy: i32) -> bool {
    lights
        .iter()
        .any(|&[x, y]| x == qx || y == qy || x - qx == y - qy || x - qx == qy - y)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            grid_illumination(5, &[[0, 0], [4, 4]], &[[1, 1], [1, 0]]),
            [1, 0]
        );
        debug_assert_eq!(
            grid_illumination(5, &[[0, 0], [4, 4]], &[[1, 1], [1, 1]]),
            [1, 1]
        );
        debug_assert_eq!(
            grid_illumination(5, &[[0, 0], [0, 4]], &[[0, 4], [0, 1], [1, 4]]),
            [1, 1, 0]
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
