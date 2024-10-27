mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn num_similar_groups(strs: &[&str]) -> i32 {
    let mut dsu = dsu::DSU::new(strs.len());
    for (i1, a) in strs.iter().enumerate() {
        for (i2, b) in strs.iter().enumerate().skip(1 + i1) {
            if check(a, b) {
                dsu.union(i1, i2);
            }
        }
    }
    dsu.count() as i32
}

fn check(a: &str, b: &str) -> bool {
    let mut it = a.bytes().zip(b.bytes()).filter(|&(a, b)| a != b);
    match (it.next(), it.next()) {
        (None, None) => true,
        (Some((a1, b1)), Some((a2, b2))) => it.next().is_none() && (a1, a2) == (b2, b1),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(num_similar_groups(&["tars", "rats", "arts", "star"]), 2);
        debug_assert_eq!(num_similar_groups(&["omv", "ovm"]), 1);
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
