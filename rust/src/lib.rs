mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn color_grid(n: i32, m: i32, mut sources: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    use std::cmp::Reverse;
    use std::collections::VecDeque;

    let [rows, cols] = [n, m].map(|v| v as usize);
    sources.sort_unstable_by_key(|v| Reverse(v[2]));
    let mut queue = VecDeque::new();
    let mut res = vec![vec![0; cols]; rows];
    for s in sources.iter() {
        let [r, c, val] = s[..] else { unreachable!() };
        let [r, c] = [r, c].map(|v| v as usize);
        res[r][c] = val;
        queue.push_back((r, c, val));
    }
    while !queue.is_empty() {
        let len = queue.len();
        for _ in 0..len {
            let (r, c, val) = queue.pop_front().unwrap();
            for [nr, nc] in neighbors([r, c]) {
                if res
                    .get(nr)
                    .is_some_and(|row| row.get(nc).is_some_and(|&v| v == 0))
                {
                    res[nr][nc] = res[nr][nc].max(val);
                    queue.push_back((nr, nc, val));
                }
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
