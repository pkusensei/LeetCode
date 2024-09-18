mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn erase_overlap_intervals(mut intervals: Vec<[i32; 2]>) -> i32 {
    if intervals.len() < 2 {
        return 0;
    }
    intervals.sort_unstable();
    let (mut i1, mut i2) = (0, 1);
    let mut res = 0;
    while i2 < intervals.len() {
        if intervals[i1][1] <= intervals[i2][0] {
            // [1, 2] [2, 4]
            i1 = i2;
            i2 += 1;
        } else if intervals[i2][0] < intervals[i1][1] && intervals[i1][1] < intervals[i2][1] {
            // [1, 3] [2, 4]
            i2 += 1;
            res += 1
        } else {
            // [1, 4] [2, 3]
            i1 = i2;
            i2 += 1;
            res += 1
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
            erase_overlap_intervals(vec![[1, 2], [2, 3], [3, 4], [1, 3]]),
            1
        );
        debug_assert_eq!(erase_overlap_intervals(vec![[1, 2], [1, 2], [1, 2]]), 2);
        debug_assert_eq!(erase_overlap_intervals(vec![[1, 2], [2, 3]]), 0);
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
