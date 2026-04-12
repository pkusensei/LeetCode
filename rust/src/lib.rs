mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn internal_angles(mut sides: Vec<i32>) -> Vec<f64> {
    sides.sort_unstable();
    let [a, b, c] = sides[..] else { return vec![] };
    if a + b <= c {
        return vec![];
    }
    let v1 = f64::acos(f64::from(a.pow(2) + b.pow(2) - c.pow(2)) / f64::from(2 * a * b)) * 180.0
        / std::f64::consts::PI;
    let v2 = f64::acos(f64::from(a.pow(2) + c.pow(2) - b.pow(2)) / f64::from(2 * a * c)) * 180.0
        / std::f64::consts::PI;
    let v3 = f64::acos(f64::from(c.pow(2) + b.pow(2) - a.pow(2)) / f64::from(2 * c * b)) * 180.0
        / std::f64::consts::PI;
    let mut res = vec![v1, v2, v3];
    res.sort_unstable_by(|a, b| a.total_cmp(b));
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
