mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_array_sum(mut nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut min = i32::MAX;
    let mut max = 0;
    for &num in nums.iter() {
        min = min.min(num);
        max = max.max(num);
    }
    if min == 1 {
        return n as i64;
    }
    let mut small = vec![None; 1 + max as usize];
    nums.sort_unstable();
    let mut res = 0;
    for num in nums {
        if let Some(p) = small[num as usize] {
            res += i64::from(p)
        } else {
            let p = num as usize;
            for val in (p..=max as usize).step_by(p) {
                small[val].get_or_insert(num);
            }
            res += i64::from(num);
        }
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
    fn basics() {}

    #[test]
    fn test() {}
}
