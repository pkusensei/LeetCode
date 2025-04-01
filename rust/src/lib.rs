mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn is_winner(player1: Vec<i32>, player2: Vec<i32>) -> i32 {
    let a = solve(&player1);
    let b = solve(&player2);
    match a.cmp(&b) {
        std::cmp::Ordering::Less => 2,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

fn solve(nums: &[i32]) -> i32 {
    let mut res = 0;
    let mut double = 0;
    for &num in nums {
        if double > 0 {
            res += 2 * num;
            double -= 1;
        } else {
            res += num;
        }
        if num == 10 {
            double = 2
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
