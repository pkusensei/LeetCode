mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn sum_game(num: &str) -> bool {
    let n = num.len();
    let mut sum = 0;
    let [mut left, mut right] = [0, 0];
    for b in num[..n / 2].bytes() {
        if b == b'?' {
            left += 1
        } else {
            sum += i32::from(b - b'0');
        }
    }
    for b in num[n / 2..].bytes() {
        if b == b'?' {
            right += 1
        } else {
            sum -= i32::from(b - b'0');
        }
    }
    if (left + right) & 1 == 1 {
        return true;
    }
    sum += left / 2 * 9 - right / 2 * 9;
    sum != 0
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
        assert!(!sum_game("5023"));
        assert!(sum_game("25??"));
        assert!(!sum_game("?3295???"));
    }

    #[test]
    fn test() {}
}
