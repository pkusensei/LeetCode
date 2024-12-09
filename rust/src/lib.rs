mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn is_array_special(nums: &[i32], queries: &[[i32; 2]]) -> Vec<bool> {
    let mut prefix = Vec::with_capacity(nums.len());
    prefix.push(0);
    for (i, &num) in nums.iter().enumerate().skip(1) {
        prefix.push(prefix.last().unwrap_or(&0) + i32::from(num & 1 == nums[i - 1] & 1));
    }
    let mut res = Vec::with_capacity(queries.len());
    for q in queries.iter() {
        // No increase in "abnormal" numbers
        res.push(prefix[q[1] as usize] == prefix[q[0] as usize]);
    }
    res
}

fn with_binary_search(nums: &[i32], queries: &[[i32; 2]]) -> Vec<bool> {
    fn bs(indices: &[usize], start: usize, end: usize) -> bool {
        let mut left = 0;
        let mut right = indices.len() - 1;
        while left <= right {
            let mid = left + (right - left) / 2;
            let i = indices[mid];
            if i < start {
                left = 1 + mid;
            } else if i > end {
                right = mid - 1;
            } else {
                return true;
            }
        }
        false
    }

    let indices: Vec<_> = nums
        .windows(2)
        .enumerate()
        .filter_map(|(i, w)| {
            if w[0] & 1 == w[1] & 1 {
                Some(1 + i)
            } else {
                None
            }
        })
        .collect();
    queries
        .iter()
        .map(|q| !bs(&indices, 1 + q[0] as usize, q[1] as usize))
        .collect()
}

fn sliding_window(nums: &[i32], queries: &[[i32; 2]]) -> Vec<bool> {
    let n = nums.len();
    let mut reach = Vec::with_capacity(n);
    let mut end = 0;
    for start in 0..n {
        end = start.max(end);
        while end < n - 1 && nums[end] & 1 != nums[1 + end] & 1 {
            end += 1;
        }
        reach.push(end);
    }
    queries
        .iter()
        .map(|q| q[1] as usize <= reach[q[0] as usize])
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(sliding_window(&[3, 4, 1, 2, 6], &[[0, 4]]), [false]);
        assert_eq!(
            sliding_window(&[4, 3, 1, 6], &[[0, 2], [2, 3]]),
            [false, true]
        );
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
