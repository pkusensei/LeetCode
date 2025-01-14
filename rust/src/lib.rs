mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn longest_subarray(nums: &[i32], limit: i32) -> i32 {
    let mut map = std::collections::BTreeMap::new();
    let mut left = 0;
    let mut res = 0;
    for (right, &num) in nums.iter().enumerate() {
        *map.entry(num).or_insert(0) += 1;
        while map
            .keys()
            .last()
            .zip(map.keys().next())
            .is_some_and(|(a, b)| a - b > limit)
        {
            *map.entry(nums[left]).or_insert(0) -= 1;
            if 0 == map[&nums[left]] {
                map.remove(&nums[left]);
            }
            left += 1;
        }
        res = res.max(right + 1 - left);
    }
    res as _
}

fn with_deque(nums: &[i32], limit: i32) -> i32 {
    let [mut max_queue, mut min_queue] = [0, 1].map(|_| std::collections::VecDeque::new());
    let mut left = 0;
    let mut res = 0;
    for (right, &num) in nums.iter().enumerate() {
        while max_queue.back().is_some_and(|&v| v < num) {
            max_queue.pop_back();
        }
        max_queue.push_back(num);
        while min_queue.back().is_some_and(|&v| v > num) {
            min_queue.pop_back();
        }
        min_queue.push_back(num);
        while max_queue
            .front()
            .zip(min_queue.front())
            .is_some_and(|(a, b)| a - b > limit)
        {
            if max_queue.front().is_some_and(|&v| v == nums[left]) {
                max_queue.pop_front();
            }
            if min_queue.front().is_some_and(|&v| v == nums[left]) {
                min_queue.pop_front();
            }
            left += 1;
        }
        res = res.max(right + 1 - left);
    }
    res as _
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(longest_subarray(&[8, 2, 4, 7], 4), 2);
        assert_eq!(longest_subarray(&[10, 1, 2, 4, 7, 2], 5), 4);
        assert_eq!(longest_subarray(&[4, 2, 2, 2, 4, 4, 2, 2], 0), 3);

        assert_eq!(with_deque(&[8, 2, 4, 7], 4), 2);
        assert_eq!(with_deque(&[10, 1, 2, 4, 7, 2], 5), 4);
        assert_eq!(with_deque(&[4, 2, 2, 2, 4, 4, 2, 2], 0), 3);
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
