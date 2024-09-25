mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_poisoned_duration(time_series: &[i32], duration: i32) -> i32 {
    if duration == 0 {
        return 0;
    }
    let (mut start, mut end) = (time_series[0], time_series[0] + duration);
    let mut res = 0;
    // or zip(skip(1))
    for &num in time_series.iter().skip(1) {
        res += end.min(num) - start;
        start = num;
        end = start + duration;
    }
    res += duration;
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(find_poisoned_duration(&[1, 4], 2), 4);
        debug_assert_eq!(find_poisoned_duration(&[1, 2], 2), 3);
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
