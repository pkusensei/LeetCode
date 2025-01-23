mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_requests(n: i32, requests: &[[i32; 2]]) -> i32 {
    let mut res = 0;
    backtrack(&mut vec![0; n as usize], requests, 0, &mut res);
    res
}

fn backtrack(nums: &mut [i32], requests: &[[i32; 2]], curr: i32, res: &mut i32) {
    match requests {
        [] => {
            if nums.iter().all(|&v| v == 0) {
                *res = (*res).max(curr);
            }
        }
        [head, tail @ ..] => {
            backtrack(nums, tail, curr, res);
            let a = head[0] as usize;
            let b = head[1] as usize;
            nums[a] -= 1;
            nums[b] += 1;
            backtrack(nums, tail, 1 + curr, res);
            nums[a] += 1;
            nums[b] -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            maximum_requests(5, &[[0, 1], [1, 0], [0, 1], [1, 2], [2, 0], [3, 4]]),
            5
        );
        assert_eq!(maximum_requests(3, &[[0, 0], [1, 2], [2, 1]]), 3);
        assert_eq!(maximum_requests(4, &[[0, 3], [3, 1], [1, 2], [2, 0]]), 4);
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
