mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn split_into_fibonacci(num: String) -> Vec<i32> {
    let mut curr = vec![];
    if backtrack(&num, &mut curr) {
        curr.into_iter().map(|v| v as i32).collect()
    } else {
        vec![]
    }
}

fn backtrack(s: &str, curr: &mut Vec<i64>) -> bool {
    if s.is_empty() && curr.len() >= 3 {
        return true;
    }
    for end in 1..=s.len() {
        if end > 1 && s.starts_with('0') {
            return false;
        }
        let Ok(num) = s[..end].parse::<i32>() else {
            return false;
        };
        if curr.len() < 2 || curr.iter().rev().take(2).sum::<i64>() == i64::from(num) {
            curr.push(i64::from(num));
            if backtrack(&s[end..], curr) {
                return true;
            }
            curr.pop();
        }
    }
    false
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

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
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
