mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

// 3 2 1
// 3 3 3
pub fn count_non_decreasing_subarrays(nums: Vec<i32>, k: i32) -> i64 {
    use std::collections::VecDeque;

    let n = nums.len();
    let mut k = i64::from(k);
    let mut right = n - 1;
    let mut queue = VecDeque::from([(nums[right], 1)]);
    let mut res = 1;
    for (left, &num) in nums[..n - 1].iter().enumerate().rev() {
        let mut count = 1;
        while let Some(&(f_num, f_c)) = queue.front()
            && f_num < num
        {
            queue.pop_front();
            k -= i64::from(num - f_num) * f_c;
            count += f_c;
        }
        queue.push_front((num, count));
        while k < 0 {
            let Some((b_num, mut b_c)) = queue.pop_back() else {
                break;
            };
            k += i64::from(b_num - nums[right]);
            right -= 1;
            b_c -= 1;
            if b_c > 0 {
                queue.push_back((b_num, b_c));
            }
        }
        res += 1 + right - left
    }
    res as i64
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
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
