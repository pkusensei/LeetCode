mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn max_equal_freq(nums: &[i32]) -> i32 {
    let mut count = HashMap::new();
    let mut freq = HashMap::new();
    let mut max_freq = 0;
    let mut res = 0;
    for (idx, &num) in nums.iter().enumerate() {
        let v = count.entry(num).or_insert(0);
        *v += 1;
        *freq.entry(*v).or_insert(0) += 1;
        *freq.entry(*v - 1).or_insert(0) -= 1;
        max_freq = max_freq.max(*v);
        // everything once
        // one item occurs exactly once
        // one with max_freq, everything else with (max_freq-1)
        if 1 == max_freq
            || max_freq * freq[&max_freq] == idx as i32
            || (max_freq - 1) * (freq.get(&(max_freq - 1)).unwrap_or(&0) + 1) == idx as i32
        {
            res = 1 + idx as i32
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
        assert_eq!(max_equal_freq(&[2, 2, 1, 1, 5, 3, 3, 5]), 7);
        assert_eq!(max_equal_freq(&[1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5]), 13);
    }

    #[test]
    fn test() {
        assert_eq!(max_equal_freq(&[1, 2]), 2);
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
        if (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() > EP {
            dbg!(a, b);
        }
    }
}
