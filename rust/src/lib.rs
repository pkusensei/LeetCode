mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn interval_intersection(first_list: &[[i32; 2]], second_list: &[[i32; 2]]) -> Vec<[i32; 2]> {
    let mut res = vec![];
    let (mut i1, mut i2) = (0, 0);
    while i1 < first_list.len() && i2 < second_list.len() {
        let (s1, e1) = (first_list[i1][0], first_list[i1][1]);
        let (s2, e2) = (second_list[i2][0], second_list[i2][1]);
        let start = s1.max(s2);
        let end = e1.min(e2);
        if start <= end {
            res.push([start, end]);
        }
        if e1 <= e2 {
            i1 += 1;
        } else {
            i2 += 1;
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
            interval_intersection(
                &[[0, 2], [5, 10], [13, 23], [24, 25]],
                &[[1, 5], [8, 12], [15, 24], [25, 26]]
            ),
            [[1, 2], [5, 5], [8, 10], [15, 23], [24, 24], [25, 25]]
        );
        debug_assert!(interval_intersection(&[[1, 3], [5, 9]], &[]).is_empty());
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
