mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn distinct_names(ideas: &[&str]) -> i64 {
    use std::collections::HashSet;
    let map = ideas.iter().fold(vec![HashSet::new(); 26], |mut acc, s| {
        let [head, tail @ ..] = s.as_bytes() else {
            unreachable!()
        };
        acc[usize::from(head - b'a')].insert(tail);
        acc
    });
    let mut res = 0;
    for (i, a) in map.iter().enumerate() {
        for b in map.iter().skip(1 + i) {
            let common = a.intersection(b).count();
            res += (a.len() - common) * (b.len() - common);
        }
    }
    (2 * res) as _
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
        assert_eq!(distinct_names(&["coffee", "donuts", "time", "toffee"]), 6);
        assert_eq!(distinct_names(&["lack", "back"]), 0);
    }

    #[test]
    fn test() {}
}
