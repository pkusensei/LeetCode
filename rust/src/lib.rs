mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_consecutive_answers(answer_key: &str, k: i32) -> i32 {
    slide(answer_key.as_bytes(), k, b'F').max(slide(answer_key.as_bytes(), k, b'T'))
}

fn slide(answer_key: &[u8], k: i32, byte: u8) -> i32 {
    let mut left = 0;
    let mut res = 0;
    let mut missed = 0;
    for (right, &b) in answer_key.iter().enumerate() {
        if b == byte {
            missed += 1;
        }
        while missed > k {
            missed -= i32::from(answer_key[left] == byte);
            left += 1;
        }
        res = res.max((right + 1 - left) as i32);
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
    fn basics() {
        assert_eq!(max_consecutive_answers("TTFF", 2), 4);
        assert_eq!(max_consecutive_answers("TFFT", 1), 3);
        assert_eq!(max_consecutive_answers("TTFTTFTT", 1), 5);
    }

    #[test]
    fn test() {}
}
