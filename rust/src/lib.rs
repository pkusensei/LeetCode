mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn odd_even_jumps(arr: &[i32]) -> i32 {
    let n = arr.len();
    let mut dp = vec![[-1; 2]; n];
    (0..n).map(|i| dfs(arr, i, 1, &mut dp)).sum()
}

fn dfs(nums: &[i32], idx: usize, odd: usize, dp: &mut [[i32; 2]]) -> i32 {
    if idx >= nums.len() - 1 {
        return 1;
    }
    if dp[idx][odd] > -1 {
        return dp[idx][odd];
    }
    let res = if odd == 1 {
        if let Some(jump) = nums[1 + idx..]
            .iter()
            .enumerate()
            .filter(|(_, &v)| v >= nums[idx])
            .min_by_key(|(_, &v)| v)
            .map(|(i, _)| i + 1 + idx)
        {
            dfs(nums, jump, 1 - odd, dp)
        } else {
            0
        }
    } else if let Some(jump) = nums[1 + idx..]
        .iter()
        .enumerate()
        .filter(|(_, &v)| v <= nums[idx])
        .max_by(|(i1, v1), (i2, v2)| v1.cmp(v2).then(i2.cmp(i1)))
        .map(|(i, _)| i + 1 + idx)
    {
        dfs(nums, jump, 1 - odd, dp)
    } else {
        0
    };
    dp[idx][odd] = res;
    res
}

fn with_btreemap(nums: &[i32]) -> i32 {
    let n = nums.len();
    let [mut odd, mut even] = [0, 1].map(|_| vec![false; n]);
    odd[n - 1] = true;
    even[n - 1] = true;
    let mut res = 1;
    let mut map = std::collections::BTreeMap::from([(nums[n - 1], n - 1)]);
    for (idx, &num) in nums.iter().enumerate().rev().skip(1) {
        if let Some((_, &i)) = map.range(num..).next() {
            odd[idx] = even[i];
        }
        if let Some((_, &i)) = map.range(..=num).next_back() {
            even[idx] = odd[i];
        }
        if odd[idx] {
            res += 1;
        }
        map.insert(num, idx);
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(with_btreemap(&[10, 13, 12, 14, 15]), 2);
        debug_assert_eq!(with_btreemap(&[2, 3, 1, 1, 4]), 3);
        debug_assert_eq!(with_btreemap(&[5, 1, 3, 4, 2]), 3);
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
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
