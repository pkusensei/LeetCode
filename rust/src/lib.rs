mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn shortest_common_supersequence(str1: &str, str2: &str) -> String {
    let [(s1, n1), (s2, n2)] = [&str1, &str2].map(|s| (s.as_bytes(), s.len()));
    let mut dp = vec![vec![0; 1 + n2]; 1 + n1];
    for (i1, b1) in s1.iter().enumerate() {
        for (i2, b2) in s2.iter().enumerate() {
            dp[1 + i1][1 + i2] = if b1 == b2 {
                1 + dp[i1][i2]
            } else {
                dp[1 + i1][i2].max(dp[i1][1 + i2])
            };
        }
    }
    let mut i1 = n1;
    let mut i2 = n2;
    let mut arr = vec![];
    while i1 > 0 && i2 > 0 {
        if s1[i1 - 1] == s2[i2 - 1] {
            arr.push(s1[i1 - 1]);
            i1 -= 1;
            i2 -= 1;
        } else if dp[i1 - 1][i2] > dp[i1][i2 - 1] {
            arr.push(s1[i1 - 1]);
            i1 -= 1
        } else {
            arr.push(s2[i2 - 1]);
            i2 -= 1
        }
    }
    arr.extend(s1[..i1].iter().rev());
    arr.extend(s2[..i2].iter().rev());
    arr.reverse();
    String::from_utf8(arr).unwrap()
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
