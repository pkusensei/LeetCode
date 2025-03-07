mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_deletion(nums: &[i32]) -> i32 {
    let mut stack = vec![];
    let mut res = 0;
    'outer: for &num in nums.iter() {
        if stack.len() & 1 == 0 {
            stack.push(num);
        } else {
            while stack.last().is_some_and(|&v| v == num) {
                res += 1;
                continue 'outer;
            }
            stack.push(num);
        }
    }
    res + (stack.len() & 1) as i32
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
        assert_eq!(min_deletion(&[1, 1, 2, 3, 5]), 1);
        assert_eq!(min_deletion(&[1, 1, 2, 2, 3, 3]), 2);
    }

    #[test]
    fn test() {}
}
