mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn find_duplicate(paths: &[&str]) -> Vec<Vec<String>> {
    let mut map: HashMap<&str, Vec<_>> = HashMap::new();
    for path in paths.iter() {
        for (content, file) in process(path) {
            map.entry(content).or_default().push(file);
        }
    }
    map.into_values().filter(|v| v.len() > 1).collect()
}

fn process(s: &str) -> impl Iterator<Item = (&str, String)> {
    let mut it = s.split_whitespace();
    let dir = it.next().unwrap_or_default();
    it.map(move |f| {
        let i = f.find('(').unwrap();
        let content = &f[i + 1..f.len() - 1]; // ( ... )
        let fullpath = format!("{}/{}", dir, &f[..i]);
        (content, fullpath)
    })
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        sort_eq(
            find_duplicate(&[
                "root/a 1.txt(abcd) 2.txt(efgh)",
                "root/c 3.txt(abcd)",
                "root/c/d 4.txt(efgh)",
                "root 4.txt(efgh)",
            ]),
            [
                vec!["root/a/2.txt", "root/c/d/4.txt", "root/4.txt"],
                vec!["root/a/1.txt", "root/c/3.txt"],
            ],
        );
        sort_eq(
            find_duplicate(&[
                "root/a 1.txt(abcd) 2.txt(efgh)",
                "root/c 3.txt(abcd)",
                "root/c/d 4.txt(efgh)",
            ]),
            [
                vec!["root/a/2.txt", "root/c/d/4.txt"],
                vec!["root/a/1.txt", "root/c/3.txt"],
            ],
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
