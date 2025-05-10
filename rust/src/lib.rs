mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::collections::VecDeque;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_cost(target: &str, words: &[&str], costs: &[i32]) -> i32 {
    let s = target.as_bytes();
    let n = s.len();
    let mut ac = AhoCorasick::new();
    for (word, &cost) in words.iter().zip(costs.iter()) {
        ac.insert(word.as_bytes(), cost);
    }
    ac.build();
    let matches = ac.find_all(s);
    let mut dp = vec![i32::MAX; n + 1];
    dp[0] = 0;
    for i in 1..=n {
        for &(len, cost) in &matches[i] {
            if dp[i - len] != i32::MAX {
                dp[i] = dp[i].min(dp[i - len] + cost);
            }
        }
    }
    if dp[n] == i32::MAX { -1 } else { dp[n] }
}

#[derive(Default)]
struct Node {
    children: [Option<usize>; 26],
    fail: usize,
    outputs: Vec<(usize, i32)>, // (length of word, cost)
}

struct AhoCorasick {
    nodes: Vec<Node>,
}

impl AhoCorasick {
    fn new() -> Self {
        Self {
            nodes: vec![Node::default()],
        }
    }

    fn insert(&mut self, word: &[u8], cost: i32) {
        let mut curr = 0;
        for &b in word {
            let idx = (b - b'a') as usize;
            if let Some(v) = self.nodes[curr].children[idx] {
                curr = v;
            } else {
                let new_idx = self.nodes.len();
                self.nodes.push(Node::default());
                self.nodes[curr].children[idx] = Some(new_idx);
                curr = new_idx;
            }
        }
        self.nodes[curr].outputs.push((word.len(), cost));
    }

    fn build(&mut self) {
        let mut queue = VecDeque::new();
        for child in self.nodes[0].children.into_iter().flatten() {
            self.nodes[child].fail = 0;
            queue.push_back(child);
        }
        while let Some(curr) = queue.pop_front() {
            for i in 0..26 {
                if let Some(child_idx) = self.nodes[curr].children[i] {
                    let mut fail = self.nodes[curr].fail;
                    while fail != 0 && self.nodes[fail].children[i].is_none() {
                        fail = self.nodes[fail].fail;
                    }
                    if let Some(fail_to) = self.nodes[fail].children[i] {
                        self.nodes[child_idx].fail = fail_to;
                        let outputs = self.nodes[fail_to].outputs.clone();
                        self.nodes[child_idx].outputs.extend(outputs);
                    }
                    queue.push_back(child_idx);
                }
            }
        }
    }

    fn find_all(&self, s: &[u8]) -> Vec<Vec<(usize, i32)>> {
        let mut state = 0;
        let mut result = vec![vec![]; s.len() + 1];
        for (i, &b) in s.iter().enumerate() {
            let idx = (b - b'a') as usize;
            while state != 0 && self.nodes[state].children[idx].is_none() {
                state = self.nodes[state].fail;
            }
            if let Some(next_state) = self.nodes[state].children[idx] {
                state = next_state;
            }
            for &(len, cost) in &self.nodes[state].outputs {
                result[i + 1].push((len, cost));
            }
        }
        result
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
    fn basics() {
        assert_eq!(
            minimum_cost(
                "abcdef",
                &["abdef", "abc", "d", "def", "ef"],
                &[100, 1, 1, 10, 5]
            ),
            7
        );
        assert_eq!(minimum_cost("aaaa", &["z", "zz", "zzz"], &[1, 10, 100]), -1);
    }

    #[test]
    fn test() {}
}
