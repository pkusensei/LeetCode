mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn have_conflict(event1: Vec<String>, event2: Vec<String>) -> bool {
    let mut map = std::collections::BTreeMap::new();
    *map.entry(parse(&event1[0])).or_insert(0) += 1;
    *map.entry(parse(&event1[1]) + 1).or_insert(0) -= 1;
    *map.entry(parse(&event2[0])).or_insert(0) += 1;
    *map.entry(parse(&event2[1]) + 1).or_insert(0) -= 1;
    let mut curr = 0;
    for v in map.values() {
        curr += v;
        if curr > 1 {
            return true;
        }
    }
    false
}

fn parse(s: &str) -> i32 {
    let (h, m) = s.split_once(':').unwrap();
    h.parse::<i32>().unwrap_or(0) * 60 + m.parse::<i32>().unwrap_or(0)
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
