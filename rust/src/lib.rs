mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn get_folder_names(names: Vec<String>) -> Vec<String> {
    let mut map = std::collections::HashMap::new();
    let mut res = Vec::with_capacity(names.len());
    for name in names {
        if let Some(v) = map.get(&name) {
            let mut v = *v;
            let mut curr = format!("{name}({v})");
            while map.contains_key(&curr) {
                v += 1;
                curr = format!("{name}({v})")
            }
            res.push(curr.clone());
            map.insert(name, 1 + v); // update this count to skip while loop
            map.insert(curr, 1); // register this name
        } else {
            res.push(name.clone());
            map.insert(name, 1);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {}

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
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
