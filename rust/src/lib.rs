mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
    let [m, n] = [m, n].map(|v| v as usize);
    let mut dp = vec![vec![0; 1 + n]; 1 + m];
    for s in strs.iter() {
        let zeros = s.bytes().filter(|&b| b == b'0').count();
        let ones = s.len() - zeros;
        for i0 in (zeros..=m).rev() {
            for i1 in (ones..=n).rev() {
                dp[i0][i1] = dp[i0][i1].max(1 + dp[i0 - zeros][i1 - ones]);
            }
        }
    }
    dp[m][n]
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
