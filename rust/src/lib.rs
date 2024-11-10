mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_deletion_size(strs: &[&str]) -> i32 {
    let (rows, cols) = get_dimensions(strs);
    let mut keep: Vec<usize> = vec![];
    for c in 0..cols {
        let mut can_keep = true;
        for r in 1..rows {
            if strs[r - 1].as_bytes()[c] > strs[r].as_bytes()[c] {
                can_keep = false;
                for &prev in keep.iter().rev() {
                    if strs[r - 1].as_bytes()[prev] < strs[r].as_bytes()[prev] {
                        can_keep = true;
                        break;
                    }
                }
                if !can_keep {
                    break;
                }
            }
        }
        if can_keep {
            keep.push(c);
        }
    }
    (cols - keep.len()) as i32
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(min_deletion_size(&["ca", "bb", "ac"]), 1);
        debug_assert_eq!(min_deletion_size(&["xc", "yb", "za"]), 0);
        debug_assert_eq!(min_deletion_size(&["zyx", "wvu", "tsr"]), 3);
    }

    #[test]
    fn test() {}

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

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64>,
        T2: Into<f64>,
    {
        const EP: f64 = 1e-5;
        debug_assert!((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP);
    }
}
