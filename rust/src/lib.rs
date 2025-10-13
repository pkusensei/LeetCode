mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_sum_of_three_subarrays(nums: &[i32], k: i32) -> [usize; 3] {
    let k = k as usize;
    let mut sums = vec![];
    let mut window = 0;
    for (i, &num) in nums.iter().enumerate() {
        window += num;
        if i >= k {
            window -= nums[i - k];
        }
        if i >= k - 1 {
            sums.push(window);
        }
    }
    let mut max_i = 0;
    let mut left = vec![];
    for (i, &v) in sums.iter().enumerate() {
        if v > sums[max_i] {
            max_i = i;
        }
        left.push(max_i);
    }
    let mut right = vec![];
    max_i = sums.len() - 1;
    for (i, &v) in sums.iter().enumerate().rev() {
        // smallest idx
        if v >= sums[max_i] {
            max_i = i;
        }
        right.push(max_i);
    }
    right.reverse();
    let mut max_sum = 0;
    let mut res = [0, 0, 0];
    for mid in k..sums.len() - k {
        let curr = sums[mid] + sums[left[mid - k]] + sums[right[mid + k]];
        if curr > max_sum {
            max_sum = curr;
            res = [left[mid - k], mid, right[mid + k]];
        }
    }
    res
}

pub fn with_three_windows(nums: &[i32], k: i32) -> [usize; 3] {
    let n = nums.len();
    let k = k as usize;
    let mut best_seq1 = [0];
    let mut best_seq2 = [0, k];
    let mut best_seq3 = [0, k, 2 * k];
    let mut sum1: i32 = nums[..k].iter().sum();
    let mut sum2: i32 = nums[k..2 * k].iter().sum();
    let mut sum3: i32 = nums[2 * k..3 * k].iter().sum();
    let mut max_sum1 = sum1;
    let mut max_sum2 = sum1 + sum2;
    let mut max_sum3 = sum1 + sum2 + sum3;
    let mut i1 = 1;
    let mut i2 = 1 + k;
    for i3 in 1 + 2 * k..=n - k {
        sum1 += nums[i1 + k - 1] - nums[i1 - 1];
        sum2 += nums[i2 + k - 1] - nums[i2 - 1];
        sum3 += nums[i3 + k - 1] - nums[i3 - 1];
        if sum1 > max_sum1 {
            max_sum1 = sum1;
            best_seq1 = [i1];
        }
        if sum2 + max_sum1 > max_sum2 {
            max_sum2 = sum2 + max_sum1;
            best_seq2 = [best_seq1[0], i2];
        }
        if sum3 + max_sum2 > max_sum3 {
            max_sum3 = sum3 + max_sum2;
            best_seq3 = [best_seq2[0], best_seq2[1], i3];
        }
        i1 += 1;
        i2 += 1;
    }
    best_seq3
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
        assert_eq!(
            max_sum_of_three_subarrays(&[1, 2, 1, 2, 6, 7, 5, 1], 2),
            [0, 3, 5]
        );
        assert_eq!(
            max_sum_of_three_subarrays(&[1, 2, 1, 2, 1, 2, 1, 2, 1], 2),
            [0, 2, 4]
        );

        assert_eq!(with_three_windows(&[1, 2, 1, 2, 6, 7, 5, 1], 2), [0, 3, 5]);
        assert_eq!(
            with_three_windows(&[1, 2, 1, 2, 1, 2, 1, 2, 1], 2),
            [0, 2, 4]
        );
    }

    #[test]
    fn test() {}
}
