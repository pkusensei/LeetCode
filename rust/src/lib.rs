mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_tastiness(price: &mut [i32], k: i32) -> i32 {
    price.sort_unstable();
    let mut left = 0;
    let mut right = *price.last().unwrap();
    while left < right {
        let mid = left + (right + 1 - left) / 2;
        if check(price, mid, k) {
            left = mid;
        } else {
            right = mid - 1;
        }
    }
    left
}

fn check(nums: &[i32], mid: i32, k: i32) -> bool {
    let n = nums.len();
    let mut res = 1;
    let mut prev = nums[0];
    while res < k {
        let i = nums.partition_point(|&v| v < prev + mid);
        if i < n {
            res += 1;
            prev = nums[i];
        } else {
            break;
        }
    }
    res >= k
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
        assert_eq!(maximum_tastiness(&mut [13, 5, 1, 8, 21, 2], 3), 8);
    }

    #[test]
    fn test() {}
}
