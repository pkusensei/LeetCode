mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn find_right_interval(intervals: &[[i32; 2]]) -> Vec<i32> {
    let n = intervals.len();
    let mut starts: Vec<_> = intervals
        .iter()
        .enumerate()
        .map(|(i, v)| (v[0], i))
        .collect();
    starts.sort_unstable_by_key(|(v, _)| *v);
    let mut res = Vec::with_capacity(n);
    for end in intervals.iter().map(|v| v[1]) {
        let i = starts.partition_point(|(v, _)| *v < end);
        if i < n {
            res.push(starts[i].1 as i32);
        } else {
            res.push(-1);
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
        debug_assert_eq!(find_right_interval(&[[1, 2]]), [-1]);
        debug_assert_eq!(find_right_interval(&[[3, 4], [2, 3], [1, 2]]), [-1, 0, 1]);
        debug_assert_eq!(find_right_interval(&[[1, 4], [2, 3], [3, 4]]), [-1, 2, -1]);
    }

    #[test]
    fn test() {
        debug_assert_eq!(find_right_interval(&[[4, 4]]), [0]);
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
}
