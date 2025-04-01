mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_operations_to_empty_array(mut nums: Vec<i32>) -> i64 {
    use std::collections::HashMap;
    let n = nums.len();
    let pos: HashMap<_, _> = nums.iter().enumerate().map(|(i, &v)| (v, i)).collect();
    let mut res = n;
    nums.sort_unstable();
    for (i, w) in nums.windows(2).enumerate() {
        let [x, y] = w[..] else { unreachable!() };
        // x<y, remove x before y
        // but x appears after y in original array
        // removing x takes (1+i) moves => the length of array [.. x]
        // i.e the price to pay before moving anothing bigger than x
        if pos[&y] < pos[&x] {
            res += n - (1 + i);
        }
    }
    res as i64
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
        assert_eq!(count_operations_to_empty_array(vec![3, 4, -1]), 5);
        assert_eq!(count_operations_to_empty_array(vec![1, 2, 4, 3]), 5);
        assert_eq!(count_operations_to_empty_array(vec![1, 2, 3]), 3);
    }

    #[test]
    fn test() {}
}
