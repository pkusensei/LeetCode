mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_beauty(nums: &mut [i32], k: i32) -> i32 {
    nums.sort_unstable();
    let mut res = 0;
    let mut left = 0;
    for (right, &num) in nums.iter().enumerate() {
        while left < right && num - nums[left] > 2 * k {
            left += 1
        }
        res = res.max(right - left + 1);
    }
    res as i32
}

fn with_prefix_sum(nums: &[i32], k: i32) -> i32 {
    if nums.len() == 1 {
        return 1;
    }
    let max = nums.iter().copied().max().unwrap();
    let mut count = vec![0; 1 + max as usize];
    for &num in nums.iter() {
        count[(num - k).max(0) as usize] += 1;
        count[(num + k + 1).min(max) as usize] -= 1;
    }
    let mut res = 0;
    let mut curr = 0;
    for c in count {
        curr += c;
        res = res.max(curr);
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(with_prefix_sum(&mut [4, 6, 1, 2], 2), 3);
        assert_eq!(with_prefix_sum(&mut [1, 1, 1, 1], 10), 4);
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
        if (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() > EP {
            dbg!(a, b);
        }
    }
}
