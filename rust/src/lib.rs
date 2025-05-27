mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_difference(nums: &[i32]) -> i32 {
    let mut min = i32::MAX;
    let mut max = 0;
    let mut max_gap = 0;
    for w in nums.windows(2) {
        let a = w[0].min(w[1]);
        let b = w[0].max(w[1]);
        if a == -1 && b > -1 {
            min = min.min(b);
            max = max.max(b);
        } else {
            max_gap = max_gap.max(b - a);
        }
    }
    let mut left = max_gap;
    let mut right = (max - min + 1) / 2;
    while left < right {
        let mid = left + (right - left) / 2;
        if check(nums, min + mid, max - mid, mid) {
            right = mid;
        } else {
            left = 1 + mid;
        }
    }
    left
}

fn check(nums: &[i32], x: i32, y: i32, d: i32) -> bool {
    let mut count = 0; // minus ones
    let mut prev = 0_i32;
    for &num in nums {
        if num == -1 {
            if prev > 0 && prev.abs_diff(x).min(prev.abs_diff(y)) > d as u32 {
                return false; // abs(prev-curr) must be <= d
            }
            count += 1;
        } else {
            if count > 0 {
                let diff = if prev > 0 {
                    std::cmp::min(
                        prev.abs_diff(x).max(num.abs_diff(x)),
                        prev.abs_diff(y).max(num.abs_diff(y)),
                    )
                } else {
                    num.abs_diff(x).min(num.abs_diff(y))
                };
                if diff > d as u32 && (count == 1 || y - x > d) {
                    return false;
                }
            }
            prev = num;
            count = 0;
        }
    }
    true
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
        assert_eq!(min_difference(&[1, 2, -1, 10, 8]), 4);
        assert_eq!(min_difference(&[-1, 10, -1, 8]), 1);
    }

    #[test]
    fn test() {}
}
