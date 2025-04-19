mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_complete_substrings(word: &str, k: i32) -> i32 {
    let mut res = 0;
    for ch in word.as_bytes().chunk_by(|&a, &b| a.abs_diff(b) <= 2) {
        res += substr(ch, k);
    }
    res
}

fn substr(s: &[u8], k: i32) -> i32 {
    let n = s.len();
    let k = k as usize;
    let mut res = 0;
    for len in (k..=(26 * k).min(n)).step_by(k) {
        let mut freq = s[..len].iter().fold([0; 26], |mut acc, &b| {
            acc[usize::from(b - b'a')] += 1;
            acc
        });
        res += i32::from(freq.iter().filter(|&&f| f == k).count() == len / k);
        for idx in len..n {
            let add = usize::from(s[idx] - b'a');
            freq[add] += 1;
            let del = usize::from(s[idx - len] - b'a');
            freq[del] -= 1;
            res += i32::from(freq.iter().filter(|&&f| f == k).count() == len / k);
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
        assert_eq!(count_complete_substrings("igigee", 2), 3);
        assert_eq!(count_complete_substrings("aaabbbccc", 3), 6);
    }

    #[test]
    fn test() {}
}
