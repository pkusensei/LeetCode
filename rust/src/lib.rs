mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn shortest_superstring(words: &[&str]) -> String {
    let n = words.len();
    let mut overlap = vec![vec![0; n]; n];
    for (i1, a) in words.iter().enumerate() {
        for (i2, b) in words.iter().enumerate().skip(1 + i1) {
            overlap[i1][i2] = get_overlap(a, b);
            overlap[i2][i1] = get_overlap(b, a);
        }
    }
    let (dp, parent) = enumerate(n, &overlap);
    let perm = build(n, dp, parent);
    let mut res = words[perm[0]].to_string();
    for w in perm.windows(2) {
        let over = overlap[w[0]][w[1]];
        res.push_str(&words[w[1]][over..]);
    }
    res
}

fn build(n: usize, dp: Vec<Vec<usize>>, parent: Vec<Vec<Option<usize>>>) -> Vec<usize> {
    let mut perm = Vec::with_capacity(n);
    let mut mask = (1 << n) - 1;
    // Find last added str
    let mut p = (0..n).max_by_key(|&i| dp[mask][i]);
    let mut seen = vec![false; n];
    while let Some(v) = p {
        perm.push(v);
        seen[v] = true;
        let p2 = parent[mask][v];
        mask ^= 1 << v;
        p = p2;
    }
    perm.reverse();
    // add strs with no overlaps
    perm.extend((0..n).filter(|&i| !seen[i]));
    perm
}

fn enumerate(n: usize, overlap: &[Vec<usize>]) -> (Vec<Vec<usize>>, Vec<Vec<Option<usize>>>) {
    // dp[mask][i] => mask which ends with i
    let mut dp = vec![vec![0; n]; 1 << n];
    // parent[mask][i] => previous i which leads to mask with i
    let mut parent = vec![vec![None; n]; 1 << n];
    for mask in 0..1 << n {
        // try this end bit
        for bit in 0..n {
            if (mask >> bit) & 1 == 0 {
                continue;
            }
            let pmask = mask ^ (1 << bit);
            if pmask == 0 {
                continue; // avoid parent->parent loop
            }
            for pi in 0..n {
                if (pmask >> pi) & 1 == 0 {
                    continue;
                }
                // For each possible parent, find its overlap
                let val = dp[pmask][pi] + overlap[pi][bit];
                if val > dp[mask][bit] {
                    dp[mask][bit] = val;
                    parent[mask][bit] = Some(pi);
                }
            }
        }
    }
    (dp, parent)
}

fn get_overlap(a: &str, b: &str) -> usize {
    let mut n = a.len().min(b.len());
    while n > 0 {
        if a.ends_with(&b[..n]) {
            return n;
        }
        n -= 1;
    }
    0
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            shortest_superstring(&["alex", "loves", "leetcode"]),
            "leetcodealexloves"
        );
        debug_assert_eq!(
            shortest_superstring(&["catg", "ctaagt", "gcta", "ttca", "atgcatc"]),
            "gctaagttcatgcatc"
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

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64>,
        T2: Into<f64>,
    {
        const EP: f64 = 1e-5;
        debug_assert!((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP);
    }
}
