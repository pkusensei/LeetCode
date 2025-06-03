mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn zigzag_traversal(grid: &[&[i32]]) -> Vec<i32> {
    let [rows, cols] = get_dimensions(&grid);
    let mut res = vec![];
    let mut flag = true;
    let it = Iter {
        rows,
        cols,
        r: 0,
        c: 0,
    };
    for [r, c] in it {
        if flag {
            res.push(grid[r][c]);
        }
        flag = !flag;
    }
    res
}

struct Iter {
    rows: usize,
    cols: usize,
    r: usize,
    c: usize,
}

impl Iterator for Iter {
    type Item = [usize; 2];

    fn next(&mut self) -> Option<Self::Item> {
        let res = if self.r < self.rows && self.c < self.cols {
            Some([self.r, self.c])
        } else {
            None
        };
        if self.r & 1 == 0 {
            self.c += 1;
            if self.c == self.cols {
                self.r += 1;
                self.c = self.cols - 1;
            }
        } else {
            if let Some(v) = self.c.checked_sub(1) {
                self.c = v
            } else {
                self.r += 1;
                self.c = 0;
            }
        }
        res
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
        assert_eq!(zigzag_traversal(&[&[1, 2], &[3, 4]]), [1, 4]);
    }

    #[test]
    fn test() {}
}
