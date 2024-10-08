mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
    let (mut left, mut right) = (1, m * n);
    while left < right {
        let mid = left + (right - left) / 2;
        if count(mid, m, n) >= k {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left
}

fn count(mid: i32, m: i32, n: i32) -> i32 {
    let mut res = 0;
    for row in 1..=m {
        // mid/row > n => mid is bigger than the whole row
        // take n as count
        // other wise, mid falls on mid/row on current row
        res += (mid / row).min(n);
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        // debug_assert_eq!(find_kth_number(3, 3, 5), 3);
        debug_assert_eq!(find_kth_number(2, 3, 6), 6);
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
