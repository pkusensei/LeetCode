mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn grid_illumination(_n: i32, lamps: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    use std::collections::{HashMap, HashSet};
    let mut set = HashSet::new();
    let [mut rows, mut cols, mut diag1, mut diag2] = [0; 4].map(|_| HashMap::new());
    for v in lamps {
        let [r, c] = v[..] else { unreachable!() };
        if set.insert([r, c]) {
            *rows.entry(r).or_insert(0) += 1;
            *cols.entry(c).or_insert(0) += 1;
            // diag /
            *diag1.entry(r + c).or_insert(0) += 1;
            // diag \
            *diag2.entry(r - c).or_insert(0) += 1;
        }
    }
    let mut res = vec![];
    for q in queries {
        let [r, c] = q[..] else { unreachable!() };
        let is_on = rows.get(&r).is_some_and(|&v| v > 0)
            || cols.get(&c).is_some_and(|&v| v > 0)
            || diag1.get(&(r + c)).is_some_and(|&v| v > 0)
            || diag2.get(&(r - c)).is_some_and(|&v| v > 0);
        res.push(i32::from(is_on));
        for [dr, dc] in ALL_DIRS.into_iter().chain([[0, 0]]) {
            let nr = r + dr;
            let nc = c + dc;
            if set.remove(&[nr, nc]) {
                *rows.entry(nr).or_insert(0) -= 1;
                *cols.entry(nc).or_insert(0) -= 1;
                // diag /
                *diag1.entry(nr + nc).or_insert(0) -= 1;
                // diag \
                *diag2.entry(nr - nc).or_insert(0) -= 1;
            }
        }
    }
    res
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
