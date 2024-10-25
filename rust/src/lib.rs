mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn ambiguous_coordinates(s: &str) -> Vec<String> {
    let s = s.trim_matches(['(', ')']);
    let mut res = vec![];
    for i in 1..s.len() {
        if let (Some(left), Some(right)) = (parse(&s[..i]), parse(&s[i..])) {
            for a in left.iter() {
                for b in right.iter() {
                    res.push(format!("({}, {})", a, b));
                }
            }
        }
    }
    res
}

fn parse(s: &str) -> Option<Vec<String>> {
    if s.is_empty() {
        return None;
    }
    let mut res = vec![s.to_string()];
    if s.len() == 1 {
        return Some(res);
    }
    if s.ends_with('0') {
        if s.starts_with('0') {
            return None;
        }
        Some(res)
    } else {
        let mut v = s.as_bytes().to_vec();
        if s.starts_with('0') {
            res.clear(); // remove vec![s.to_string()]
            v.insert(1, b'.');
            res.push(String::from_utf8(v).unwrap());
        } else {
            res.extend((1..s.len()).map(|i| {
                let mut temp = v.clone();
                temp.insert(i, b'.');
                String::from_utf8(temp).unwrap()
            }));
        }
        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        sort_eq(
            ambiguous_coordinates("(123)"),
            ["(1, 2.3)", "(1, 23)", "(1.2, 3)", "(12, 3)"],
        );
        sort_eq(
            ambiguous_coordinates("(0123)"),
            [
                "(0, 1.23)",
                "(0, 12.3)",
                "(0, 123)",
                "(0.1, 2.3)",
                "(0.1, 23)",
                "(0.12, 3)",
            ],
        );
        sort_eq(
            ambiguous_coordinates("(00011)"),
            ["(0, 0.011)", "(0.001, 1)"],
        );
    }

    #[test]
    fn test() {
        sort_eq(ambiguous_coordinates("(0100)"), ["(0, 100)"]);
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
