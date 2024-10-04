mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_distance(arrays: &[&[i32]]) -> i32 {
    let mut min = arrays[0][0];
    let mut max = arrays[0].last().copied().unwrap_or(min);
    let mut res = 0;
    for a in arrays.iter().skip(1) {
        let temp_min = a[0];
        let temp_max = a.last().copied().unwrap_or(temp_min);
        res = res.max(temp_max.abs_diff(min)).max(max.abs_diff(temp_min));
        // min and max can be from different arrays
        // because res is only calculated when seeing a new one
        min = min.min(temp_min);
        max = max.max(temp_max);
    }
    res as _
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(max_distance(&[&[1, 2, 3], &[4, 5], &[1, 2, 3]]), 4);
        debug_assert_eq!(max_distance(&[&[1], &[1]]), 0);
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
