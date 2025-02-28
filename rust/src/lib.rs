mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_points(rings: String) -> i32 {
    let mut count = [0; 10];
    for w in rings.as_bytes().chunks(2) {
        let [color, rod] = w[..] else { unreachable!() };
        let idx = usize::from(rod - b'0');
        match color {
            b'R' => count[idx] |= 0b001,
            b'G' => count[idx] |= 0b010,
            _ => count[idx] |= 0b100,
        }
    }
    count.into_iter().filter(|&v| v == 0b111).count() as _
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
