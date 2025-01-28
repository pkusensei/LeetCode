mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_moves(nums: &[i32], limit: i32) -> i32 {
    let mut delta = vec![0; 2 + 2 * limit as usize];
    let n = nums.len();
    for (&a, &b) in nums.iter().zip(nums.iter().rev()).take(n / 2) {
        delta[a.min(b) as usize + 1] -= 1;
        delta[(a + b) as usize] -= 1;
        delta[(a + b + 1) as usize] += 1;
        delta[(a.max(b) + limit) as usize + 1] += 1;
    }
    let mut res = i32::MAX;
    let mut curr = n as i32;
    for v in delta[2..].iter() {
        curr += v;
        res = res.min(curr);
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    #[test]
    fn basics() {
        assert_eq!(min_moves(&[1, 2, 4, 3], 4), 1);
        assert_eq!(min_moves(&[1, 2, 2, 1], 2), 2);
        assert_eq!(min_moves(&[1, 2, 1, 2], 2), 0);
    }

    #[test]
    fn test() {}

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
