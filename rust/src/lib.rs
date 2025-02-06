mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn second_highest(s: String) -> i32 {
    let mut digits: Vec<_> = s
        .bytes()
        .fold([false; 10], |mut acc, b| {
            if b.is_ascii_digit() {
                acc[usize::from(b - b'0')] = true;
            }
            acc
        })
        .into_iter()
        .enumerate()
        .filter_map(|(digit, v)| if v { Some(digit as i32) } else { None })
        .collect();
    if digits.len() < 2 {
        return -1;
    }
    let (_, res, _) = digits.select_nth_unstable_by_key(1, |&v| std::cmp::Reverse(v));
    *res
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
