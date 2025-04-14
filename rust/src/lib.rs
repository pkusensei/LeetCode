mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn last_visited_integers(nums: Vec<i32>) -> Vec<i32> {
    use std::collections::VecDeque;
    let mut seen = VecDeque::new();
    let mut idx = 0;
    let mut res = vec![];
    for &num in nums.iter() {
        if num != -1 {
            seen.push_front(num);
            idx = 0;
        } else {
            res.push(*seen.get(idx).unwrap_or(&-1));
            idx += 1;
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
