mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_anagram_length(s: &str) -> i32 {
    use itertools::Itertools;
    let n = s.len();
    let func = |mut acc: [i32; 26], b| {
        acc[usize::from(b - b'a')] += 1;
        acc
    };
    let freq = s
        .bytes()
        .fold([0; 26], func)
        .into_iter()
        .filter(|&v| v > 0)
        .collect_vec();
    let gcd_ = freq.iter().fold(freq[0], |acc, &v| gcd(acc, v));
    let window = freq.iter().map(|&v| v / gcd_).sum::<i32>() as usize;
    for curr in (window..n).step_by(window) {
        if s.as_bytes()
            .chunks(curr)
            .map(|w| w.iter().copied().fold([0; 26], func))
            .all_equal()
        {
            return curr as i32;
        }
    }
    n as i32
}

const fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 { b } else { gcd(b % a, a) }
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
    fn test() {
        assert_eq!(min_anagram_length("ccccaaaa"), 8);
    }
}
