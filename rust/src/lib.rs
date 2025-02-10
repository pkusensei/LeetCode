mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_sum_min_product(nums: &[i32]) -> i32 {
    let n = nums.len();
    let it = nums.iter().copied().enumerate();
    let left_smallers = smallers(nums, it.clone());
    let mut right_smallers = smallers(nums, it.rev());
    right_smallers.reverse();
    let prefix = nums.iter().fold(vec![0], |mut acc, &num| {
        acc.push(i64::from(num) + acc.last().unwrap_or(&0));
        acc
    });
    let mut res = 0;
    for (i, (left, right)) in left_smallers.into_iter().zip(right_smallers).enumerate() {
        let left = left.map(|v| v + 1).unwrap_or(0);
        let right = right.unwrap_or(n);
        res = res.max(i64::from(nums[i]) * (prefix[right] - prefix[left]));
    }
    (res % 1_000_000_007) as _
}

fn smallers(nums: &[i32], it: impl Iterator<Item = (usize, i32)>) -> Vec<Option<usize>> {
    let n = nums.len();
    let mut stack = vec![];
    let mut res = Vec::with_capacity(n);
    for (idx, num) in it {
        // Suppose it's scanning from right to left
        // For current (idx, num) pair, pop everything that's bigger from stack
        // Top of stack is the next smaller element
        while stack.last().is_some_and(|&i| nums[i] >= num) {
            stack.pop();
        }
        // None means this num is the min in [idx..]
        res.push(stack.last().copied());
        stack.push(idx);
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
        assert_eq!(max_sum_min_product(&[1, 2, 3, 2]), 14);
        assert_eq!(max_sum_min_product(&[2, 3, 3, 1, 2]), 18);
        assert_eq!(max_sum_min_product(&[3, 1, 5, 6, 4, 2]), 60);
    }

    #[test]
    fn test() {}
}
