mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn diagonal_prime(nums: Vec<Vec<i32>>) -> i32 {
    let n = nums.len();
    let mut res = None;
    for i in 0..n {
        let a = nums[i][i];
        let b = nums[i][n - i - 1];
        for p in [a, b] {
            if res.is_none_or(|v| v < p) && is_prime(p) {
                res = Some(p)
            }
        }
    }
    res.unwrap_or(0)
}

fn is_prime(num: i32) -> bool {
    if num < 2 {
        false
    } else {
        let sq = num.isqrt();
        for p in 2..=sq {
            if num % p == 0 {
                return false;
            }
        }
        true
    }
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
        assert!(is_prime(2))
    }

    #[test]
    fn test() {}
}
