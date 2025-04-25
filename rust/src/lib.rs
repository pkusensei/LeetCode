mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn earliest_second_to_mark_indices(nums: &[i32], change_indices: &[i32]) -> i32 {
    let m = change_indices.len() as i32;
    let sum: i32 = nums.iter().sum();
    let mut left = sum;
    let mut right = 1 + change_indices.len() as i32;
    while left < right {
        let mid = left + (right - left) / 2;
        if check(nums, change_indices, mid) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    if left <= m { left } else { -1 }
}

fn check(nums: &[i32], change_indices: &[i32], mid: i32) -> bool {
    let mut latest = vec![-1; nums.len()];
    for i in 0..mid {
        latest[change_indices[i as usize] as usize - 1] = i;
    }
    let mut marked = 0;
    let mut decrement = 0;
    for i in 0..mid {
        let idx = change_indices[i as usize] as usize - 1;
        if i == latest[idx] {
            if decrement < nums[idx] {
                return false;
            }
            marked += 1;
            decrement -= nums[idx];
        } else {
            decrement += 1;
        }
    }
    marked as usize == nums.len()
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
        assert_eq!(
            earliest_second_to_mark_indices(&[2, 2, 0], &[2, 2, 2, 2, 3, 2, 2, 1],),
            8
        );
        assert_eq!(
            earliest_second_to_mark_indices(&[1, 3], &[1, 1, 1, 2, 1, 1, 1]),
            6
        );
        assert_eq!(earliest_second_to_mark_indices(&[0, 1], &[2, 2, 2]), -1);
    }

    #[test]
    fn test() {
        assert_eq!(
            earliest_second_to_mark_indices(&[0, 2, 3, 0], &[2, 4, 1, 3, 3, 3, 3, 3, 3, 2, 1]),
            10
        );
        assert_eq!(
            earliest_second_to_mark_indices(&[1, 0, 3], &[1, 1, 3, 2]),
            -1
        )
    }
}
