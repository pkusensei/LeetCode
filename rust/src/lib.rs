mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_candies(candies: &[i32], k: i64) -> i32 {
    let sum: i64 = candies.iter().map(|&v| i64::from(v)).sum();
    if sum < k {
        return 0;
    }
    let mut left = 1;
    let mut right = *candies.iter().max().unwrap();
    while left < right {
        let mid = left + (right + 1 - left) / 2;
        if candies.iter().map(|&v| i64::from(v / mid)).sum::<i64>() >= k {
            left = mid;
        } else {
            right = mid - 1;
        }
    }
    left
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
    fn basics() {
        assert_eq!(maximum_candies(&[5, 8, 6], 3), 5);
    }

    #[test]
    fn test() {}
}
