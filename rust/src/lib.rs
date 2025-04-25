mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn largest_square_area(bottom_left: &[[i32; 2]], top_right: &[[i32; 2]]) -> i64 {
    use itertools::{Itertools, izip};
    let mut res = 0;
    for [(bl1, tr1), (bl2, tr2)] in
        izip!(bottom_left.iter(), top_right.iter()).array_combinations::<2>()
    {
        let [x1, y1] = bl1[..] else { unreachable!() };
        let [x2, y2] = tr1[..] else { unreachable!() };
        let [x3, y3] = bl2[..] else { unreachable!() };
        let [x4, y4] = tr2[..] else { unreachable!() };
        let dx = x2.min(x4) - x1.max(x3);
        let dy = y2.min(y4) - y1.max(y3);
        let side = dx.min(dy);
        if side < 0 {
            continue;
        }
        res = res.max(i64::from(side).pow(2));
    }
    res
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
        assert_eq!(
            largest_square_area(&[[1, 1], [1, 3], [1, 5]], &[[5, 5], [5, 7], [5, 9]]),
            4
        );
    }

    #[test]
    fn test() {}
}
