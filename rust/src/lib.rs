mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn kth_grammar(n: i32, k: i32) -> i32 {
    if k == 1 {
        return 0;
    }
    let len = 2i32.pow(n as u32 - 1);
    if k > len / 2 {
        1 - kth_grammar(n, k - len / 2) // flip
    } else {
        kth_grammar(n - 1, k)
    }
    // bit magic
    // (k - 1).count_ones() as i32 & 1
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(kth_grammar(1, 1), 0);
        debug_assert_eq!(kth_grammar(2, 1), 0);
        debug_assert_eq!(kth_grammar(2, 2), 1);
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
