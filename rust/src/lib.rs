mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub const fn min_moves(sx: i32, sy: i32, mut tx: i32, mut ty: i32) -> i32 {
    let mut res = 0;
    while sx <= tx && sy <= ty {
        if sx == tx && sy == ty {
            return res;
        }
        res += 1;
        if tx >= 2 * ty {
            if tx & 1 == 1 {
                break;
            }
            tx /= 2;
        } else if tx > ty {
            tx -= ty
        } else if ty >= 2 * tx {
            if ty & 1 == 1 {
                break;
            }
            ty /= 2
        } else if ty > tx {
            ty -= tx;
        } else {
            if sx == 0 {
                tx = 0
            } else if sy == 0 {
                ty = 0
            } else {
                break;
            }
        }
    }
    -1
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
        assert_eq!(min_moves(1, 2, 5, 4), 2);
    }

    #[test]
    fn test() {
        assert_eq!(min_moves(1, 0, 4480, 36448), 19);
    }
}
