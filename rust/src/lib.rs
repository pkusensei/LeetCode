mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
    let [mut arr1, mut arr2] = [vec![nums[0]], vec![nums[1]]];
    let [mut prev1, mut prev2] = [nums[0], nums[1]];
    for &num in &nums[2..] {
        if prev1 > prev2 {
            arr1.push(num);
            prev1 = num;
        } else {
            arr2.push(num);
            prev2 = num;
        }
    }
    arr1.extend(arr2);
    arr1
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
