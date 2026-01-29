mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;
use itertools::izip;

/// Computes shortest paths between all pairs of nodes using Floyd-Warshall.
/// Returns matrix where `result[i][j]` is the minimum cost from node i to j,
/// or None if no path exists.
pub fn fw(n: usize, src: &[usize], dst: &[usize], cost: &[i32]) -> Vec<Vec<Option<i32>>> {
    assert_eq!(
        src.len(),
        dst.len(),
        "'src' and 'dst' must be equal in length"
    );
    assert_eq!(
        src.len(),
        cost.len(),
        "'src' and 'cost' must be equal in length"
    );
    let mut res = vec![vec![None; n]; n];
    for i in 0..n {
        res[i][i] = Some(0);
    }
    for (&a, &b, &c) in izip!(src, dst, cost) {
        let v = res[a][b].get_or_insert(c);
        *v = (*v).min(c); // handle dups
    }
    // With mid being the outmost loop
    // We start by trying only 0 as intermediate
    // With each following loop we incoporate 1,2,..,(n-1) as intermediates
    // Thus we exhaust all possible paths between any pair of nodes
    for mid in 0..n {
        for a in 0..n {
            for b in 0..n {
                if let Some((x, y)) = res[a][mid].zip(res[mid][b]) {
                    let v = res[a][b].get_or_insert(x + y);
                    *v = (*v).min(x + y);
                }
            }
        }
    }
    res
}

pub fn minimum_cost(
    source: &str,
    target: &str,
    original: &[u8],
    changed: &[u8],
    cost: &[i32],
) -> i64 {
    use itertools::izip;
    let mut mat = [[None; 26]; 26];
    for i in 0..26 {
        mat[i][i] = Some(0);
    }
    for (a, b, c) in izip!(original.iter(), changed.iter(), cost.iter()) {
        let [a, b] = [a, b].map(|&v| usize::from(v as u8 - b'a'));
        let v = mat[a][b].get_or_insert(i64::from(*c));
        *v = (*v).min(i64::from(*c));
    }
    for mid in 0..26 {
        for a in 0..26 {
            for b in 0..26 {
                if let Some((x, y)) = mat[a][mid].zip(mat[mid][b]) {
                    let v = mat[a][b].get_or_insert(x + y);
                    *v = (*v).min(x + y);
                }
            }
        }
    }
    let mut res = 0;
    for (a, b) in izip!(source.bytes(), target.bytes()) {
        let [a, b] = [a, b].map(|v| usize::from(v - b'a'));
        if let Some(v) = mat[a][b] {
            res += v;
        } else {
            return -1;
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
