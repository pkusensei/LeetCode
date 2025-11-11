mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub const fn reaching_points(sx: i32, sy: i32, mut tx: i32, mut ty: i32) -> bool {
    if tx < sx || ty < sy {
        return false;
    }
    while tx > sx && ty > sy {
        if tx > ty {
            tx %= ty;
        } else if tx < ty {
            ty %= tx;
        } else {
            break;
        }
    }
    if sx == tx {
        return (ty - sy) % sx == 0;
    }
    if sy == ty {
        return (tx - sx) % sy == 0;
    }
    tx == sx && ty == sy
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
    fn basics() {
        assert!(reaching_points(1, 1, 3, 5));
    }

    #[test]
    fn test() {}
}
