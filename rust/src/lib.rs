mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn latest_day_to_cross(row: i32, col: i32, cells: &[[i32; 2]]) -> i32 {
    let [row, col] = [row, col].map(|v| v as usize);
    let n = row * col;
    let mut dsu = DSU::new(2 + n);
    let id_of = |r, c| r * col + c;
    for c in 0..col {
        dsu.union(n, c); // top tow
        dsu.union(1 + n, id_of(row - 1, c)); // bottom row
    }
    let mut land = vec![false; n];
    for (idx, cell) in cells.iter().enumerate().rev() {
        let [r, c] = {
            let [r, c] = cell[..] else { unreachable!() };
            [(r - 1) as usize, c as usize - 1]
        };
        let id = id_of(r, c);
        land[id] = true;
        for [nr, nc] in neighbors([r, c]) {
            let nid = id_of(nr, nc);
            if nr < row && nc < col && land[nid] {
                dsu.union(id, nid);
            }
        }
        if dsu.find(n) == dsu.find(1 + n) {
            return idx as i32;
        }
    }
    -1
}

struct DSU {
    parent: Vec<usize>,
    rank: Vec<i32>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, v: usize) -> usize {
        if self.parent[v] != v {
            self.parent[v] = self.find(self.parent[v])
        }
        self.parent[v]
    }

    fn union(&mut self, x: usize, y: usize) {
        let [rx, ry] = [x, y].map(|v| self.find(v));
        if rx == ry {
            return;
        }
        match self.rank[rx].cmp(&self.rank[ry]) {
            std::cmp::Ordering::Less => self.parent[rx] = ry,
            std::cmp::Ordering::Equal => {
                self.parent[ry] = rx;
                self.rank[rx] += 1;
            }
            std::cmp::Ordering::Greater => self.parent[ry] = rx,
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
    fn basics() {
        assert_eq!(
            latest_day_to_cross(
                3,
                3,
                &[
                    [1, 2],
                    [2, 1],
                    [3, 3],
                    [2, 2],
                    [1, 1],
                    [1, 3],
                    [2, 3],
                    [3, 2],
                    [3, 1]
                ]
            ),
            3
        );
    }

    #[test]
    fn test() {}
}
