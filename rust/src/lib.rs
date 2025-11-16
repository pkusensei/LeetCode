mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_distinct(n: i64) -> i64 {
    let s = n.to_string().into_bytes();
    let n = s.len();
    let mut memo = vec![[[-1; 2]; 2]; 1 + n];
    dfs(&s, 1, 1, &mut memo)
}

fn dfs(s: &[u8], tight: usize, leading_zero: usize, memo: &mut [[[i64; 2]; 2]]) -> i64 {
    if s.is_empty() {
        return i64::from(leading_zero == 0); // not zero
    }
    if memo[s.len()][tight][leading_zero] > -1 {
        return memo[s.len()][tight][leading_zero];
    }
    let upper = if tight == 1 { s[0] } else { b'9' };
    let lower = if leading_zero == 1 { b'0' } else { b'1' };
    let mut res = 0;
    for b in lower..=upper {
        let ntight = tight & usize::from(b == upper);
        let nleading = leading_zero & usize::from(b == b'0');
        res += dfs(&s[1..], ntight, nleading, memo);
    }
    memo[s.len()][tight][leading_zero] = res;
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
        assert_eq!(count_distinct(10), 9);
        assert_eq!(count_distinct(3), 3);
    }

    #[test]
    fn test() {}
}
