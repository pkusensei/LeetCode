mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn get_averages(nums: &[i32], k: i32) -> Vec<i32> {
    let (n, k) = (nums.len(), k as usize);
    let mut res = vec![-1; n];
    if n < 2 * k + 1 {
        return res;
    }
    let mut window: i64 = nums[..1 + 2 * k].iter().map(|&v| i64::from(v)).sum();
    for i in k..n - k {
        res[i] = (window / (1 + 2 * k as i64)) as i32;
        window -= i64::from(nums[i - k]);
        window += i64::from(*nums.get(i + k + 1).unwrap_or(&0));
    }
    res
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {{
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        }};
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        }};
    }

    #[test]
    fn basics() {}

    #[test]
    fn test() {}
}
