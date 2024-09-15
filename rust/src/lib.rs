mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn find_nth_digit(n: i32) -> i32 {
    let mut step = 1;
    let mut start = 1;
    let mut count = 9;
    let mut curr = i64::from(n);

    while curr > step * count {
        curr -= step * count;
        step += 1;
        count *= 10;
        start *= 10;
    }
    let mut num = start + (curr - 1) / step;
    let mut idx = num.ilog10() as i64 - (curr - 1) % step; // 1-based index
    while idx > 0 {
        num /= 10;
        idx -= 1
    }
    (num % 10) as _
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(find_nth_digit(3), 3);
        debug_assert_eq!(find_nth_digit(11), 0);
    }

    #[test]
    fn test() {
        debug_assert_eq!(find_nth_digit(10), 1);
        debug_assert_eq!(find_nth_digit(1000000000), 1);
    }

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(i1: I1, i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: IntoIterator<Item = T1>,
        I2: IntoIterator<Item = T2>,
    {
        let mut v1: Vec<_> = i1.into_iter().collect();
        let mut v2: Vec<_> = i2.into_iter().collect();
        v1.sort_unstable();
        v2.sort_unstable();
        debug_assert_eq!(v1, v2);
    }
}
