mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn largest_word_count(messages: Vec<String>, senders: Vec<String>) -> String {
    let mut map = std::collections::HashMap::new();
    for (m, s) in messages.iter().zip(senders) {
        *map.entry(s).or_insert(0) += m.split_ascii_whitespace().count();
    }
    map.into_iter()
        .max_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)))
        .map(|i| i.0)
        .unwrap_or_default()
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
