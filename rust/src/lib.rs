mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

// Suppose s = "aba"
// idx => dp[1+idx]
// idx = 1, previous seqs are [], "a"
//          Without 'b' => [], "a"
//          With 'b' => "b", "ab"
// idx = 2, [], "a", "b", "ab"
//          "a", "aa", "ba", "aba"
//          Remove dupe "a" => Remove any seq ending with 'a', dp[index('a')]
pub fn distinct_subseq_ii(s: &str) -> i32 {
    const M: i32 = 1_000_000_007;
    let n = s.len();
    let mut prev = [None::<usize>; 26];
    let mut dp = vec![0_i32; 1 + n];
    dp[0] = 1; // empty seq
    for (idx, b) in s.bytes().enumerate() {
        let bi = usize::from(b - b'a');
        dp[1 + idx] = 2 * dp[idx] % M;
        if let Some(v) = prev[bi] {
            dp[1 + idx] = (dp[1 + idx] - dp[v]).rem_euclid(M);
        }
        prev[bi] = Some(idx);
    }
    (dp[n] - 1).rem_euclid(M)
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
