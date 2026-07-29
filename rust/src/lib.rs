mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn smallest_palindrome(s: String, k: i32) -> String {
    let n = s.len();
    let mut k = i64::from(k);
    let mut freq = s.bytes().take(n / 2).fold([0; 26], |mut acc, b| {
        acc[usize::from(b - b'a')] += 1;
        acc
    });
    let mut res = Vec::with_capacity(n);
    for idx in 0..n / 2 {
        for cand in 0..26 {
            if freq[cand] == 0 {
                continue;
            }
            freq[cand] -= 1;
            let ways = count_perm(&freq, (n / 2 - idx - 1) as i64, k);
            if ways < k {
                freq[cand] += 1;
                k -= ways;
            } else {
                res.push(cand as u8 + b'a');
                break;
            }
        }
    }
    if res.len() < n / 2 {
        return "".to_string();
    }
    if n & 1 == 1 {
        res.push(s.as_bytes()[n / 2]);
    }
    res.extend_from_within(..n / 2);
    res[(1 + n) / 2..].reverse();
    String::from_utf8(res).unwrap()
}

fn count_perm(freq: &[i64; 26], mut rem: i64, max: i64) -> i64 {
    let mut res = 1;
    for &f in freq {
        if f == 0 {
            continue;
        }
        res *= n_choose_k(rem, f, max);
        if res > max {
            break;
        }
        rem -= f;
    }
    res
}

fn n_choose_k(n: i64, k: i64, max: i64) -> i64 {
    let k = k.min(n - k);
    let mut res = 1;
    for i in 1..=k {
        res = res * (n - i + 1) / i;
        if res > max {
            return 1 + max;
        }
    }
    res
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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
