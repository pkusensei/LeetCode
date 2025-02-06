mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_score(nums: &[i32], k: i32) -> i32 {
    let (n, k) = (nums.len(), k as usize);
    let mut res = nums[k];
    let mut curr_min = nums[k];
    let [mut left, mut right] = [k, k];
    while left > 0 || right < n {
        let peek_left = left.checked_sub(1).map(|i| nums[i]);
        let peek_right = nums.get(1 + right).copied();
        match (peek_left, peek_right) {
            (Some(a), None) => {
                curr_min = curr_min.min(a);
                left -= 1;
            }
            (None, Some(b)) => {
                curr_min = curr_min.min(b);
                right += 1;
            }
            (Some(a), Some(b)) => {
                if a <= b {
                    curr_min = curr_min.min(b);
                    right += 1;
                } else {
                    curr_min = curr_min.min(a);
                    left -= 1;
                }
            }
            _ => break,
        }
        res = res.max(curr_min * (right - left + 1) as i32);
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
        assert_eq!(maximum_score(&[1, 4, 3, 7, 4, 5], 3), 15);
        assert_eq!(maximum_score(&[5, 5, 4, 5, 4, 1, 1, 1], 0), 20);
    }

    #[test]
    fn test() {
        // 1622 * 6
        assert_eq!(
            maximum_score(
                &[6569, 9667, 3148, 7698, 1622, 2194, 793, 9041, 1670, 1872],
                5
            ),
            9732
        );
    }
}
