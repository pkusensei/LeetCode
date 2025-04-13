mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_sum_of_heights(heights: &[i32]) -> i64 {
    let n = heights.len();
    let mut res = 0;
    for (peak, &num) in heights.iter().enumerate() {
        let mut curr = i64::from(num);
        let mut temp = num;
        for left in (0..peak).rev() {
            temp = temp.min(heights[left]);
            curr += i64::from(temp);
        }
        temp = num;
        for right in 1 + peak..n {
            temp = temp.min(heights[right]);
            curr += i64::from(temp);
        }
        res = res.max(curr)
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
        assert_eq!(maximum_sum_of_heights(&[6, 5, 3, 9, 2, 7]), 22);
    }

    #[test]
    fn test() {}
}
