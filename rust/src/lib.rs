mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn group_strings(words: &[&str]) -> Vec<i32> {
    let n = words.len();
    let mut dsu = DSU::new(n);
    let mut mask_idx = std::collections::HashMap::new();
    for (idx, s) in words.iter().enumerate() {
        let mask = to_mask(s);
        if let Some(&v) = mask_idx.get(&mask) {
            dsu.union(v, idx);
        } else {
            mask_idx.insert(mask, idx);
        }
        for bit in 0..26 {
            if (mask >> bit) & 1 == 1 {
                let del = mask ^ (1 << bit);
                if let Some(&v) = mask_idx.get(&del) {
                    dsu.union(v, idx);
                } else {
                    mask_idx.insert(del, idx);
                }
            }
        }
    }
    vec![dsu.count as i32, *dsu.size.iter().max().unwrap_or(&1)]
}

fn to_mask(s: &str) -> i32 {
    s.bytes().fold(0, |acc, b| acc | (1 << (b - b'a')))
}

struct DSU {
    parent: Vec<usize>,
    size: Vec<i32>,
    count: usize,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            count: n,
        }
    }

    fn find(&mut self, v: usize) -> usize {
        if self.parent[v] != v {
            self.parent[v] = self.find(self.parent[v]);
        }
        self.parent[v]
    }

    fn union(&mut self, x: usize, y: usize) {
        let [rx, ry] = [x, y].map(|v| self.find(v));
        if rx == ry {
            return;
        }
        match self.size[rx].cmp(&self.size[ry]) {
            std::cmp::Ordering::Less => {
                self.parent[rx] = ry;
                self.size[ry] += self.size[rx];
            }
            _ => {
                self.parent[ry] = rx;
                self.size[rx] += self.size[ry];
            }
        }
        self.count -= 1;
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
        assert_eq!(group_strings(&["a", "b", "ab", "cde"]), [2, 3]);
        assert_eq!(group_strings(&["a", "ab", "abc"]), [1, 3]);
    }

    #[test]
    fn test() {
        assert_eq!(group_strings(&["qamp", "am", "khdrn"]), [3, 1]);
    }
}
