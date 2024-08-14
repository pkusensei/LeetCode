use std::collections::HashMap;

pub fn smallest_distance_pair(nums: &[i32], mut k: i32) -> i32 {
    let max_num = nums.iter().max().copied().unwrap_or(nums[0]);
    let mut diff = vec![0; 1 + max_num as usize];
    for (i, x) in nums.iter().enumerate() {
        for y in nums.iter().skip(i + 1) {
            diff[x.abs_diff(*y) as usize] += 1;
        }
    }
    for dist in 0..=max_num {
        k -= diff[dist as usize];
        if k <= 0 {
            return dist;
        }
    }
    -1
}

fn binary_search_dp(mut nums: Vec<i32>, k: i32) -> i32 {
    fn count_pairs(
        nums: &[i32],
        prefix_count: &[usize],
        value_count: &HashMap<i32, i32>,
        max_dist: i32,
    ) -> i32 {
        let (mut count, mut idx) = (0, 0);
        let size = nums.len();

        while idx < size {
            let curr = nums[idx];
            let curr_count = value_count[&curr];
            let pairs_with_larger =
                prefix_count[(curr + max_dist) as usize] - prefix_count[curr as usize];
            let pairs_with_same = curr_count * (curr_count - 1) / 2;
            count += pairs_with_larger as i32 * curr_count + pairs_with_same;

            while idx + 1 < size && nums[idx] == nums[idx + 1] {
                idx += 1;
            }
            idx += 1;
        }

        count
    }

    let size = nums.len();
    nums.sort_unstable();
    let max_num = nums.last().copied().unwrap_or(nums[0]);
    let max_dist = 2 * max_num;
    let mut prefix_count = vec![0; max_dist as usize];
    let value_count = nums.iter().fold(HashMap::new(), |mut acc, &num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    });

    let mut idx = 0;
    for value in 0..max_dist {
        while idx < size && nums[idx] <= value {
            idx += 1;
        }
        prefix_count[value as usize] = idx;
    }

    let (mut low, mut high) = (0, max_num);
    while low < high {
        let mid = (high - low) / 2 + low;
        let count = count_pairs(&nums, &prefix_count, &value_count, mid);
        if count < k {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    low
}

fn binary_search_sliding_window(mut nums: Vec<i32>, k: i32) -> i32 {
    fn count_pairs(nums: &[i32], max_dist: i32) -> i32 {
        let (mut count, mut left) = (0, 0);
        let size = nums.len();
        for right in 0..size {
            while nums[right] - nums[left] > max_dist {
                left += 1;
            }
            count += right - left;
        }
        count as i32
    }

    let size = nums.len();
    nums.sort_unstable();

    let mut low = 0;
    let mut high = nums[size - 1] - nums[0];
    while low < high {
        let mid = (high - low) / 2 + low;
        let count = count_pairs(&nums, mid);
        if count < k {
            low = mid + 1
        } else {
            high = mid;
        }
    }
    low
}

// fn is_palindrome(s: &str) -> bool {
//     if s.len() < 2 {
//         return true;
//     }
//     s.bytes()
//         .rev()
//         .zip(s.bytes().take(s.len() / 2 + 1))
//         .all(|(b1, b2)| b1 == b2)
// }

// type Coord = (usize, usize);

// fn neighbors((a, b): Coord) -> impl Iterator<Item = Coord> {
//     [
//         (a.saturating_sub(1), b),
//         (a + 1, b),
//         (a, b.saturating_sub(1)),
//         (a, b + 1),
//     ]
//     .into_iter()
//     .filter(move |&p| p != (a, b))
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(smallest_distance_pair(&[1, 3, 1], 1), 0);
        debug_assert_eq!(smallest_distance_pair(&[1, 1, 1], 2), 0);
        debug_assert_eq!(smallest_distance_pair(&[1, 6, 1], 3), 5);

        debug_assert_eq!(binary_search_dp(vec![1, 3, 1], 1), 0);
        debug_assert_eq!(binary_search_dp(vec![1, 1, 1], 2), 0);
        debug_assert_eq!(binary_search_dp(vec![1, 6, 1], 3), 5);

        debug_assert_eq!(binary_search_sliding_window(vec![1, 3, 1], 1), 0);
        debug_assert_eq!(binary_search_sliding_window(vec![1, 1, 1], 2), 0);
        debug_assert_eq!(binary_search_sliding_window(vec![1, 6, 1], 3), 5);
    }

    #[test]
    fn test() {}
}
