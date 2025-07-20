mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn delete_duplicate_folder(paths: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut trie = Trie::default();
    for p in paths.iter() {
        trie.add(p);
    }
    let mut freq = HashMap::new();
    trie.construct(&mut freq);
    let mut res = vec![];
    trie.operate(&freq, &mut vec![], &mut res);
    res
}

#[derive(Default)]
struct Trie {
    serial: String,
    nodes: HashMap<String, Trie>,
}

impl Trie {
    fn add(&mut self, path: &[String]) {
        let mut curr = self;
        for p in path {
            curr = curr.nodes.entry(p.clone()).or_default();
        }
    }

    fn construct(&mut self, freq: &mut HashMap<String, i32>) {
        if self.nodes.is_empty() {
            return;
        }
        let mut v = vec![];
        for (folder, node) in self.nodes.iter_mut() {
            node.construct(freq);
            v.push(format!("{}({})", folder, node.serial));
        }
        v.sort_unstable();
        self.serial = v.join("");
        *freq.entry(self.serial.clone()).or_insert(0) += 1;
    }

    fn operate(
        &self,
        freq: &HashMap<String, i32>,
        path: &mut Vec<String>,
        res: &mut Vec<Vec<String>>,
    ) {
        if freq.get(&self.serial).unwrap_or(&0) > &1 {
            return;
        }
        if !path.is_empty() {
            res.push(path.clone());
        }
        for (folder, node) in self.nodes.iter() {
            path.push(folder.clone());
            node.operate(freq, path, res);
            path.pop();
        }
    }
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

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
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
