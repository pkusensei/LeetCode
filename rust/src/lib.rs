mod helper;
mod trie;

use std::collections::BTreeMap;

#[allow(unused_imports)]
use helper::*;

pub fn count_of_atoms(formula: &str) -> String {
    let map = count(formula.as_bytes());
    map.into_iter()
        .filter_map(|(k, v)| {
            if !k.is_empty() {
                if v > 1 {
                    Some(format!("{k}{v}"))
                } else {
                    Some(k)
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join("")
}

fn count(s: &[u8]) -> BTreeMap<String, i32> {
    let mut name = vec![];
    let mut count = 0;
    let mut opens = vec![];
    let mut res = BTreeMap::new();
    let mut idx = 0;
    while idx < s.len() {
        let b = s[idx];
        match b {
            b'0'..=b'9' => count = 10 * count + i32::from(b - b'0'),
            b'a'..=b'z' => name.push(b),
            b'A'..=b'Z' => {
                insert(name, &mut count, &mut res);
                name = vec![b];
            }
            b'(' => {
                insert(name, &mut count, &mut res);
                name = vec![];

                opens.push(res);
                res = BTreeMap::new();
            }
            b')' => {
                insert(name, &mut count, &mut res);
                name = vec![];

                let mut c = 0;
                idx += 1;
                while idx < s.len() && s[idx].is_ascii_digit() {
                    c = 10 * c + i32::from(s[idx] - b'0');
                    idx += 1;
                }
                let Some(last) = opens.pop() else {
                    continue;
                };
                let c = if c == 0 { 1 } else { c };
                for v in res.values_mut() {
                    *v *= c;
                }
                for (k, v) in last.into_iter() {
                    *res.entry(k).or_insert(0) += v;
                }
                continue; // skip +=1 before next loop
            }
            _ => unreachable!(),
        }
        idx += 1;
    }
    if !name.is_empty() {
        insert(name, &mut count, &mut res);
    }
    res
}

fn insert(name: Vec<u8>, count: &mut i32, res: &mut BTreeMap<String, i32>) {
    let n = String::from_utf8(name).unwrap();
    let c = if *count == 0 { 1 } else { *count };
    *res.entry(n).or_insert(0) += c;
    *count = 0;
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(count_of_atoms("H2O"), "H2O");
        debug_assert_eq!(count_of_atoms("Mg(OH)2"), "H2MgO2");
        debug_assert_eq!(count_of_atoms("K4(ON(SO3)2)2"), "K4N2O14S4");
    }

    #[test]
    fn test() {
        debug_assert_eq!(count_of_atoms("Mg(H2O)N"), "H2MgNO");
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
