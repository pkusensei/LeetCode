mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn invalid_transactions(transactions: &[&str]) -> Vec<String> {
    let mut res = std::collections::HashSet::new();
    let trs: Vec<_> = transactions.iter().filter_map(|s| parse(s)).collect();
    for (i1, t1) in trs.iter().enumerate() {
        if t1.2 > 1000 {
            res.insert(i1);
        }
        for (i2, t2) in trs.iter().enumerate().skip(1 + i1) {
            if t1.0 == t2.0 && t1.3 != t2.3 && t1.1.abs_diff(t2.1) <= 60 {
                res.extend([i1, i2]);
            }
        }
    }
    res.into_iter()
        .map(|i| format!("{},{},{},{}", trs[i].0, trs[i].1, trs[i].2, trs[i].3))
        .collect()
}

fn parse(line: &str) -> Option<(&str, u16, u16, &str)> {
    let mut it = line.split(',');
    let name = it.next()?;
    let time = it.next().and_then(|s| s.parse().ok())?;
    let amount = it.next().and_then(|s| s.parse().ok())?;
    let city = it.next()?;
    Some((name, time, amount, city))
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        sort_eq(
            invalid_transactions(&["alice,20,800,mtv", "alice,50,100,beijing"]),
            ["alice,20,800,mtv", "alice,50,100,beijing"],
        );
        sort_eq(
            invalid_transactions(&["alice,20,800,mtv", "alice,50,1200,mtv"]),
            ["alice,50,1200,mtv"],
        );
        sort_eq(
            invalid_transactions(&["alice,20,800,mtv", "bob,50,1200,mtv"]),
            ["bob,50,1200,mtv"],
        );
    }

    #[test]
    fn test() {
        sort_eq(
            invalid_transactions(&[
                "alice,20,800,mtv",
                "alice,50,100,mtv",
                "alice,51,100,frankfurt",
            ]),
            [
                "alice,20,800,mtv",
                "alice,50,100,mtv",
                "alice,51,100,frankfurt",
            ],
        );
        sort_eq(
            invalid_transactions(&["alice,20,1220,mtv", "alice,20,1220,mtv"]),
            ["alice,20,1220,mtv", "alice,20,1220,mtv"],
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
        if (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() > EP {
            dbg!(a, b);
        }
    }
}
