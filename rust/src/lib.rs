mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_cost(
    source: String,
    target: String,
    original: Vec<char>,
    changed: Vec<char>,
    cost: Vec<i32>,
) -> i64 {
    use itertools::izip;
    let mut mat = [[i64::MAX; 26]; 26];
    for (a, b, c) in izip!(original, changed, cost) {
        let [i1, i2] = [a, b].map(|v| usize::from(v as u8 - b'a'));
        mat[i1][i2] = mat[i1][i2].min(i64::from(c));
    }
    for mid in 0..26 {
        for a in 0..26 {
            for b in 0..26 {
                if mat[a][mid] < i64::MAX && mat[mid][b] < i64::MAX {
                    mat[a][b] = mat[a][b].min(mat[a][mid] + mat[mid][b]);
                }
            }
        }
    }
    let mut res = 0;
    for (a, b) in izip!(source.bytes(), target.bytes()).filter(|(a, b)| a != b) {
        let [i1, i2] = [a, b].map(|v| usize::from(v - b'a'));
        if mat[i1][i2] < i64::MAX {
            res += mat[i1][i2]
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
    fn basics() {}

    #[test]
    fn test() {}
}
