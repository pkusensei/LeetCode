mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximize_square_hole_area(
    n: i32,
    m: i32,
    mut h_bars: Vec<i32>,
    mut v_bars: Vec<i32>,
) -> i32 {
    h_bars.sort_unstable();
    v_bars.sort_unstable();
    let a = h_bars.chunk_by(|a, b| b - a == 1).map(|ch| ch.len()).max();
    let b = v_bars.chunk_by(|a, b| b - a == 1).map(|ch| ch.len()).max();
    a.zip(b)
        .map(|(a, b)| (a.min(b) as i32 + 1).pow(2))
        .unwrap_or(1)
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
