mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn robot_with_string(s: &str) -> String {
    let mut count = s
        .bytes()
        .fold(std::collections::HashMap::new(), |mut acc, b| {
            *acc.entry(b).or_insert(0) += 1;
            acc
        });
    let mut t = vec![];
    let mut p = vec![];
    let mut smaller = b'a';
    for b in s.bytes() {
        t.push(b);
        count.entry(b).and_modify(|v| *v -= 1);
        while smaller < b'z' && count.get(&smaller).is_none_or(|&v| v == 0) {
            smaller += 1; // all chars <= smaller that are exhausted by this point
        }
        while t.last().is_some_and(|&v| v <= smaller) {
            p.push(t.pop().unwrap());
        }
    }
    String::from_utf8(p).unwrap()
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
        assert_eq!(robot_with_string("zza"), "azz");
        assert_eq!(robot_with_string("bac"), "abc");
        assert_eq!(robot_with_string("bdda"), "addb");
    }

    #[test]
    fn test() {}
}
