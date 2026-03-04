mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
    let [rows, cols] = get_dimensions(&mat);
    let [mut rsum, mut csum] = [vec![0; rows], vec![0; cols]];
    for (r, row) in mat.iter().enumerate() {
        for (c, &v) in row.iter().enumerate() {
            rsum[r] += v;
            csum[c] += v;
        }
    }
    mat.iter()
        .enumerate()
        .flat_map(|(r, row)| row.iter().enumerate().map(move |(c, &v)| (r, c, v)))
        .filter(|&(r, c, v)| rsum[r] == 1 && csum[c] == 1 && v == 1)
        .count() as i32
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
