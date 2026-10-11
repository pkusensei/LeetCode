mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_inversions(words: &[&str], target: &[i32]) -> i32 {
    let n = target.len();
    let mut target_idx = vec![0; n];
    for (i, &v) in target.iter().enumerate() {
        target_idx[v as usize] = i;
    }
    let mut trie = Trie::new();
    for (i, w) in words.iter().enumerate() {
        trie.insert(w.as_bytes(), target_idx[i]);
    }
    trie.traverse().1
}

const MAX: usize = (b'j' - b'a') as usize;
#[derive(Clone)]
struct Trie {
    nodes: [Option<Box<Trie>>; 1 + MAX],
    pos: usize, // idx in target
}

impl Trie {
    fn new() -> Self {
        Self {
            nodes: Default::default(),
            pos: 10_001,
        }
    }

    fn insert(&mut self, s: &[u8], pos: usize) {
        let mut curr = self;
        for b in s {
            let i = usize::from(b - b'a');
            let v = curr.nodes[i].get_or_insert(Box::new(Trie::new()));
            curr = v;
        }
        curr.pos = pos;
    }

    fn traverse(&self) -> (Vec<usize>, i32) {
        let mut childs = vec![];
        let mut all_groups = vec![];
        let mut res = 0;
        for v in self.nodes.iter() {
            let Some(node) = v.as_ref() else {
                continue;
            };
            let (a, b) = node.traverse();
            childs.extend(a.clone());
            all_groups.push(a);
            res += b;
        }
        childs.sort_unstable();
        let mut min = None;
        for gr in all_groups {
            let mut curr = 0;
            for (i_gr, pos) in gr.into_iter().enumerate() {
                let i = childs.partition_point(|&v| v < pos);
                curr += (i - i_gr) as i32;
            }
            let v = min.get_or_insert(curr);
            *v = (*v).min(curr)
        }
        if self.pos < 10_001 {
            let i = childs.partition_point(|&v| v < self.pos);
            res += i as i32;
            childs.push(self.pos);
        }
        childs.sort_unstable();
        res += min.unwrap_or(0);
        (childs, res)
    }
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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
    fn basics() {
        assert_eq!(
            min_inversions(&["bad", "bag", "fig", "fed"], &[3, 1, 0, 2]),
            2
        );
    }

    #[test]
    fn test() {
        assert_eq!(min_inversions(&["jf", "b", "gd", "gg"], &[1, 3, 0, 2]), 1);
    }
}
