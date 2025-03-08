mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

// https://cp-algorithms.com/string/z-function.html
pub fn sum_scores(s: &str) -> i64 {
    let (s, n) = (s.as_bytes(), s.len());
    let mut z = vec![0; n];
    let [mut left, mut right] = [0, 0];
    for idx in 1..n {
        if idx < right {
            z[idx] = (right - idx).min(z[idx - left]);
        }
        while s.get(idx + z[idx]).is_some_and(|&v| v == s[z[idx]]) {
            z[idx] += 1;
        }
        if idx + z[idx] > right {
            left = idx;
            right = idx + z[idx];
        }
    }
    (z.iter().sum::<usize>() + n) as i64
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
        assert_eq!(sum_scores("babab"), 9);
        assert_eq!(sum_scores("azbazbzaz"), 14);
    }

    #[test]
    fn test() {}
}
