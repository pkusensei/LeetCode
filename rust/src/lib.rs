mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn count_cells(grid: &[&[char]], pattern: &str) -> i32 {
    use itertools::Itertools;
    let n = pattern.len();
    if n == 1 {
        return grid
            .iter()
            .flat_map(|row| row.iter().filter(|&&c| c as u8 == pattern.as_bytes()[0]))
            .count() as i32;
    }
    let lps = kmp(pattern.as_bytes());
    let h = grid
        .iter()
        .enumerate()
        .flat_map(|(r, row)| row.iter().enumerate().map(move |(c, &ch)| (ch as u8, r, c)))
        .collect_vec();
    let h_set = merge(&h, &search(&h, pattern.as_bytes(), &lps), n);
    let mut v = vec![];
    let [rows, cols] = get_dimensions(&grid);
    for c in 0..cols {
        for r in 0..rows {
            v.push((grid[r][c] as u8, r, c));
        }
    }
    let v_set = merge(&v, &search(&v, pattern.as_bytes(), &lps), n);
    h_set.intersection(&v_set).count() as i32
}

fn merge(hay: &[(u8, usize, usize)], matched: &[usize], n: usize) -> HashSet<(usize, usize)> {
    let mut start = 0;
    let mut end = 0;
    let mut set = HashSet::new();
    for &i in matched {
        if i > end {
            if start != end {
                set.extend(hay[start..=end].iter().map(|&(_, r, c)| (r, c)));
            }
            start = i;
        }
        end = i + n - 1;
    }
    if start != end {
        set.extend(hay[start..=end].iter().map(|&(_, r, c)| (r, c)));
    }
    set
}

fn search(hay: &[(u8, usize, usize)], needle: &[u8], lps: &[usize]) -> Vec<usize> {
    let n = needle.len();
    let mut len = 0;
    let mut matched = vec![];
    for (idx, &(b, _, _)) in hay.iter().enumerate() {
        while len > 0 && (len == n || b != needle[len]) {
            len = lps[len - 1];
        }
        if b == needle[len] {
            len += 1;
        }
        if len == n {
            matched.push(idx + 1 - len);
        }
    }
    matched
}

fn kmp(s: &[u8]) -> Vec<usize> {
    let n = s.len();
    let mut lps = vec![0; n];
    let mut len = 0;
    for idx in 1..n {
        while len > 0 && s[idx] != s[len] {
            len = lps[len - 1]
        }
        if s[idx] == s[len] {
            len += 1
        }
        lps[idx] = len;
    }
    lps
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
            count_cells(
                &[
                    &['a', 'a', 'c', 'c'],
                    &['b', 'b', 'b', 'c'],
                    &['a', 'a', 'b', 'a'],
                    &['c', 'a', 'a', 'c'],
                    &['a', 'a', 'b', 'a']
                ],
                "abaca"
            ),
            1
        );
    }

    #[test]
    fn test() {}
}
