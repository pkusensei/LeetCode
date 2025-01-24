mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn visible_points(points: &[[i32; 2]], angle: i32, location: [i32; 2]) -> i32 {
    let mut polars = vec![];
    let mut count: i32 = 0;
    for p in points.iter() {
        if p == &location {
            count += 1;
        } else {
            let dx = f64::from(p[0] - location[0]);
            let dy = f64::from(p[1] - location[1]);
            polars.push(dy.atan2(dx).to_degrees());
        }
    }
    polars.sort_unstable_by(|a, b| a.total_cmp(b));
    let n = polars.len();
    polars.extend_from_within(..);
    for v in polars[n..].iter_mut() {
        *v += 360.0;
    }
    let angle = f64::from(angle);
    let mut left = 0;
    let mut res = 0;
    for (right, &val) in polars.iter().enumerate() {
        while val - polars[left] > angle {
            left += 1;
        }
        res = res.max(right + 1 - left);
    }
    res as i32 + count
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    #[test]
    fn basics() {
        assert_eq!(visible_points(&[[2, 1], [2, 2], [3, 3]], 90, [1, 1]), 3);
        assert_eq!(
            visible_points(&[[2, 1], [2, 2], [3, 4], [1, 1]], 90, [1, 1]),
            4
        );
        assert_eq!(visible_points(&[[1, 0], [2, 1]], 13, [1, 1]), 1);
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
