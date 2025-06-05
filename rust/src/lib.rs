mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
    let mut dsu = DSU::new();
    for (b1, b2) in s1.bytes().zip(s2.bytes()) {
        dsu.union(usize::from(b1 - b'a'), usize::from(b2 - b'a'));
    }
    base_str
        .bytes()
        .map(|b| char::from(dsu.find(usize::from(b - b'a')) as u8 + b'a'))
        .collect()
}

struct DSU {
    parent: [usize; 26],
}

impl DSU {
    fn new() -> Self {
        let mut parent = [0; 26];
        for (i, v) in parent.iter_mut().enumerate() {
            *v = i;
        }
        Self { parent }
    }

    fn find(&mut self, v: usize) -> usize {
        if self.parent[v] != v {
            self.parent[v] = self.find(self.parent[v]);
        }
        self.parent[v]
    }

    fn union(&mut self, x: usize, y: usize) {
        let [rx, ry] = [x, y].map(|v| self.find(v));
        let r = rx.min(ry);
        self.parent[rx] = r;
        self.parent[ry] = r;
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
    fn basics() {}

    #[test]
    fn test() {}
}
