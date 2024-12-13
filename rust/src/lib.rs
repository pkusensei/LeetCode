mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn check_straight_line(coordinates: &[[i32; 2]]) -> bool {
    if coordinates.len() <= 2 {
        return true;
    }
    let [x1, y1] = [coordinates[0][0], coordinates[0][1]];
    let [x2, y2] = [coordinates[1][0], coordinates[1][1]];
    let dx = x2 - x1;
    let dy = y2 - y1;
    for v in coordinates.iter().skip(2) {
        if dy * (v[0] - x1) != dx * (v[1] - y1) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert!(check_straight_line(&[
            [1, 2],
            [2, 3],
            [3, 4],
            [4, 5],
            [5, 6],
            [6, 7]
        ]));
        assert!(!check_straight_line(&[
            [1, 1],
            [2, 2],
            [3, 4],
            [4, 5],
            [5, 6],
            [7, 7]
        ]));
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
        if (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() > EP {
            dbg!(a, b);
        }
    }
}
