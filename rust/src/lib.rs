mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn find_kth_number(n: i32, k: i32) -> i32 {
    let (mut res, mut count) = (0, 0);
    for i in 1..=9 {
        dfs(i, n as _, &mut res, &mut count, k);
    }
    res
}

fn dfs(curr: i64, n: i64, res: &mut i32, count: &mut i32, k: i32) {
    if curr > n {
        return;
    }
    *count += 1;
    if *count == k {
        *res = curr as i32;
        return;
    }
    let next = curr * 10;
    for i in 0..=9 {
        dfs(next + i, n, res, count, k);
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(find_kth_number(13, 2), 10);
        debug_assert_eq!(find_kth_number(1, 1), 1);
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
