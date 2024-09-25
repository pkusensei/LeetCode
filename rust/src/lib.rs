mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn construct_rectangle(area: i32) -> [i32; 2] {
    let upper = (area as f64).sqrt().trunc() as i32;
    for w in (1..=upper).rev() {
        if area % w == 0 {
            return [area / w, w];
        }
    }
    [area, 1]
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(construct_rectangle(4), [2, 2]);
        debug_assert_eq!(construct_rectangle(37), [37, 1]);
        debug_assert_eq!(construct_rectangle(122122), [427, 286]);
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
