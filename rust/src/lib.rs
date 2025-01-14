mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_the_prefix_common_array(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut res = Vec::with_capacity(a.len());
    let [mut mask1, mut mask2] = [0u64; 2];
    // let mut map = std::collections::HashMap::with_capacity(a.len());
    for (&x, &y) in a.iter().zip(b.iter()) {
        mask1 |= 1 << x;
        mask2 |= 1 << y;
        res.push((mask1 & mask2).count_ones() as i32);
        //     *map.entry(x).or_insert(0) += 1;
        //     *map.entry(y).or_insert(0) += 1;
        //     res.push(map.values().filter(|&&v| v > 1).count() as i32);
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            find_the_prefix_common_array(&[1, 3, 2, 4], &[3, 1, 2, 4]),
            [0, 2, 3, 4]
        );
        assert_eq!(
            find_the_prefix_common_array(&[2, 3, 1], &[3, 1, 2]),
            [0, 1, 3]
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
