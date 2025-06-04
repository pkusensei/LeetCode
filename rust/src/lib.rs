mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_non_decreasing_subarrays(nums: &[i32], k: i32) -> i64 {
    let k = i64::from(k);
    let mut right = nums.len() - 1;
    let mut queue = std::collections::VecDeque::from([(nums[right], 1)]);
    let mut cost = 0;
    let mut res = 1;
    for (left, &num) in nums.iter().enumerate().rev().skip(1) {
        let mut curr_count = 1;
        // "Flatten" the back half of deque to be >= this num
        while queue.back().is_some_and(|&(v, _)| v < num) {
            let Some((last, count)) = queue.pop_back() else {
                break;
            };
            cost += i64::from(num - last) * count;
            curr_count += count;
        }
        queue.push_back((num, curr_count));
        while cost > k {
            // Pop smaller (to the right of array) off queue
            let Some((front, mut count)) = queue.pop_front() else {
                break;
            };
            cost -= i64::from(front - nums[right]);
            right -= 1;
            count -= 1;
            if count > 0 {
                queue.push_front((front, count));
            }
        }
        res += right + 1 - left;
    }
    res as i64
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
        assert_eq!(count_non_decreasing_subarrays(&[6, 3, 1, 2, 4, 4], 7), 17);
        assert_eq!(count_non_decreasing_subarrays(&[6, 3, 1, 3, 6], 4), 12);
    }

    #[test]
    fn test() {
        assert_eq!(
            count_non_decreasing_subarrays(&[1000000000, 1, 1, 1, 1], 1000000000),
            12
        );
    }
}
