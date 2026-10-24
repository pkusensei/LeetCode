mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_magical_pairs(s: String) -> i64 {
    let (s, n) = (s.as_bytes(), s.len());
    let mut prefix = vec![[0; 27]; n];
    let mut suffix = vec![[0; 27]; n];
    let mut pos = [None; 26];
    for (i, &b) in s.iter().enumerate() {
        let p = add(&mut pos, i, b);
        let pn = p.len();
        for k in 0..pn {
            prefix[i][1 + k] = p[pn - 1 - k] - if 1 + k < pn { p[pn - 2 - k] } else { -1 };
        }
    }
    pos.fill(None);
    for (i, &b) in s.iter().enumerate().rev() {
        let p = add(&mut pos, i, b);
        let pn = p.len();
        for k in 0..pn {
            suffix[i][1 + k] = if 1 + k < pn { p[1 + k] } else { n as i64 } - p[k];
        }
    }
    let mut res = 0;
    for i in 0..n - 1 {
        for k in 1..=26 {
            res += prefix[i][k] * suffix[1 + i][k];
        }
    }
    res
}

fn add(pos: &mut [Option<usize>; 26], i: usize, b: u8) -> Vec<i64> {
    use itertools::Itertools;
    pos[usize::from(b - b'a')] = Some(i);
    let res = pos
        .iter()
        .filter_map(|x| x.map(|v| v as i64))
        .sorted_unstable()
        .collect_vec();
    res
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
    fn basics() {}

    #[test]
    fn test() {}
}
