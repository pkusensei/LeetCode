mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_operations(nums: Vec<i32>, locked: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let mut pref_max = 0;
    let mut unlock = 0;
    for (i, &num) in nums.iter().enumerate() {
        pref_max = pref_max.max(num);
        if locked[i] == 1 && !(1 + i == num as usize && num == pref_max) {
            unlock += 1;
            if unlock > k {
                return -1;
            }
        }
    }
    let mut swap = 0;
    let mut arr = Vec::with_capacity(n);
    for &num in nums.iter() {
        let i = arr.partition_point(|&v| v <= num);
        swap += (arr.len() - i) as i32;
        arr.insert(i, num);
    }
    // for left in 0..n {
    //     for right in 1 + left..n {
    //         if nums[left] > nums[right] {
    //             swap += 1;
    //         }
    //     }
    // }
    swap + unlock
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
