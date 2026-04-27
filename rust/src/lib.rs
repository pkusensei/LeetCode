mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn has_valid_path(grid: Vec<Vec<i32>>) -> bool {
    let [rows, cols] = get_dimensions(&grid);
    let mut dsu = DSU::new(rows * cols);
    for (r, row) in grid.iter().enumerate() {
        for (c, &val) in row.iter().enumerate() {
            let dir = DIR[val as usize - 1];
            for (i, &[dr, dc]) in dir.iter().enumerate() {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if (0..rows as i32).contains(&nr)
                    && (0..cols as i32).contains(&nc)
                    && NEI[val as usize - 1][i].contains(&grid[nr as usize][nc as usize])
                {
                    dsu.union(r * cols + c, nr as usize * cols + nc as usize);
                }
            }
        }
    }
    dsu.find(0) == dsu.find(rows * cols - 1)
}

const DIR: [[[i32; 2]; 2]; 6] = [
    [[0, -1], [0, 1]],
    [[1, 0], [-1, 0]],
    [[0, -1], [1, 0]],
    [[0, 1], [1, 0]],
    [[0, -1], [-1, 0]],
    [[0, 1], [-1, 0]],
];

const NEI: [[[i32; 3]; 2]; 6] = [
    [[1, 4, 6], [1, 3, 5]],
    [[2, 5, 6], [2, 3, 4]],
    [[1, 4, 6], [2, 5, 6]],
    [[1, 3, 5], [2, 5, 6]],
    [[1, 4, 6], [2, 3, 4]],
    [[1, 3, 5], [2, 3, 4]],
];

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
    fn basics() {}

    #[test]
    fn test() {}
}
