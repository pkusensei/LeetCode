mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn hit_bricks(grid: &[&[i32]], hits: &[[i32; 2]]) -> Vec<i32> {
    let (rows, cols) = get_dimensions(grid);
    let mut aftermath: Vec<_> = grid.iter().map(|v| v.to_vec()).collect();
    for h in hits.iter() {
        aftermath[h[0] as usize][h[1] as usize] = 0;
    }
    let mut dsu = Dsu::new(1 + rows * cols);
    for (r, row) in aftermath.iter().enumerate() {
        for (c, &val) in row.iter().enumerate() {
            if val == 1 {
                let i = r * cols + c;
                if r == 0 {
                    dsu.union(i, rows * cols);
                }
                if r.checked_sub(1).is_some_and(|v| aftermath[v][c] == 1) {
                    dsu.union(i, (r - 1) * cols + c);
                }
                if c.checked_sub(1).is_some_and(|v| aftermath[r][v] == 1) {
                    dsu.union(i, r * cols + c - 1);
                }
            }
        }
    }
    let mut res = vec![];
    for &[r, c] in hits.iter().rev() {
        let pre_roof = dsu.top();
        if grid[r as usize][c as usize] == 0 {
            res.push(0);
        } else {
            let i = r as usize * cols + c as usize;
            for (nr, nc) in neighbors((r as usize, c as usize)) {
                if aftermath
                    .get(nr)
                    .is_some_and(|row| row.get(nc).is_some_and(|&v| v == 1))
                {
                    dsu.union(i, nr * cols + nc);
                }
            }
            if r == 0 {
                dsu.union(i, rows * cols);
            }
            aftermath[r as usize][c as usize] = 1;
            res.push(0.max(dsu.top() as i32 - pre_roof as i32 - 1));
        }
    }
    res.reverse();
    res
}

#[derive(Debug, Clone)]
struct Dsu {
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let (mut xr, mut yr) = (self.find(x), self.find(y));
        if xr == yr {
            return;
        }
        if self.rank[xr] < self.rank[yr] {
            std::mem::swap(&mut xr, &mut yr);
        }
        if self.rank[xr] == self.rank[yr] {
            self.rank[xr] += 1;
        }
        self.parent[yr] = xr;
        self.size[xr] += self.size[yr];
    }

    fn size(&mut self, x: usize) -> usize {
        let i = self.find(x);
        self.size[i]
    }

    fn top(&mut self) -> usize {
        self.size(self.size.len() - 1) - 1
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(hit_bricks(&[&[1, 0, 0, 0], &[1, 1, 1, 0]], &[[1, 0]]), [2]);
        debug_assert_eq!(
            hit_bricks(&[&[1, 0, 0, 0], &[1, 1, 0, 0]], &[[1, 1], [1, 0]]),
            [0, 0]
        );
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }
}
