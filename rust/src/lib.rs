mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_palindrome_paths(parent: &[i32], s: &str) -> i64 {
    let n = parent.len();
    let adj = parent
        .iter()
        .enumerate()
        .fold(vec![vec![]; n], |mut acc, (i, &p)| {
            if p >= 0 {
                acc[p as usize].push(i);
            }
            acc
        });
    let mut masks = vec![0; n];
    dfs(&adj, s.as_bytes(), 0, 0, &mut masks);
    let mut res = 0;
    let mut seen = std::collections::HashMap::new();
    for &mask in masks[1..].iter() {
        res += i64::from(mask.count_ones() <= 1);
        for bit in 0..26 {
            let xor = mask ^ (1 << bit);
            res += seen.get(&xor).unwrap_or(&0);
        }
        let v = seen.entry(mask).or_insert(0);
        res += *v;
        *v += 1;
    }
    res
}

fn dfs(adj: &[Vec<usize>], s: &[u8], node: usize, mut mask: i32, masks: &mut [i32]) {
    if node > 0 {
        mask ^= 1 << (s[node] - b'a');
        masks[node] = mask;
    }
    for &next in adj[node].iter() {
        dfs(adj, s, next, mask, masks);
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
        assert_eq!(count_palindrome_paths(&[-1, 0, 0, 1, 1, 2], &"acaabc"), 8);
        assert_eq!(count_palindrome_paths(&[-1, 0, 0, 0, 0], &"aaaaa"), 10);
    }

    #[test]
    fn test() {}
}
