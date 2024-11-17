mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_k_bit_flips(nums: &mut [i32], k: i32) -> i32 {
    let n = nums.len();
    let k = k as usize;
    let mut idx = 0;
    let mut res = 0;
    while idx <= n - k {
        if nums[idx] == 1 {
            idx += 1;
            continue;
        }
        for v in nums[idx..].iter_mut().take(k) {
            *v = 1 - *v;
        }
        res += 1;
        idx += 1;
    }
    if nums.iter().all(|&v| v == 1) {
        res
    } else {
        -1
    }
}

fn with_array(nums: &[i32], k: i32) -> i32 {
    let (n, k) = (nums.len(), k as usize);
    let mut flipped = vec![false; n];
    let mut valid_flips = 0;
    let mut res = 0;
    for (idx, &num) in nums.iter().enumerate() {
        if idx >= k && flipped[idx - k] {
            valid_flips -= 1;
        }
        if num == valid_flips & 1 {
            if idx + k > n {
                return -1;
            }
            valid_flips += 1;
            flipped[idx] = true;
            res += 1;
        }
    }
    res
}

fn with_deque(nums: &[i32], k: i32) -> i32 {
    let (n, k) = (nums.len(), k as usize);
    let mut flip_queue = std::collections::VecDeque::with_capacity(k);
    let mut flipped = 0;
    let mut res = 0;
    for (idx, &num) in nums.iter().enumerate() {
        if idx >= k {
            let Some(v) = flip_queue.pop_front() else {
                break;
            };
            flipped ^= v;
        }
        if num == flipped {
            if idx + k > n {
                return -1;
            }
            flip_queue.push_back(1);
            flipped ^= 1;
            res += 1;
        } else {
            flip_queue.push_back(0);
        }
    }
    res
}

fn with_constant_space(nums: &mut [i32], k: i32) -> i32 {
    let (n, k) = (nums.len(), k as usize);
    let mut curr = 0;
    let mut res = 0;
    for idx in 0..n {
        if idx >= k && nums[idx - k] == 2 {
            curr -= 1;
            // nums[idx - k] -= 2; // restores original state
        }
        if curr & 1 == nums[idx] {
            if idx + k > n {
                return -1;
            }
            nums[idx] = 2;
            curr += 1;
            res += 1;
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
        debug_assert_eq!(with_constant_space(&mut [0, 1, 0], 1), 2);
        debug_assert_eq!(with_constant_space(&mut [1, 1, 0], 2), -1);
        debug_assert_eq!(with_constant_space(&mut [0, 0, 0, 1, 0, 1, 1, 0], 3), 3);
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
