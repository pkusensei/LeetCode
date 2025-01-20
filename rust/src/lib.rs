mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_swaps(grid: &[&[i32]]) -> i32 {
    let n = grid.len();
    let mut nums: Vec<_> = grid
        .iter()
        .map(|row| {
            row.iter()
                .rposition(|&v| v != 0)
                .map(|v| v as i16)
                .unwrap_or(-1)
        })
        .collect();
    let mut temp = nums.clone();
    temp.sort_unstable();
    if temp.iter().enumerate().any(|(r, &pos)| pos > r as i16) {
        return -1;
    }
    let mut res = 0;
    for r in 0..n {
        if nums[r] <= r as i16 {
            continue;
        }
        let Some(i) = nums.iter().enumerate().skip(1 + r).find_map(|(i, &v)| {
            if v <= r as i16 {
                Some(i)
            } else {
                None
            }
        }) else {
            break;
        };
        res += (i - r) as i32;
        nums[r..=i].rotate_right(1);
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(min_swaps(&[&[0, 0, 1], &[1, 1, 0], &[1, 0, 0]]), 3);
        assert_eq!(
            min_swaps(&[&[0, 1, 1, 0], &[0, 1, 1, 0], &[0, 1, 1, 0], &[0, 1, 1, 0]]),
            -1
        );
        assert_eq!(min_swaps(&[&[1, 0, 0], &[1, 1, 0], &[1, 1, 1]]), 0);
    }

    #[test]
    fn test() {
        assert_eq!(min_swaps(&[&[0, 0], &[0, 1]]), 0);
        assert_eq!(
            min_swaps(&[
                &[1, 0, 0, 0, 0, 0],
                &[0, 1, 0, 1, 0, 0],
                &[1, 0, 0, 0, 0, 0],
                &[1, 1, 1, 0, 0, 0],
                &[1, 1, 0, 1, 0, 0],
                &[1, 0, 0, 0, 0, 0]
            ]),
            2
        );
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
