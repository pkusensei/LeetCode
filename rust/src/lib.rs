mod helper;

use std::{cmp::Reverse, collections::HashMap};

#[allow(unused_imports)]
use helper::*;

pub fn find_itinerary<'a>(tickets: &[[&'a str; 2]]) -> Vec<&'a str> {
    let mut graph: HashMap<&str, Vec<&str>> =
        tickets.iter().fold(HashMap::new(), |mut acc, &[from, to]| {
            acc.entry(from).or_default().push(to);
            acc
        });
    for v in graph.values_mut() {
        v.sort_unstable_by_key(|&s| Reverse(s));
    }
    let mut res = vec![];
    dfs(&mut graph, "JFK", &mut res);
    res.reverse();
    res
}

fn dfs<'a>(graph: &mut HashMap<&'a str, Vec<&'a str>>, from: &'a str, path: &mut Vec<&'a str>) {
    while let Some(to) = graph.get_mut(&from).and_then(|v| v.pop()) {
        dfs(graph, to, path);
    }
    path.push(from);
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            find_itinerary(&[
                ["MUC", "LHR"],
                ["JFK", "MUC"],
                ["SFO", "SJC"],
                ["LHR", "SFO"]
            ]),
            ["JFK", "MUC", "LHR", "SFO", "SJC"]
        );
        debug_assert_eq!(
            find_itinerary(&[
                ["JFK", "SFO"],
                ["JFK", "ATL"],
                ["SFO", "ATL"],
                ["ATL", "JFK"],
                ["ATL", "SFO"]
            ]),
            ["JFK", "ATL", "JFK", "SFO", "ATL", "SFO"]
        );
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(i1: I1, i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: IntoIterator<Item = T1>,
        I2: IntoIterator<Item = T2>,
    {
        let mut v1: Vec<_> = i1.into_iter().collect();
        let mut v2: Vec<_> = i2.into_iter().collect();
        v1.sort_unstable();
        v2.sort_unstable();
        debug_assert_eq!(v1, v2);
    }
}
