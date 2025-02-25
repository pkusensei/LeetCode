mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
    let mut left = 1;
    let mut right = *quantities.iter().max().unwrap_or(&100000);
    while left < right {
        let mid = left + (right - left) / 2;
        if count(&quantities, mid) > n {
            left = 1 + mid;
        } else {
            right = mid;
        }
    }
    left
}

fn count(nums: &[i32], mid: i32) -> i32 {
    nums.iter().map(|n| n / mid + i32::from(n % mid > 0)).sum()
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
