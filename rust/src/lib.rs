mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimize_result(expression: String) -> String {
    let (left, right) = expression.split_once('+').unwrap();
    let mut val = u64::MAX;
    let mut res = String::new();
    for [left1, left2] in split(left) {
        if left2.is_empty() {
            continue;
        }
        let [a1, a2] = [left1, left2].map(|v| v.parse::<u64>().unwrap_or(1));
        for [right1, right2] in split(right) {
            if right1.is_empty() {
                continue;
            }
            let [b1, b2] = [right1, right2].map(|v| v.parse::<u64>().unwrap_or(1));
            if a1 * (a2 + b1) * b2 < val {
                val = a1 * (a2 + b1) * b2;
                res = format!("{}({}+{}){}", left1, left2, right1, right2);
            }
        }
    }
    res
}

fn split(s: &str) -> impl Iterator<Item = [&str; 2]> {
    let n = s.len();
    (0..=n).map(|i| [&s[..i], &s[i..]])
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
