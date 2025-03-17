mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn largest_palindromic(num: &str) -> String {
    use std::collections::VecDeque;
    let count = num.bytes().fold([0; 10], |mut acc, b| {
        acc[usize::from(b - b'0')] += 1;
        acc
    });
    let mut left = VecDeque::new();
    let mut right = VecDeque::new();
    for (idx, &c) in count.iter().enumerate().rev() {
        if c >= 2 {
            let ch = char::from(idx as u8 + b'0');
            for _ in 0..c / 2 {
                left.push_back(ch);
                right.push_front(ch);
            }
        }
    }
    while left.front().is_some_and(|&v| v == '0') {
        left.pop_front();
        right.pop_back();
    }
    if let Some(i) = count
        .iter()
        .enumerate()
        .rev()
        .find_map(|(i, &c)| if c & 1 == 1 { Some(i) } else { None })
    {
        let ch = char::from(i as u8 + b'0');
        left.push_back(ch);
    }
    let res: String = left.into_iter().chain(right).collect();
    if res.is_empty() { "0".into() } else { res }
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
        assert_eq!(largest_palindromic("444947137"), "7449447");
        assert_eq!(largest_palindromic("00009"), "9");
    }

    #[test]
    fn test() {}
}
