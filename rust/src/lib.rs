mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_card_pickup(cards: Vec<i32>) -> i32 {
    let mut res = i32::MAX;
    let mut seen = std::collections::HashMap::new();
    for (i, &num) in cards.iter().enumerate() {
        if let Some(prev) = seen.get(&num) {
            res = res.min((1 + i - prev) as i32);
        }
        seen.insert(num, i);
    }
    if res == i32::MAX {
        -1
    } else {
        res
    }
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
