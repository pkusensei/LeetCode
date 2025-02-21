mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn longest_subsequence_repeated_k(s: &str, k: i32) -> String {
    let chs: Vec<_> = s
        .bytes()
        .fold([0; 26], |mut acc, b| {
            acc[usize::from(b - b'a')] += 1;
            acc
        })
        .into_iter()
        .enumerate()
        .rev()
        .flat_map(|(i, count)| std::iter::repeat(b'a' + i as u8).take((count / k) as usize))
        .collect();
    let mut res = vec![];
    backtrack(s.as_bytes(), &chs, k, &mut vec![], &mut res, 0);
    String::from_utf8(res).unwrap()
}

fn backtrack(s: &[u8], chs: &[u8], k: i32, curr: &mut Vec<u8>, res: &mut Vec<u8>, mask: i32) {
    for (i, &b) in chs.iter().enumerate() {
        if (mask >> i) & 1 == 1 {
            continue;
        }
        curr.push(b);
        if find(s, curr, k) {
            if curr.len() > res.len() {
                *res = curr.clone();
            }
            backtrack(s, chs, k, curr, res, mask | (1 << i));
        }
        curr.pop();
    }
}

fn find(s: &[u8], curr: &[u8], mut k: i32) -> bool {
    let [mut i1, mut i2] = [0, 0];
    while k > 0 && i1 < s.len() && i2 < curr.len() {
        if s[i1] == curr[i2] {
            i1 += 1;
            i2 += 1;
            if i2 == curr.len() {
                k -= 1;
                i2 = 0;
            }
        } else {
            i1 += 1;
        }
    }
    k <= 0
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
        assert_eq!(longest_subsequence_repeated_k("letsleetcode", 2), "let");
        assert_eq!(longest_subsequence_repeated_k("bb", 2), "b");
        assert_eq!(longest_subsequence_repeated_k("ab", 2), "");
    }

    #[test]
    fn test() {
        assert_eq!(longest_subsequence_repeated_k("rehrklqrhrklqrhrklqrhrklqrhrklqrhrklqrhrklqrhrklqrhrklqrhrkloqnrhrklqrhrkilyqrhrklqrphrkliqrherkglqrhzraklqrhrklqrhrlklqrhrklqrxhrklqrhrklkqhrhzorklrq", 22),"rhrklq");
    }
}
