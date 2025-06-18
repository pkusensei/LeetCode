mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn smallest_palindrome(s: &str, k: i32) -> String {
    use itertools::Itertools;
    let n = s.len();
    let mut k = i64::from(k);
    let freq = s.bytes().fold([0; 26], |mut acc, b| {
        acc[usize::from(b - b'a')] += 1;
        acc
    });
    let mut res = Vec::with_capacity((1 + n) / 2);
    let mut fhalf = freq.map(|f| f / 2);
    while res.len() < n / 2 {
        let mut pushed = false;
        for cand in 0..26 {
            if fhalf[cand] > 0 {
                fhalf[cand] -= 1;
                let v = next_perm(&fhalf);
                if v < k {
                    k -= v;
                    fhalf[cand] += 1;
                } else {
                    res.push(cand as u8 + b'a');
                    pushed = true;
                    break;
                }
            }
        }
        if !pushed {
            return "".into();
        }
    }
    let rev = res.iter().copied().rev().collect_vec();
    for (idx, f) in freq.iter().enumerate() {
        if f & 1 == 1 {
            res.push(idx as u8 + b'a');
        }
    }
    res.extend(rev);
    String::from_utf8(res).unwrap()
}

fn next_perm(fhalf: &[i32; 26]) -> i64 {
    let mut sum = 0;
    let mut denoms = vec![];
    for &f in fhalf {
        if f > 0 {
            let f = i64::from(f);
            sum += f;
            denoms.extend(2..=f);
        }
    }
    let mut res = 1;
    let mut idx = 0;
    for i in 2..=sum {
        res *= i;
        while denoms.get(idx).is_some_and(|d| res % d == 0) {
            res /= denoms[idx];
            idx += 1;
        }
        if res > 1_000_000 {
            return res;
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
        assert_eq!(smallest_palindrome("aa", 2), "");
        assert_eq!(smallest_palindrome("abba", 2), "baab");
        assert_eq!(smallest_palindrome("bacab", 1), "abcba");
    }

    #[test]
    fn test() {
        assert_eq!(smallest_palindrome("gnllllng", 6), "llgnngll");
    }
}
