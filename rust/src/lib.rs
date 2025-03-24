mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn top_students(
    positive_feedback: Vec<String>,
    negative_feedback: Vec<String>,
    report: Vec<String>,
    student_id: Vec<i32>,
    k: i32,
) -> Vec<i32> {
    use itertools::Itertools;
    use std::collections::HashSet;
    let pos: HashSet<_> = positive_feedback.iter().map(|s| s.as_str()).collect();
    let neg: HashSet<_> = negative_feedback.iter().map(|s| s.as_str()).collect();
    report
        .iter()
        .zip(student_id.iter())
        .map(|(rep, id)| {
            let words = rep.split_ascii_whitespace();
            let p = words.clone().filter(|w| pos.contains(w)).count() as i32;
            let n = words.filter(|w| neg.contains(w)).count() as i32;
            (3 * p - n, *id)
        })
        .sorted_unstable_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)))
        .take(k as usize)
        .map(|(_, id)| id)
        .collect()
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
