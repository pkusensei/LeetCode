mod dsu;
mod helper;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn brace_expansion_ii(expression: &str) -> Vec<String> {
        let mut res: Vec<_> = parse(expression).into_iter().collect();
        res.sort_unstable();
        res
}

fn parse(s: &str) -> HashSet<String> {
    let mut groups = vec![vec![]];
    let (mut open, mut left) = (0, 0);
    for (right, ch) in s.char_indices() {
        match ch {
            '{' => {
                if open == 0 {
                    left = 1 + right;
                }
                open += 1;
            }
            '}' => {
                open -= 1;
                if open == 0 {
                    if let Some(v) = groups.last_mut() {
                        v.push(parse(&s[left..right]));
                    }
                }
            }
            ',' if open == 0 => groups.push(vec![]),
            _ if open == 0 => {
                if let Some(v) = groups.last_mut() {
                    v.push(HashSet::from([ch.to_string()]));
                }
            }
            _ => (),
        }
    }
    let mut set = HashSet::new();
    for group in groups.iter() {
        let mut prev = vec!["".to_string()];
        for words in group.iter() {
            let mut temp = vec![];
            for p in prev.iter() {
                temp.extend(words.iter().map(|w| format!("{p}{w}")));
            }
            prev = temp;
        }
        set.extend(prev);
    }
    set
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            brace_expansion_ii("{a,b}{c,{d,e}}"),
            ["ac", "ad", "ae", "bc", "bd", "be"]
        );
        debug_assert_eq!(
            brace_expansion_ii("{{a,z},a{b,c},{ab,z}}"),
            ["a", "ab", "ac", "z"]
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
        if (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() > EP {
            dbg!(a, b);
        }
    }
}
