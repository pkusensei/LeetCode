mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;
use itertools::{chain, izip};
use std::collections::{HashMap, VecDeque};

pub fn max_amount(
    initial_currency: String,
    pairs1: Vec<Vec<String>>,
    rates1: Vec<f64>,
    pairs2: Vec<Vec<String>>,
    rates2: Vec<f64>,
) -> f64 {
    let mut str_id = HashMap::new();
    str_id.insert(initial_currency.as_str(), 0);
    for s in chain!(pairs1.iter(), pairs2.iter()).flatten() {
        let id = str_id.len();
        str_id.entry(s.as_str()).or_insert(id);
    }
    let mat1 = build(&pairs1, &rates1, &str_id);
    let mat2 = build(&pairs2, &rates2, &str_id);
    let mut seen = bfs(mat1, HashMap::from([(0, 1.0)]));
    seen = bfs(mat2, seen);
    seen[&0]
}

fn bfs(mat1: Vec<Vec<f64>>, mut seen: HashMap<usize, f64>) -> HashMap<usize, f64> {
    let mut queue: VecDeque<_> = seen.iter().map(|(k, v)| (*k, *v)).collect();
    while let Some((node, num)) = queue.pop_front() {
        for (next, v) in mat1[node].iter().enumerate() {
            let next_num = v * num;
            if seen.get(&next).is_none_or(|&v| v < next_num) {
                seen.insert(next, next_num);
                queue.push_back((next, next_num));
            }
        }
    }
    seen
}

fn build(pairs: &[Vec<String>], rates: &[f64], str_id: &HashMap<&str, usize>) -> Vec<Vec<f64>> {
    let n = str_id.len();
    let mut mat = vec![vec![0.0; n]; n];
    for i in 0..n {
        mat[i][i] = 1.0;
    }
    for (p, &r) in izip!(pairs.iter(), rates.iter()) {
        let [a, b] = [0, 1].map(|i| str_id[p[i].as_str()]);
        mat[a][b] = r;
        mat[b][a] = 1.0 / r;
    }
    mat
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {{
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        }};
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        }};
    }

    #[test]
    fn basics() {}

    #[test]
    fn test() {}
}
