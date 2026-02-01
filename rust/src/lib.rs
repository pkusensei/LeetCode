mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn longest_alternating(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut left_len = Vec::with_capacity(n);
    left_len.push([1, 1]);
    let mut res = 1;
    // [0] == <
    // [1] == >
    for w in nums.windows(2) {
        let last = left_len.last().unwrap();
        let curr = match w[0].cmp(&w[1]) {
            std::cmp::Ordering::Less => [1 + last[1], 1],
            std::cmp::Ordering::Equal => [1, 1],
            std::cmp::Ordering::Greater => [1, 1 + last[0]],
        };
        res = res.max(curr[0]).max(curr[1]);
        left_len.push(curr);
    }
    let mut right_len = Vec::with_capacity(n);
    right_len.push([1, 1]);
    for w in nums.windows(2).rev() {
        let last = right_len.last().unwrap();
        let curr = match w[0].cmp(&w[1]) {
            std::cmp::Ordering::Less => [1 + last[1], 1],
            std::cmp::Ordering::Equal => [1, 1],
            std::cmp::Ordering::Greater => [1, 1 + last[0]],
        };
        right_len.push(curr);
    }
    right_len.reverse();
    for del in 1..n - 1 {
        let curr = match nums[del - 1].cmp(&nums[1 + del]) {
            std::cmp::Ordering::Less => left_len[del - 1][1] + right_len[1 + del][1],
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => left_len[del - 1][0] + right_len[1 + del][0],
        };
        res = res.max(curr);
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
    fn basics() {
        assert_eq!(longest_alternating(&[3, 2, 1, 2, 3, 2, 1]), 4);
        assert_eq!(longest_alternating(&[2, 1, 3, 2]), 4);
        assert_eq!(longest_alternating(&[100000, 100000]), 1);
    }

    #[test]
    fn test() {}
}
