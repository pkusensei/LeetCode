mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn punishment_number(n: i32) -> i32 {
    (1..=n)
        .filter_map(|num| {
            if (0..=1).contains(&(num % 9)) && backtrack(num, num.pow(2)) {
                Some(num.pow(2))
            } else {
                None
            }
        })
        .sum()
}

// lol
const ST: [i32; 29] = [
    1, 9, 10, 36, 45, 55, 82, 91, 99, 100, 235, 297, 369, 370, 379, 414, 657, 675, 703, 756, 792,
    909, 918, 945, 964, 990, 991, 999, 1000,
];

const fn backtrack(target: i32, num: i32) -> bool {
    if target < 0 || num < target {
        return false;
    }
    if num == target {
        return true;
    }
    backtrack(target - num % 10, num / 10)
        || backtrack(target - num % 100, num / 100)
        || backtrack(target - num % 1000, num / 1000)
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
        assert_eq!(punishment_number(10), 182);
        assert_eq!(punishment_number(37), 1478);
    }

    #[test]
    fn test() {}
}
