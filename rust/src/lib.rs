mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn get_max_len(nums: &[i32]) -> i32 {
    let mut zero = 0;
    let [mut first_neg, mut last_neg] = [None; 2];
    let mut neg_count = 0;
    let mut res = 0;
    for (idx, num) in (0..).zip(
        std::iter::once(0)
            .chain(nums.iter().copied())
            .chain(std::iter::once(0)),
    ) {
        match num.cmp(&0) {
            std::cmp::Ordering::Less => {
                first_neg.get_or_insert(idx);
                _ = last_neg.insert(idx);
                neg_count += 1;
            }
            std::cmp::Ordering::Equal => {
                if zero + 1 >= idx {
                    zero = idx;
                    continue;
                }
                if neg_count & 1 == 0 {
                    res = res.max(idx - zero - 1);
                } else {
                    let a = first_neg.map(|i| idx - i - 1).unwrap_or(0);
                    let b = last_neg.map(|i| i - zero - 1).unwrap_or(0);
                    res = res.max(a.max(b));
                }
                [first_neg, last_neg] = [None; 2];
                neg_count = 0;
                zero = idx;
            }
            std::cmp::Ordering::Greater => (),
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
        assert_eq!(get_max_len(&[1, -2, -3, 4]), 4);
        assert_eq!(get_max_len(&[0, 1, -2, -3, -4]), 3);
        assert_eq!(get_max_len(&[-1, -2, -3, 0, 1]), 2);
    }

    #[test]
    fn test() {
        assert_eq!(get_max_len(&[-1, 2]), 1);
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
