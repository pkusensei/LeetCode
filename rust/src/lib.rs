mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_capability(nums: &[i32], k: i32) -> i32 {
    let mut left = *nums.iter().min().unwrap();
    let mut right = *nums.iter().max().unwrap();
    while left < right {
        let mid = left + (right - left) / 2;
        if count(nums, mid) >= k {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left
}

fn count(nums: &[i32], mid: i32) -> i32 {
    let mut res = 0;
    let mut prev = None;
    for idx in nums
        .iter()
        .enumerate()
        .filter_map(|(i, &v)| if v <= mid { Some(i) } else { None })
    {
        if prev.is_none_or(|v| v + 1 < idx) {
            res += 1;
            prev = Some(idx);
        }
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
    fn basics() {
        assert_eq!(min_capability(&[2, 3, 5, 9], 2), 5);
        assert_eq!(min_capability(&[2, 7, 9, 3, 1], 2), 2);
    }

    #[test]
    fn test() {}
}
