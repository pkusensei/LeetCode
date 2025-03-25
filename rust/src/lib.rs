mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn is_it_possible(word1: &str, word2: &str) -> bool {
    fn count(nums: &[i32; 26]) -> usize {
        nums.iter().filter(|&&v| v > 0).count()
    }

    let [mut freq1, mut freq2] = [&word1, &word2].map(|s| {
        s.bytes().fold([0; 26], |mut acc, b| {
            acc[usize::from(b - b'a')] += 1;
            acc
        })
    });
    for a in 0..26 {
        if freq1[a] == 0 {
            continue;
        }
        for b in 0..26 {
            if freq2[b] == 0 {
                continue;
            }
            freq1[a] -= 1;
            freq2[a] += 1;
            freq2[b] -= 1;
            freq1[b] += 1;
            if count(&freq1) == count(&freq2) {
                return true;
            }
            freq1[a] += 1;
            freq2[a] -= 1;
            freq2[b] += 1;
            freq1[b] -= 1;
        }
    }
    false
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
        assert!(!is_it_possible("ac", "b"));
        assert!(is_it_possible("abcc", "aab"));
        assert!(is_it_possible("abcde", "fghij"));
    }

    #[test]
    fn test() {}
}
