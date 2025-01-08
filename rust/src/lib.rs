mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn rank_teams(votes: &[&str]) -> String {
    let n = votes[0].len();
    let mut map = std::collections::HashMap::new();
    for row in votes.iter() {
        for (i, b) in row.bytes().enumerate() {
            map.entry(b).or_insert(vec![0; n])[i] += 1;
        }
    }
    let mut res: Vec<_> = map.keys().copied().collect();
    res.sort_unstable_by(|a, b| map[b].cmp(&map[a]).then(a.cmp(b)));
    String::from_utf8(res).unwrap()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(rank_teams(&["ABC", "ACB", "ABC", "ACB", "ACB"]), "ACB");
        assert_eq!(rank_teams(&["WXYZ", "XYZW"]), "XWYZ");
        assert_eq!(
            rank_teams(&["ZMNAGUEDSJYLBOPHRQICWFXTVK"]),
            "ZMNAGUEDSJYLBOPHRQICWFXTVK"
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            rank_teams(&[
                "ABCDEFGH", "BACDEFGH", "GHABCDEF", "GHBACDEF", "EFGHABCD", "EFGHBACD", "CDEFGHBA",
                "CDEFGHBA"
            ]),
            "CEGBADFH"
        );
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
