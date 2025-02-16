mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn can_see_persons_count(heights: &[i32]) -> Vec<i32> {
    let mut res = vec![];
    let mut stack = vec![];
    for &num in heights.iter().rev() {
        let mut curr = 0;
        while stack.last().is_some_and(|&v| v < num) {
            stack.pop();
            curr += 1;
        }
        curr += i32::from(stack.last().is_some());
        stack.push(num);
        res.push(curr);
    }
    res.reverse();
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
        assert_eq!(
            can_see_persons_count(&[10, 6, 8, 5, 11, 9]),
            [3, 1, 2, 1, 1, 0]
        );
        assert_eq!(can_see_persons_count(&[5, 1, 2, 3, 10]), [4, 1, 1, 1, 0]);
    }

    #[test]
    fn test() {}
}
