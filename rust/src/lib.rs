mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn surface_area(grid: &[&[i32]]) -> i32 {
    let mut res = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, &num) in row.iter().enumerate() {
            if num == 0 {
                continue;
            }
            res += 2;
            for (nx, ny) in [
                (x.saturating_sub(1), y),
                (x + 1, y),
                (x, y.saturating_sub(1)),
                (x, y + 1),
            ] {
                if (nx, ny) == (x, y) {
                    res += num;
                } else {
                    let v = grid.get(ny).and_then(|r| r.get(nx)).copied().unwrap_or(0);
                    res += (num - v).max(0);
                }
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
        debug_assert_eq!(surface_area(&[&[1, 2], &[3, 4]]), 34);
        debug_assert_eq!(surface_area(&[&[1, 1, 1], &[1, 0, 1], &[1, 1, 1]]), 32);
        debug_assert_eq!(surface_area(&[&[2, 2, 2], &[2, 1, 2], &[2, 2, 2]]), 46);
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
