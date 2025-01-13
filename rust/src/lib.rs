mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn entity_parser(text: &str) -> String {
    let entities = std::collections::HashMap::from([
        ("&quot;", "\""),
        ("&apos;", "'"),
        ("&amp;", "&"),
        ("&gt;", ">"),
        ("&lt;", "<"),
        ("&frasl;", "/"),
    ]);
    let mut res = String::new();
    let mut idx = 0;
    while idx < text.len() {
        let mut flag = false;
        for (k, v) in entities.iter() {
            if text[idx..].starts_with(k) {
                res.push_str(v);
                idx += k.len();
                flag = true;
            }
        }
        if !flag && idx < text.len() {
            res.push_str(&text[idx..1 + idx]);
            idx += 1;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            entity_parser("&amp; is an HTML entity but &ambassador; is not.".into()),
            "& is an HTML entity but &ambassador; is not."
        );
        assert_eq!(
            entity_parser("and I quote: &quot;...&quot;".into()),
            "and I quote: \"...\""
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            entity_parser("&amp;quot;&amp;apos;&amp;amp;&amp;gt;&amp;lt;&amp;frasl;".into()),
            "&quot;&apos;&amp;&gt;&lt;&frasl;"
        );
        assert_eq!(entity_parser("&&gt;"), "&>");
        assert_eq!(
            entity_parser("x &gt; y &amp;&amp; x &lt; y is always false"),
            "x > y && x < y is always false"
        );
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
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
