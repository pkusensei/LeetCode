mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn shortest_superstring(words: &[&str]) -> String {
    let n = words.len();
    let mut overlaps = vec![vec![0; n]; n];
    for (i1, a) in words.iter().enumerate() {
        for (i2, b) in words.iter().enumerate() {
            overlaps[i1][i2] = find_overlap(a, b);
            overlaps[i2][i1] = find_overlap(b, a);
        }
    }
    let [max_overlap, prev_bits] = dp(n, &overlaps);
    let seq = build(n, max_overlap, prev_bits);
    let mut res = words[seq[0]].to_string();
    for w in seq.windows(2) {
        let [a, b] = w[..] else { unreachable!() };
        let overlap = overlaps[a][b];
        res.push_str(&words[b][overlap..]);
    }
    res
}

fn build(n: usize, max_overlap: Vec<Vec<usize>>, prev_bits: Vec<Vec<usize>>) -> Vec<usize> {
    let full_mask = 1 << n;
    let mut build = vec![];
    let mut mask = full_mask - 1;
    let mut bit = (0..n).max_by_key(|&b| max_overlap[mask][b]).unwrap();
    while mask > 0 {
        build.push(bit);
        let prev_bit = prev_bits[mask][bit];
        mask ^= 1 << bit;
        bit = prev_bit;
    }
    build.reverse();
    build
}

fn dp(n: usize, overlaps: &[Vec<usize>]) -> [Vec<Vec<usize>>; 2] {
    let full_mask = 1 << n;
    // dp[mask][prev]
    // To reach mask, it lastly used with prev
    let mut max_overlap = vec![vec![0; n]; full_mask];
    // sentinel=n
    let mut prev_bits = vec![vec![n; n]; full_mask];
    'out: for mask in 1..full_mask {
        for bit in 0..n {
            // This bit is set in mask
            // This str is used in building
            if mask & (1 << bit) > 0 {
                // Single bit/str
                if mask == 1 << bit {
                    continue 'out;
                }
                let prev_mask = mask ^ (1 << bit);
                for prev_bit in 0..n {
                    if prev_mask & (1 << prev_bit) == 0 {
                        continue;
                    }
                    let curr_overlap = max_overlap[prev_mask][prev_bit] + overlaps[prev_bit][bit];
                    // >= handles non-overlap cases
                    if curr_overlap >= max_overlap[mask][bit] {
                        max_overlap[mask][bit] = curr_overlap;
                        prev_bits[mask][bit] = prev_bit;
                    }
                }
            }
        }
    }
    [max_overlap, prev_bits]
}

fn find_overlap(a: &str, b: &str) -> usize {
    let [a, b] = [a, b].map(|v| v.as_bytes());
    let mut res = a.len().min(b.len());
    while res > 0 {
        if a.ends_with(&b[..res]) {
            break;
        }
        res -= 1;
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
    fn basics() {
        assert_eq!(
            shortest_superstring(&["alex", "loves", "leetcode"]),
            "alexlovesleetcode"
        );
    }

    #[test]
    fn test() {}
}
