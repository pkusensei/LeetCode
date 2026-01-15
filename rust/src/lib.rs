mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximize_square_hole_area(
    _n: i32,
    _m: i32,
    mut h_bars: Vec<i32>,
    mut v_bars: Vec<i32>,
) -> i32 {
    h_bars.sort_unstable();
    v_bars.sort_unstable();
    let [hmax, vmax] = [&h_bars, &v_bars].map(|v| {
        v.chunk_by(|a, b| 1 + a == *b)
            .flat_map(|ch| ch.last().zip(ch.first()).map(|(y, x)| y - x))
            .max()
            .unwrap_or_default()
    });
    (2 + hmax.min(vmax)).pow(2)
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
