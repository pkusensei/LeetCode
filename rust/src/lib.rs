mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_operations(queries: Vec<Vec<i32>>) -> i64 {
    const P: [i64; 16] = {
        let mut p = [1; 16];
        let mut i = 1;
        while i < 16 {
            p[i] = 4 * p[i - 1];
            i += 1;
        }
        p
    };
    let mut res = 0;
    for q in &queries {
        let [left, right] = [0, 1].map(|i| i64::from(q[i]));
        let mut sum = 0;
        for pow in 1..16 {
            let a = left.max(P[pow - 1]);
            let b = right.min(P[pow] - 1);
            if a <= b {
                sum += pow as i64 * (b - a + 1);
            }
        }
        res += (1 + sum) / 2;
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
