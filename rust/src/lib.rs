mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_replace_string(
    mut s: String,
    indices: &[i32],
    sources: &[&str],
    targets: &[&str],
) -> String {
    let mut ists: Vec<_> = indices
        .iter()
        .zip(sources.iter())
        .zip(targets.iter())
        .map(|((i, src), tgt)| (*i as usize, *src, *tgt))
        .collect();
    ists.sort_unstable();
    let mut prev_idx = s.len();
    for (i, src, tgt) in ists.into_iter().rev() {
        if i == prev_idx {
            continue;
        }
        if s[i..].starts_with(src) {
            s.replace_range(i..i + src.len(), tgt);
            prev_idx = i;
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            find_replace_string("abcd".into(), &[0, 2], &["a", "cd"], &["eee", "ffff"]),
            "eeebffff"
        );
        debug_assert_eq!(
            find_replace_string("abcd".into(), &[0, 2], &["ab", "ec"], &["eee", "ffff"]),
            "eeecd"
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
}
