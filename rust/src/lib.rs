mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn subarrays_with_k_distinct(nums: &[i32], k: i32) -> i32 {
    window_at_most(nums, k) - window_at_most(nums, k - 1)
}

fn window_at_most(nums: &[i32], k: i32) -> i32 {
    let mut res = 0;
    let mut left = 0;
    let mut map = HashMap::new();
    for (right, &num) in nums.iter().enumerate() {
        *map.entry(num).or_insert(0) += 1;
        while map.len() > k as usize {
            let Some(v) = map.get_mut(&nums[left]) else {
                break;
            };
            *v -= 1;
            if *v == 0 {
                map.remove(&nums[left]);
            }
            left += 1;
        }
        res += right - left + 1;
    }
    res as i32
}

fn single_pass(nums: &[i32], mut k: i32) -> i32 {
    let mut counts = vec![0; 1 + nums.len()];
    let mut res = 0;
    let mut left = 0;
    let mut curr = 0;
    for &num in nums.iter() {
        counts[num as usize] += 1;
        if counts[num as usize] == 1 {
            k -= 1;
        }
        if k < 0 {
            counts[nums[left] as usize] -= 1;
            if counts[nums[left] as usize] == 0 {
                k += 1;
            }
            left += 1;
            curr = 0;
        }
        if k == 0 {
            while counts[nums[left] as usize] > 1 {
                counts[nums[left] as usize] -= 1;
                left += 1;
                curr += 1;
            }
            res += 1 + curr;
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
        debug_assert_eq!(single_pass(&[1, 2, 1, 2, 3], 2), 7);
        debug_assert_eq!(single_pass(&[1, 2, 1, 3, 4], 3), 3);
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
