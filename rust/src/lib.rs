mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn make_similar(nums: &[i32], target: &[i32]) -> i64 {
    let [num_e, num_o] = split(nums);
    let [target_e, target_o] = split(target);
    (count(&num_e, &target_e) + count(&num_o, &target_o)) / 2
}

fn count(arr1: &[i32], arr2: &[i32]) -> i64 {
    arr1.iter()
        .zip(arr2)
        .map(|(a, b)| i64::from(a.abs_diff(*b) / 2))
        .sum::<i64>()
}

fn split(arr: &[i32]) -> [Vec<i32>; 2] {
    let mut evens = vec![];
    let mut odds = vec![];
    for &num in arr {
        if num & 1 == 0 {
            evens.push(num);
        } else {
            odds.push(num);
        }
    }
    evens.sort_unstable();
    odds.sort_unstable();
    [evens, odds]
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
        assert_eq!(make_similar(&[8, 12, 6], &[2, 14, 10]), 2);
        assert_eq!(make_similar(&[1, 2, 5], &[4, 1, 3]), 1);
    }

    #[test]
    fn test() {}
}
