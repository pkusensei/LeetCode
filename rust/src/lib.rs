mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_value_after_reverse(nums: &[i32]) -> i32 {
    let [mut total, mut inc] = [0; 2];
    let [mut minmax, mut maxmin] = [i32::MAX, i32::MIN];
    let n = nums.len();
    for w in nums.windows(2) {
        let [a, b] = [w[0], w[1]];
        let curr = a.abs_diff(b) as i32;
        total += curr;
        minmax = minmax.min(a.max(b));
        maxmin = maxmin.max(a.min(b));
        // reverse left half or right half
        inc = inc
            .max(nums[0].abs_diff(b) as i32 - curr)
            .max(nums[n - 1].abs_diff(a) as i32 - curr);
    }
    total + inc.max(2 * (maxmin - minmax))
    // [..a, b.. c, d..] yields delta = max(a,b)-min(a,b) + max(c,d)-min(c,d)
    // Reverse it to [..a, c.. b, d..]
    // 1) max(a,b)>=min(c,d)
    //    max(a,c)-min(a,c) + max(b,d)-min(b,d) <= delta
    //    This reversal is not desired
    // 2) max(a,b)<min(c,d)
    //    A reversal is an increase of 2*(min(c,d)-max(a,b))+delta
    //    Now keep track of max(min(c,d)) and min(max(a,b))
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(max_value_after_reverse(&[2, 3, 1, 5, 4]), 10);
        assert_eq!(max_value_after_reverse(&[2, 4, 9, 24, 2, 1, 10]), 68);
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
