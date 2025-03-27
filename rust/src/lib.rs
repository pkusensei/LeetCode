mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_score(s: &str, t: &str) -> i32 {
    let sn = s.len();
    let (t, tn) = (t.as_bytes(), t.len());
    let mut left = Vec::with_capacity(1 + sn);
    left.push(0);
    let mut count = 0;
    for b in s.bytes() {
        if t.get(count).is_some_and(|&v| v == b) {
            count += 1;
        }
        left.push(count as i32);
    }
    let mut right = Vec::with_capacity(1 + sn);
    right.push(0);
    count = 0;
    for b in s.bytes().rev() {
        if count < tn && t.get(tn - 1 - count).is_some_and(|&v| v == b) {
            count += 1;
        }
        right.push(count as i32);
    }
    right.reverse();
    let tn = tn as i32;
    let mut res = tn;
    for (a, b) in left.into_iter().zip(right) {
        if a + b < tn {
            res = res.min(tn - a - b);
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
    fn basics() {
        assert_eq!(minimum_score("abacaba", "bzaa"), 1);
        assert_eq!(minimum_score("cde", "xyz"), 3);
    }

    #[test]
    fn test() {
        assert_eq!(minimum_score("abecdebe", "eaebceae"), 6);
    }
}
