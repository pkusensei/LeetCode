mod helper;
mod trie;

use std::collections::{HashMap, HashSet, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn accounts_merge(accounts: &[&[&str]]) -> Vec<Vec<String>> {
    let mut names: HashMap<&str, &str> = HashMap::new();
    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();
    for a in accounts.iter() {
        let name = a[0];
        for em in a[1..].iter() {
            names.insert(em, name);
            graph.entry(em).or_default().extend(&a[1..]);
        }
    }

    let mut seen = HashSet::new();
    let mut groups = vec![];
    let mut acc = vec![];
    for &k in graph.keys() {
        if !seen.insert(k) {
            continue;
        }
        let mut queue = VecDeque::from([k]);
        while let Some(curr) = queue.pop_front() {
            acc.push(curr);
            for &s in graph[curr].iter() {
                if seen.insert(s) {
                    queue.push_back(s);
                }
            }
        }
        groups.push(acc);
        acc = vec![];
    }

    for gr in groups.iter_mut() {
        gr.sort_unstable();
        gr.insert(0, names[gr[0]]);
    }

    groups
        .into_iter()
        .map(|v| v.into_iter().map(|s| s.to_string()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        sort_eq(
            accounts_merge(&[
                &["John", "johnsmith@mail.com", "john_newyork@mail.com"],
                &["John", "johnsmith@mail.com", "john00@mail.com"],
                &["Mary", "mary@mail.com"],
                &["John", "johnnybravo@mail.com"],
            ]),
            [
                vec![
                    "John",
                    "john00@mail.com",
                    "john_newyork@mail.com",
                    "johnsmith@mail.com",
                ],
                vec!["Mary", "mary@mail.com"],
                vec!["John", "johnnybravo@mail.com"],
            ],
        );
        sort_eq(
            accounts_merge(&[
                &["Gabe", "Gabe0@m.co", "Gabe3@m.co", "Gabe1@m.co"],
                &["Kevin", "Kevin3@m.co", "Kevin5@m.co", "Kevin0@m.co"],
                &["Ethan", "Ethan5@m.co", "Ethan4@m.co", "Ethan0@m.co"],
                &["Hanzo", "Hanzo3@m.co", "Hanzo1@m.co", "Hanzo0@m.co"],
                &["Fern", "Fern5@m.co", "Fern1@m.co", "Fern0@m.co"],
            ]),
            [
                vec!["Ethan", "Ethan0@m.co", "Ethan4@m.co", "Ethan5@m.co"],
                vec!["Gabe", "Gabe0@m.co", "Gabe1@m.co", "Gabe3@m.co"],
                vec!["Hanzo", "Hanzo0@m.co", "Hanzo1@m.co", "Hanzo3@m.co"],
                vec!["Kevin", "Kevin0@m.co", "Kevin3@m.co", "Kevin5@m.co"],
                vec!["Fern", "Fern0@m.co", "Fern1@m.co", "Fern5@m.co"],
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
