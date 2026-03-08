mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_cost(s: &str, enc_cost: i32, flat_cost: i32) -> i64 {
    let n = s.len();
    let [enc_cost, flat_cost] = [enc_cost, flat_cost].map(i64::from);
    let pref_x = s.bytes().fold(Vec::with_capacity(n), |mut acc, b| {
        acc.push(usize::from(b == b'1') + acc.last().unwrap_or(&0));
        acc
    });
    dfs(&pref_x, enc_cost, flat_cost, 0, n - 1)
}

fn dfs(pref_x: &[usize], enc_cost: i64, flat_cost: i64, left: usize, right: usize) -> i64 {
    let x_count = pref_x[right] - if left > 0 { pref_x[left - 1] } else { 0 };
    if x_count == 0 {
        return flat_cost;
    }
    let len = 1 + right - left;
    let mut res = len as i64 * x_count as i64 * enc_cost;
    if len & 1 == 1 {
        return res;
    }
    let mid = left.midpoint(right);
    res = res.min(
        dfs(pref_x, enc_cost, flat_cost, left, mid)
            + dfs(pref_x, enc_cost, flat_cost, 1 + mid, right),
    );
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
        assert_eq!(min_cost("1010", 2, 1), 6);
        assert_eq!(min_cost("1010", 3, 10), 12);
        assert_eq!(min_cost("00", 1, 2), 2);
    }

    #[test]
    fn test() {}
}
