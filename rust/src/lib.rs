mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_palindromes_after_operations(words: &[&str]) -> i32 {
    let mut lens = vec![];
    let mut freq = [0; 26];
    for s in words.iter() {
        lens.push(s.len() as i32);
        for b in s.bytes() {
            freq[usize::from(b - b'a')] += 1;
        }
    }
    let [mut even_char, mut odd_char] = [0, 0];
    for v in freq {
        even_char += v / 2 * 2;
        odd_char += v & 1;
    }
    lens.sort_unstable();
    let mut res = 0;
    let mut deficit = 0;
    for val in lens {
        even_char -= val / 2 * 2;
        if even_char >= 0 {
            res += i32::from(val & 1 == 0);
            deficit += val & 1;
        } else {
            break;
        }
    }
    res + deficit.min(odd_char + even_char.max(0))
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
        assert_eq!(max_palindromes_after_operations(&["cd", "ef", "a"]), 1);
        assert_eq!(max_palindromes_after_operations(&["abbb", "ba", "aa"]), 3);
        assert_eq!(max_palindromes_after_operations(&["abc", "ab"]), 2);
    }

    #[test]
    fn test() {}
}
