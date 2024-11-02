mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn projection_area(grid: &[&[i32]]) -> i32 {
    let n = grid.len();
    let xy = grid
        .iter()
        .map(|r| r.iter().filter(|&&v| v > 0).count())
        .sum::<usize>() as i32;
    let xz = grid
        .iter()
        .map(|r| r.iter().max().unwrap_or(&0))
        .sum::<i32>();
    let yz = (0..n)
        .map(|c| grid.iter().map(|row| row[c]).max().unwrap_or(0))
        .sum::<i32>();
    xy + xz + yz
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(projection_area(&[&[1, 2], &[3, 4]]), 17);
        debug_assert_eq!(projection_area(&[&[2]]), 5);
        debug_assert_eq!(projection_area(&[&[1, 0], &[0, 2]]), 8);
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
