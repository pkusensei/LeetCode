mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn largest_perimeter(mut nums: Vec<i32>) -> i64 {
    nums.sort_unstable();
    let mut res = -1;
    let mut prefix = 0;
    for (count, &num) in nums.iter().enumerate() {
        let num = i64::from(num);
        if count >= 2 && prefix > num {
            res = res.max(prefix + num);
        }
        prefix += num;
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
        assert_eq!(largest_perimeter(vec![5, 5, 5]), 15);
        assert_eq!(largest_perimeter(vec![1, 12, 1, 2, 5, 50, 3]), 12);
        assert_eq!(largest_perimeter(vec![5, 5, 50]), -1);
    }

    #[test]
    fn test() {}
}
