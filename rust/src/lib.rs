mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_subarray_length(nums: &[i32], k: i32) -> i32 {
    let mut bits = [0; 32];
    let mut left = 0;
    let mut res = None::<i32>;
    for (right, &num) in nums.iter().enumerate() {
        bits = update(bits, num, 1);
        while left <= right && to_num(&bits) >= k {
            if let Some(ref mut v) = res {
                *v = (*v).min((right - left + 1) as i32);
            } else {
                res = Some((right - left + 1) as i32)
            }
            bits = update(bits, nums[left], -1);
            left += 1;
        }
    }
    res.unwrap_or(-1)
}

fn to_num(bits: &[i32; 32]) -> i32 {
    let mut res = 0;
    for (i, &b) in bits.iter().enumerate() {
        if b > 0 {
            res |= 1 << i;
        }
    }
    res
}

fn update(mut bits: [i32; 32], num: i32, delta: i32) -> [i32; 32] {
    for (i, bit) in bits.iter_mut().enumerate() {
        if (num >> i) & 1 == 1 {
            *bit += delta;
        }
    }
    bits
}

fn with_binary_search(nums: &[i32], k: i32) -> i32 {
    let n = nums.len();
    let (mut left, mut right) = (1, n);
    let mut res = None::<i32>;
    while left <= right {
        let mid = left + (right - left) / 2;
        if is_valid(nums, k, mid) {
            res = Some(mid as i32);
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    res.unwrap_or(-1)
}

fn is_valid(nums: &[i32], k: i32, window: usize) -> bool {
    let mut bits = [0; 32];
    for (right, &num) in nums.iter().enumerate() {
        bits = update(bits, num, 1);
        if right >= window {
            bits = update(bits, nums[right - window], -1);
        }
        if right >= window - 1 && to_num(&bits) >= k {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(with_binary_search(&[1, 2, 3], 2), 1);
        debug_assert_eq!(with_binary_search(&[2, 1, 8], 10), 3);
        debug_assert_eq!(with_binary_search(&[1, 2], 0), 1);
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
        T1: Into<f64>,
        T2: Into<f64>,
    {
        const EP: f64 = 1e-5;
        debug_assert!((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP);
    }
}
