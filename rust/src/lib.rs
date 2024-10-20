mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn reaching_points(sx: i32, sy: i32, mut tx: i32, mut ty: i32) -> bool {
    // if sx == tx && sy == ty {
    //     return true;
    // }
    // if tx < sx || ty < sy {
    //     return false;
    // }
    // if sx == tx {
    //     return (ty - sy) % sx == 0;
    // }
    // if sy == ty {
    //     return (tx - sx) % sy == 0;
    // }
    // match tx.cmp(&ty) {
    //     std::cmp::Ordering::Less => reaching_points(sx, sy, tx, ty % tx),
    //     std::cmp::Ordering::Greater => reaching_points(sx, sy, tx % ty, ty),
    //     std::cmp::Ordering::Equal => false,
    // }
    while tx >= sx && ty >= sy {
        match tx.cmp(&ty) {
            std::cmp::Ordering::Equal => {
                break;
            }
            std::cmp::Ordering::Less => {
                if tx == sx {
                    return (ty - sy) % sx == 0;
                } else {
                    ty %= tx
                }
            }
            std::cmp::Ordering::Greater => {
                if ty == sy {
                    return (tx - sx) % sy == 0;
                } else {
                    tx %= ty
                }
            }
        }
    }
    tx == sx && ty == sy
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(reaching_points(1, 1, 3, 5));
        debug_assert!(!reaching_points(1, 1, 2, 2));
        debug_assert!(reaching_points(1, 1, 1, 1));
    }

    #[test]
    fn test() {
        // debug_assert!(reaching_points(1, 1, 1_000_000_000, 1));
        debug_assert!(!reaching_points(1, 8, 4, 15));
    }

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }
}
