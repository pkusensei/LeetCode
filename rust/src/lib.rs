mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn is_good(nums: Vec<i32>) -> bool {
    let n = nums.len() - 1;
    if n == 0 {
        return false;
    }
    let mut count = vec![0; n];
    for &num in nums.iter() {
        let num = num as usize - 1;
        if num >= n {
            return false;
        }
        count[num] += 1;
    }
    count[..n - 1].iter().all(|&v| v == 1) && count[n - 1] == 2
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
