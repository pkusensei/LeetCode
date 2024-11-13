mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn spellchecker(wordlist: &[&str], queries: &[&str]) -> Vec<String> {
    const VOWELS: &[u8] = b"aeiouAEIOU";
    let mut res = Vec::with_capacity(queries.len());
    for q in queries.iter() {
        let [mut exact, mut capit, mut vowel] = [None; 3];
        for w in wordlist.iter() {
            if q == w {
                exact = Some(w);
                break;
            } else {
                let mut diff = q.bytes().zip(w.bytes()).filter(|(a, b)| a != b);
                if capit.is_none() && diff.clone().all(|(a, b)| a.abs_diff(b) == 32) {
                    capit = Some(w);
                } else if vowel.is_none()
                    && diff.all(|(a, b)| {
                        a.abs_diff(b) == 32 || VOWELS.contains(&a) && VOWELS.contains(&b)
                    })
                {
                    vowel = Some(w);
                }
            }
        }
        res.push(
            exact
                .or(capit)
                .or(vowel)
                .map(|s| s.to_string())
                .unwrap_or_default(),
        );
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            spellchecker(
                &["KiTe", "kite", "hare", "Hare"],
                &["kite", "Kite", "KiTe", "Hare", "HARE", "Hear", "hear", "keti", "keet", "keto"]
            ),
            ["kite", "KiTe", "KiTe", "Hare", "hare", "", "", "KiTe", "", "KiTe"]
        );
        debug_assert_eq!(spellchecker(&["yellow"], &["YellOw"]), ["yellow"]);
    }

    #[test]
    fn test() {
        debug_assert_eq!(spellchecker(&["zeo", "Zuo"], &["zuo"]), &["Zuo"]);
    }

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
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
