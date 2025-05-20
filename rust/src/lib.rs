mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_starting_index(s: &str, pattern: &str) -> i32 {
    let n1 = s.len();
    let n2 = pattern.len();
    let z1 = z_function(format!("{}{}", pattern, s).as_bytes());
    let mut combined = format!("{}{}", s, pattern).into_bytes();
    combined.reverse();
    let z2 = z_function(&combined);
    for i in 0..=(n1 - n2) {
        if z1[i + n2] + 1 + z2[n1 - i] >= n2 {
            return i as i32;
        }
    }
    -1
}

fn z_function(s: &[u8]) -> Vec<usize> {
    let n = s.len();
    let [mut left, mut right] = [0, 0];
    let mut z = vec![0; n];
    for i in 1..n {
        if i <= right {
            z[i] = (right + 1 - i).min(z[i - left]);
        }
        while i + z[i] < n && s[z[i]] == s[i + z[i]] {
            z[i] += 1;
        }
        if i + z[i] - 1 > right {
            left = i;
            right = i + z[i] - 1;
        }
    }
    z
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
    fn basics() {
        assert_eq!(min_starting_index("abcdefg", "bcdffg"), 1);
        assert_eq!(min_starting_index("ababbababa", "bacaba"), 4);
        assert_eq!(min_starting_index("abcd", "dba"), -1);
        assert_eq!(min_starting_index("dde", "d"), 0);
    }

    #[test]
    fn test() {}
}
