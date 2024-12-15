mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_score_words(words: &[&str], letters: &[char], score: &[i32]) -> i32 {
    let mut letters = letters.iter().fold([0; 26], |mut acc, &c| {
        acc[usize::from(c as u8 - b'a')] += 1;
        acc
    });
    let mut res = 0;
    dfs(words, &mut letters, score, 0, &mut res);
    res
}

fn dfs(words: &[&str], letters: &mut [i32; 26], score: &[i32], curr: i32, res: &mut i32) {
    match words {
        [] => (*res) = (*res).max(curr),
        [head, tail @ ..] => {
            dfs(tail, letters, score, curr, res);
            let mut temp = 0;
            for b in head.bytes() {
                let i = usize::from(b - b'a');
                temp += score[i];
                letters[i] -= 1;
            }
            if letters.iter().all(|&v| v >= 0) {
                dfs(tail, letters, score, curr + temp, res);
            }
            for b in head.bytes() {
                let i = usize::from(b - b'a');
                letters[i] += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            max_score_words(
                &["dog", "cat", "dad", "good"],
                &['a', 'a', 'c', 'd', 'd', 'd', 'g', 'o', 'o'],
                &[1, 0, 9, 5, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            ),
            23
        );
        assert_eq!(
            max_score_words(
                &["xxxz", "ax", "bx", "cx"],
                &['z', 'a', 'b', 'c', 'x', 'x', 'x'],
                &[4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 10]
            ),
            27
        );
        assert_eq!(
            max_score_words(
                &["leetcode"],
                &['l', 'e', 't', 'c', 'o', 'd'],
                &[0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0]
            ),
            0
        );
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
            "left = {a:?}, right = {b:?}",
        );
    }
}
