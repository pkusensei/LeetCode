mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_total_cost(nums: &[i32]) -> i64 {
    if nums.len() < 2 {
        return nums[0].into();
    }
    let [mut keep, mut flip] = [nums[0]; 2].map(i64::from);
    keep += i64::from(nums[1]);
    flip -= i64::from(nums[1]);
    for &num in nums[2..].iter() {
        let nkeep = keep.max(flip) + i64::from(num); // can always start new subarr
        let nflip = keep - i64::from(num); // but can only flip following +1
        [keep, flip] = [nkeep, nflip];
    }
    keep.max(flip)
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
        assert_eq!(maximum_total_cost(&[1, -1, 1, -1]), 4);
        assert_eq!(maximum_total_cost(&[1, -2, 3, 4]), 10);
        assert_eq!(maximum_total_cost(&[0]), 0);
    }

    #[test]
    fn test() {}
}
