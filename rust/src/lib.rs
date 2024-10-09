mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn flip_lights(n: i32, m: i32) -> i32 {
    // let n = n.min(6);
    // let shift = 6 - n;
    // let mut seen = std::collections::HashSet::new();
    // for cand in 0..16i32 {
    //     let count = cand.count_ones() as i32;
    //     if count & 1 == m & 1 && count <= m {
    //         let mut config = 0;
    //         if (cand >> 0) & 1 > 0 {
    //             config ^= 0b_111111 >> shift;
    //         }
    //         if (cand >> 1) & 1 > 0 {
    //             config ^= 0b_010101 >> shift;
    //         }
    //         if (cand >> 2) & 1 > 0 {
    //             config ^= 0b_101010 >> shift;
    //         }
    //         if (cand >> 3) & 1 > 0 {
    //             config ^= 0b_100100 >> shift;
    //         }
    //         seen.insert(config);
    //     }
    // }
    // seen.len() as _
    match n {
        1 => {
            if m == 0 {
                1
            } else {
                2
            }
        }
        2 => match m {
            0 => 1,
            1 => 3,
            _ => 4,
        },
        _ => match m {
            0 => 1,
            1 => 4,
            2 => 7,
            _ => 8,
        },
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(flip_lights(1, 1), 2);
        debug_assert_eq!(flip_lights(2, 1), 3);
        debug_assert_eq!(flip_lights(3, 1), 4);
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
}
