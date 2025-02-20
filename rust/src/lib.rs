mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn interchangeable_rectangles(rectangles: Vec<Vec<i32>>) -> i64 {
    let mut map = std::collections::HashMap::new();
    for rect in rectangles.iter() {
        let bits = (f64::from(rect[0]) / f64::from(rect[1])).to_bits();
        *map.entry(bits).or_insert(0) += 1;
    }
    map.into_values().map(|v| v * (v - 1) / 2).sum()
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
