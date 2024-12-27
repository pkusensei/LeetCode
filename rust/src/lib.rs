mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn is_solvable(words: &[&str], result: &str) -> bool {
    let (coeffs, nonzeros) = build(words, result);
    backtrack(&coeffs, &nonzeros, &mut [false; 10], &mut [0; 10], 0, 0)
}

fn backtrack(
    coeffs: &[i32],
    nonzeros: &[bool],
    used: &mut [bool; 10],
    values: &mut [i32; 10], // at most 10 unique chars
    idx: usize,
    curr: i32,
) -> bool {
    if idx == coeffs.len() {
        return curr == 0;
    }
    // Pruning on potential result
    let max: i32 = coeffs[idx..]
        .iter()
        .map(|&v| if v > 0 { 9 * v } else { 0 })
        .sum();
    let min: i32 = coeffs[idx..]
        .iter()
        .map(|&v| if v > 0 { 0 } else { 9 * v })
        .sum();
    // sum cannot ever reach 0 by current config
    if curr + max < 0 || curr + min > 0 {
        return false;
    }
    let temp = values[idx];
    let start = if nonzeros[idx] { 1 } else { 0 };
    for digit in start..=9 {
        if !used[digit as usize] {
            used[digit as usize] = true;
            values[idx] = digit;
            let next = curr + digit * coeffs[idx];
            if backtrack(coeffs, nonzeros, used, values, 1 + idx, next) {
                return true;
            }
            used[digit as usize] = false;
        }
    }
    values[idx] = temp;
    false
}

fn build(words: &[&str], result: &str) -> (Vec<i32>, Vec<bool>) {
    let mut coeffs = [(0, false); 26];
    for word in words.iter() {
        process(word, 1, &mut coeffs);
    }
    process(result, -1, &mut coeffs);
    // coeffs.sort_by_key(|(coeff, _)| std::cmp::Reverse(coeff.abs()));
    coeffs.into_iter().filter(|(coeff, _)| *coeff != 0).unzip()
}

fn process(word: &str, sign: i32, coeffs: &mut [(i32, bool); 26]) {
    let mut pow = 1;
    for (idx, b) in word.bytes().enumerate().rev() {
        let ch = usize::from(b - b'A');
        coeffs[ch].0 += sign * pow;
        pow *= 10;
        // leading chars can't be zero
        if word.len() > 1 && idx == 0 {
            coeffs[ch].1 = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert!(is_solvable(&["SEND", "MORE"], "MONEY"));
        assert!(is_solvable(&["SIX", "SEVEN", "SEVEN"], "TWENTY"));
        assert!(!is_solvable(&["LEET", "CODE"], "POINT"));
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
