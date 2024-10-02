mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn array_rank_transform(mut arr: Vec<i32>) -> Vec<i32> {
    let map = std::collections::BTreeSet::from_iter(arr.clone())
        .into_iter()
        .enumerate()
        .map(|(i, n)| (n, 1 + i as i32))
        .collect::<std::collections::HashMap<_, _>>();
    for v in arr.iter_mut() {
        *v = map[&*v];
    }
    arr
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(array_rank_transform([40, 10, 20, 30].into()), [4, 1, 2, 3]);
        debug_assert_eq!(array_rank_transform([100, 100, 100].into()), [1, 1, 1]);
        debug_assert_eq!(
            array_rank_transform([37, 12, 28, 9, 100, 56, 80, 5, 12].into()),
            [5, 3, 4, 2, 8, 6, 7, 1, 3]
        );
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
