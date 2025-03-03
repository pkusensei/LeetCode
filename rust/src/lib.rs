mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_score_indices(nums: Vec<i32>) -> Vec<i32> {
    let mut left_zeros = 0;
    let mut right_ones: i32 = nums.iter().sum();
    let mut score = left_zeros + right_ones;
    let mut res = vec![0];
    for (idx, &num) in (1..).zip(nums.iter()) {
        left_zeros += i32::from(num == 0);
        right_ones -= i32::from(num == 1);
        let curr = left_zeros + right_ones;
        match curr.cmp(&score) {
            std::cmp::Ordering::Less => (),
            std::cmp::Ordering::Equal => res.push(idx),
            std::cmp::Ordering::Greater => {
                score = curr;
                res = vec![idx];
            }
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
    fn basics() {}

    #[test]
    fn test() {}
}
