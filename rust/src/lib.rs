mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn longest_beautiful_substring(word: String) -> i32 {
    let mut res = 0;
    let mut curr = 1;
    let mut count = 1;
    for (idx, b) in word.bytes().enumerate().skip(1) {
        match word.as_bytes()[idx - 1].cmp(&b) {
            std::cmp::Ordering::Less => {
                curr += 1;
                count += 1
            }
            std::cmp::Ordering::Equal => curr += 1,
            std::cmp::Ordering::Greater => {
                curr = 1;
                count = 1
            }
        }
        if count == 5 {
            res = res.max(curr)
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
