mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn preimage_size_fzf(k: i32) -> i32 {
    const MAX: i64 = trailing_zeros(i64::MAX);
    let k = k as i64;
    match k.cmp(&MAX) {
        std::cmp::Ordering::Equal => return 3,
        std::cmp::Ordering::Greater => return 0,
        std::cmp::Ordering::Less => (),
    }

    let (mut left, mut right) = (0, i64::MAX);
    while left < right {
        let mid = left + (right - left) / 2;
        match trailing_zeros(mid).cmp(&k) {
            std::cmp::Ordering::Less => left = 1 + mid,
            std::cmp::Ordering::Equal => return 5,
            std::cmp::Ordering::Greater => right = mid - 1,
        }
    }
    0
}

const fn trailing_zeros(n: i64) -> i64 {
    let mut res = 0;
    let mut factor = 5;
    while factor <= n {
        res += n / factor;
        let Some(v) = factor.checked_mul(5) else {
            break;
        };
        factor = v;
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(trailing_zeros(5), 1);
        debug_assert_eq!(trailing_zeros(16), 3);
        debug_assert_eq!(trailing_zeros(29), 6);
        debug_assert_eq!(trailing_zeros(53), 12);
        debug_assert_eq!(trailing_zeros(127), 31);

        debug_assert_eq!(preimage_size_fzf(0), 5);
        debug_assert_eq!(preimage_size_fzf(5), 0);
        debug_assert_eq!(preimage_size_fzf(3), 5);
    }

    #[test]
    fn test() {
        debug_assert_eq!(preimage_size_fzf(1_000_000_000), 5);
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
