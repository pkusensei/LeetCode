mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn kth_remaining_integer(nums: &[i32], queries: &[[i32; 3]]) -> Vec<i32> {
    let pref_even = nums.iter().fold(vec![], |mut acc, v| {
        acc.push(i32::from(v & 1 == 0) + acc.last().unwrap_or(&0));
        acc
    });
    // let mut next_even = vec![];
    // let mut temp = None;
    // for &num in nums.iter().rev() {
    //     if num & 1 == 0 {
    //         temp = Some(num)
    //     }
    //     next_even.push(temp);
    // }
    // next_even.reverse();
    let mut res = vec![];
    for q in queries.iter() {
        let [left, right] = [0, 1].map(|i| q[i] as usize);
        let k = q[2];
        // let count = pref_even[right] - if left > 0 { pref_even[left - 1] } else { 0 };
        // if count == 0 || next_even[left].is_some_and(|v| v > 2 * k) {
        //     res.push(2 * k);
        //     continue;
        // }
        let mut low = 1;
        let mut high = i32::MAX >> 1;
        while low < high {
            let mid = low + (high - low) / 2;
            if f(&nums, &pref_even, left, right, k, mid) {
                high = mid
            } else {
                low = 1 + mid;
            }
        }
        res.push(2 * low);
    }
    res
}

fn f(nums: &[i32], pref_even: &[i32], left: usize, right: usize, k: i32, mid: i32) -> bool {
    let i = nums.partition_point(|&v| v <= 2 * mid);
    let mut even_count = 0;
    if i > left {
        let right = right.min(i - 1);
        even_count = pref_even[right] - if left > 0 { pref_even[left - 1] } else { 0 };
    }
    mid - even_count >= k
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
            kth_remaining_integer(&[1, 4, 7], &[[0, 2, 1], [1, 1, 2], [0, 0, 3]]),
            [2, 6, 6]
        );
    }

    #[test]
    fn test() {}
}
