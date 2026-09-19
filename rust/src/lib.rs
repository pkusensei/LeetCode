mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn check_overlap(
    radius: i32,
    x_center: i32,
    y_center: i32,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
) -> bool {
    // let tx = find(x_center, x1, x2);
    // let ty = find(y_center, y1, y2);
    let tx = x_center.clamp(x1, x2);
    let ty = y_center.clamp(y1, y2);
    let d = (tx - x_center).pow(2) + (ty - y_center).pow(2);
    d <= radius.pow(2)
}

const fn find(center: i32, a: i32, b: i32) -> i32 {
    if center < a {
        a
    } else if a <= center && center <= b {
        center
    } else {
        b
    }
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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
