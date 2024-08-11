use std::collections::{HashMap, HashSet, VecDeque};

pub fn find_ladders(begin_word: &str, end_word: &str, word_list: &[&str]) -> Vec<Vec<String>> {
    if !word_list.contains(&end_word) {
        return vec![];
    }
    let graph = build_graph(begin_word, word_list);
    let parents = bfs(begin_word, &graph);
    build_path(&parents, vec![end_word], end_word)
        .into_iter()
        .map(|v| v.into_iter().map(|s| s.to_string()).collect())
        .collect()
}

pub fn ladder_length(begin_word: &str, end_word: &str, word_list: &[&str]) -> i32 {
    if !word_list.contains(&end_word) {
        return 0;
    }
    let graph = build_graph(begin_word, word_list);
    let dist = bfs_length(begin_word, &graph);
    dist.get(end_word)
        .copied()
        .flatten()
        .map(|n| n + 1)
        .unwrap_or_default()
}

fn bfs<'a>(
    begin_word: &'a str,
    graph: &HashMap<&str, HashSet<&'a str>>,
) -> HashMap<&'a str, Vec<&'a str>> {
    let mut parents = HashMap::from([(begin_word, vec![])]);
    let mut dist = HashMap::from([(begin_word, 0)]);
    for &n in graph.keys().filter(|&&k| k != begin_word) {
        dist.insert(n, i32::MIN);
    }
    let mut queue = VecDeque::from([begin_word]);

    while let Some(curr) = queue.pop_front() {
        if !graph.contains_key(curr) {
            continue;
        }
        for &neighbor in graph[curr].iter() {
            if dist[neighbor] == i32::MIN {
                dist.insert(neighbor, dist[curr] + 1);
                parents.entry(neighbor).or_default().push(curr);
                queue.push_back(neighbor);
            } else if dist[curr] + 1 == dist[neighbor] {
                parents.entry(neighbor).or_default().push(curr);
            } else if dist[curr] + 1 < dist[neighbor] {
                dist.insert(neighbor, dist[curr] + 1);
                parents.insert(neighbor, vec![curr]);
            }
        }
    }
    parents
}

fn bfs_length<'a>(
    begin_word: &'a str,
    graph: &HashMap<&'a str, HashSet<&'a str>>,
) -> HashMap<&'a str, Option<i32>> {
    let mut dist = HashMap::from([(begin_word, Some(0))]);
    for &n in graph.keys().filter(|&&k| k != begin_word) {
        dist.insert(n, None);
    }
    let mut queue = VecDeque::from([begin_word]);

    while let Some(curr) = queue.pop_front() {
        if !graph.contains_key(curr) {
            continue;
        }
        for &neighbor in graph[curr].iter() {
            if dist[neighbor].is_none() {
                dist.insert(neighbor, dist[curr].map(|n| n + 1));
                queue.push_back(neighbor);
            } else if dist[curr].map(|n| n + 1) < dist[neighbor] {
                dist.insert(neighbor, dist[curr].map(|n| n + 1));
            }
        }
    }
    dist
}

fn build_path<'a>(
    parents: &HashMap<&str, Vec<&'a str>>,
    mut path: Vec<&'a str>,
    curr: &'a str,
) -> Vec<Vec<&'a str>> {
    let mut res = vec![];
    if parents.get(curr).is_none() || parents[curr].is_empty() {
        if path.len() > 1 {
            path.reverse();
            res.push(path);
        }
    } else {
        for &p in parents[curr].iter() {
            let mut c = path.clone();
            c.push(p);
            res.extend(build_path(parents, c, p))
        }
    }
    res
}

fn build_graph<'a>(begin_word: &'a str, list: &[&'a str]) -> HashMap<&'a str, HashSet<&'a str>> {
    let mut graph: HashMap<&str, HashSet<_>> = HashMap::new();
    for (i, &a) in std::iter::once(&begin_word).chain(list.iter()).enumerate() {
        for &b in list.iter().skip(i) {
            if is_adjacent(a, b) {
                graph.entry(a).or_default().insert(b);
                graph.entry(b).or_default().insert(a);
            }
        }
    }
    graph
}

fn is_adjacent(a: &str, b: &str) -> bool {
    a.bytes()
        .zip(b.bytes())
        .filter(|&(b1, b2)| b1 != b2)
        .count()
        == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            find_ladders("hit", "cog", &["hot", "dot", "dog", "lot", "log", "cog"]),
            [
                ["hit", "hot", "dot", "dog", "cog"],
                ["hit", "hot", "lot", "log", "cog"],
            ]
        );
        debug_assert_eq!(
            find_ladders("hit", "cog", &["hot", "dot", "dog", "lot", "log"]),
            Vec::<Vec<String>>::new()
        );

        debug_assert_eq!(
            ladder_length("hit", "cog", &["hot", "dot", "dog", "lot", "log", "cog"]),
            5
        );
        debug_assert_eq!(
            ladder_length("hit", "cog", &["hot", "dot", "dog", "lot", "log"]),
            0
        );
    }

    #[test]
    fn test() {
        debug_assert_eq!(find_ladders("a", "c", &["a", "b", "c"]), [["a", "c"]]);
        debug_assert_eq!(
            find_ladders("hot", "dog", &["hot", "dog"]),
            Vec::<Vec<String>>::new()
        );

        debug_assert_eq!(
            ladder_length("hit", "cog", &["hot", "dot", "tog", "cog"]),
            0
        )
    }
}
