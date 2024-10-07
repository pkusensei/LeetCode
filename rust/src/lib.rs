mod helper;
mod trie;

use std::collections::VecDeque;

#[allow(unused_imports)]
use helper::*;

pub fn predict_party_victory(s: &str) -> String {
    let n = s.len();
    let (mut ds, mut rs) = (VecDeque::new(), VecDeque::new());
    for (i, b) in s.bytes().enumerate() {
        if b == b'D' {
            ds.push_back(i);
        } else {
            rs.push_back(i);
        }
    }
    while !ds.is_empty() || !rs.is_empty() {
        let Some(d) = ds.pop_front() else {
            return "Radiant".into();
        };
        let Some(r) = rs.pop_front() else {
            return "Dire".into();
        };
        if d < r {
            ds.push_back(d + n);
        } else {
            rs.push_back(r + n);
        }
    }
    if ds.is_empty() {
        "Radiant".into()
    } else {
        "Dire".into()
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(predict_party_victory("RD".into()), "Radiant");
        debug_assert_eq!(predict_party_victory("RDD".into()), "Dire");
        debug_assert_eq!(predict_party_victory("DDRRR".into()), "Dire");
    }

    #[test]
    fn test() {
        debug_assert_eq!(predict_party_victory("DRRDRDRDRDDRDRDR".into()), "Radiant");
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
