mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort_unstable_by_key(|&n| std::cmp::Reverse(n));
        s.sort_unstable_by_key(|&n| std::cmp::Reverse(n));
        let (mut i1, mut i2) = (0, 0);
        while i1 < g.len() && i2 < s.len() {
            if g[i1] <= s[i2] {
                i1 += 1;
                i2 += 1
            } else {
                i1 += 1
            }
        }
        i2 as i32
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(find_content_children(vec![1, 2, 3], vec![1, 1]), 1);
        debug_assert_eq!(find_content_children(vec![1, 2], vec![1, 2, 3]), 2);
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
