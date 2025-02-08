mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_the_winner(n: i32, k: i32) -> i32 {
    let mut nums: Vec<_> = (1..=n).collect();
    let mut start = 0;
    let k = k as usize;
    while nums.len() > 1 {
        let idx = (start + k - 1) % nums.len();
        nums.remove(idx);
        start = idx;
    }
    nums[0]
}

pub fn with_recursion(n: i32, k: i32) -> i32 {
    fn solve(n: i32, k: i32) -> i32 {
        if n == 1 {
            0
        } else {
            (solve(n - 1, k) + k) % n
        }
    }
    solve(n, k) + 1
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
        assert_eq!(find_the_winner(5, 2), 3);
        assert_eq!(find_the_winner(6, 5), 1);

        assert_eq!(with_recursion(5, 2), 3);
        assert_eq!(with_recursion(6, 5), 1);
    }

    #[test]
    fn test() {}
}
